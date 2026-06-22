#!/usr/bin/env python3
"""
Generate a Rust struct model from an OpenAPI schema. Output to stdout only.

Usage:
  gen_resource_model.py <spec.json> "#/components/schemas/Foo"

The LLM decides whether to write a new file or apply targeted edits.

Rules applied:
  - rename_all = "camelCase" always
  - id: integer -> Option<i32> with skip_serializing_if = "Option::is_none"
  - nullable: true -> Option<T>
  - readOnly: true -> skip_serializing (also applied to known read-only field names)
  - bool fields with default: false -> #[serde(default)]
  - bool fields with default: true -> #[serde(default = "default_true")]
  - non-bool defaults -> #[serde(default = "default_<fieldname>")] + TODO comment
  - snake_case aliases for all multi-word fields
  - TODO comments for: FK integers, unknown defaults, ambiguous types
"""
import json
import re
import sys


KNOWN_READONLY_NAMES = {
    "implementationName", "infoLink", "supportsRss", "supportsSearch",
    "message", "presets", "schemas", "fields",  # fields is read-write actually
}
# fields that look like FKs but spec shows as integer
KNOWN_FK_NAMES = {"tags", "downloadClientId", "indexerId"}


def load_spec(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def resolve_ref(ref: str, spec: dict) -> dict:
    parts = ref.lstrip("#/").split("/")
    node = spec
    for p in parts:
        node = node[p]
    return node


def camel_to_snake(name: str) -> str:
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z\d])([A-Z])", r"\1_\2", s)
    return s.lower()


def is_multiword(snake: str) -> bool:
    return "_" in snake


def rust_type(field_name: str, prop: dict, spec: dict) -> tuple[str, list[str]]:
    """Returns (rust_type_str, [todo_comments])."""
    todos = []
    nullable = prop.get("nullable", False)

    if field_name == "id":
        return "Option<i32>", todos

    if "$ref" in prop:
        ref = prop["$ref"]
        name = ref.split("/")[-1]
        resolved = resolve_ref(ref, spec)
        if "enum" in resolved:
            t = name
        else:
            t = name
        return (f"Option<{t}>" if nullable else t, todos)

    t = prop.get("type", "")

    if t == "array":
        items = prop.get("items", {})
        if "$ref" in items:
            inner_name = items["$ref"].split("/")[-1]
            inner_resolved = resolve_ref(items["$ref"], spec)
            if "enum" in inner_resolved:
                return f"Vec<{inner_name}>", todos
            todos.append(f"// TODO: Vec<{inner_name}> or Vec<ResourceRef<{inner_name}>>?")
            return f"Vec<{inner_name}>", todos
        inner_t = {"string": "String", "integer": "i32", "boolean": "bool"}.get(
            items.get("type", "?"), "?"
        )
        if inner_t == "i32" and field_name in ("tags",):
            todos.append("// TODO: Vec<ResourceRef<Tag>>?")
        return f"Vec<{inner_t}>", todos

    mapping = {
        "string": "String",
        "integer": "i32",
        "boolean": "bool",
        "number": "f64",
    }
    rust = mapping.get(t, f"/* TODO: unknown type '{t}' */ serde_json::Value")

    if t == "integer" and field_name != "id" and field_name not in ("port", "sslPort", "proxyPort", "order", "priority"):
        if field_name.endswith("Id") or field_name in KNOWN_FK_NAMES:
            todos.append(f"// TODO: ResourceRef<T>?")

    if nullable:
        return f"Option<{rust}>", todos
    return rust, todos


def field_attrs(field_name: str, prop: dict, spec: dict, rust_t: str) -> list[str]:
    """Return list of #[serde(...)] attribute strings."""
    attrs = []
    snake = camel_to_snake(field_name)
    read_only = prop.get("readOnly", False) or field_name in KNOWN_READONLY_NAMES
    nullable = prop.get("nullable", False)
    default = prop.get("default")
    prop_type = prop.get("type", "")

    serde_parts = []

    if field_name == "id":
        serde_parts.append('skip_serializing_if = "Option::is_none"')
        return [f"#[serde({', '.join(serde_parts)})]"]

    if is_multiword(snake) and not read_only:
        serde_parts.append(f'alias = "{snake}"')

    if read_only:
        if is_multiword(snake):
            serde_parts.append(f'alias = "{snake}"')
        serde_parts.append("skip_serializing")

    # Defaults
    if not nullable and "Vec<" not in rust_t and not read_only:
        if prop_type == "boolean":
            if default is True:
                serde_parts.append('default = "default_true"')
            else:
                serde_parts.append("default")
        elif default is not None and prop_type in ("integer", "string", "number"):
            fn_name = f"default_{snake}"
            serde_parts.append(f'default = "{fn_name}"')

    if serde_parts:
        attrs.append(f"#[serde({', '.join(serde_parts)})]")
    return attrs


def collect_default_fns(schema: dict, spec: dict) -> list[str]:
    """Generate default_* fn bodies for non-bool defaults."""
    fns = []
    seen_true = False
    for field_name, prop in schema.get("properties", {}).items():
        snake = camel_to_snake(field_name)
        prop_type = prop.get("type", "")
        default = prop.get("default")
        read_only = prop.get("readOnly", False) or field_name in KNOWN_READONLY_NAMES
        nullable = prop.get("nullable", False)

        if read_only or nullable or default is None:
            continue

        if prop_type == "boolean":
            if default is True and not seen_true:
                fns.append("fn default_true() -> bool { true }")
                seen_true = True
        elif prop_type == "integer":
            fn_name = f"default_{snake}"
            fns.append(f"fn {fn_name}() -> i32 {{ {default} }}")
        elif prop_type == "string":
            fn_name = f"default_{snake}"
            fns.append(f'fn {fn_name}() -> String {{ "{default}".to_string() }}')
        elif prop_type == "number":
            fn_name = f"default_{snake}"
            fns.append(f"fn {fn_name}() -> f64 {{ {default} }}")

    return fns


def gen_struct(schema_name: str, schema: dict, spec: dict) -> str:
    lines = []
    lines.append("use serde::{Deserialize, Serialize};")
    lines.append("")

    props = schema.get("properties", {})
    all_todos = []

    lines.append("#[derive(Debug, Clone, Serialize, Deserialize)]")
    lines.append('#[serde(rename_all = "camelCase")]')
    lines.append(f"pub struct {schema_name} {{")

    for field_name, prop in props.items():
        snake = camel_to_snake(field_name)
        rust_t, todos = rust_type(field_name, prop, spec)
        attrs = field_attrs(field_name, prop, spec, rust_t)

        for t in todos:
            lines.append(f"    {t}")
        for a in attrs:
            lines.append(f"    {a}")
        lines.append(f"    pub {snake}: {rust_t},")

    lines.append("}")

    default_fns = collect_default_fns(schema, spec)
    if default_fns:
        lines.append("")
        for fn_line in default_fns:
            lines.append(fn_line)

    return "\n".join(lines)


def main():
    args = sys.argv[1:]
    if len(args) < 2 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path, schema_ref = args[0], args[1]
    spec = load_spec(spec_path)
    schemas = spec.get("components", {}).get("schemas", {})

    schema_name = schema_ref.split("/")[-1]
    schema = schemas.get(schema_name)
    if schema is None:
        print(f"ERROR: '{schema_ref}' not found", file=sys.stderr)
        sys.exit(1)

    if "enum" in schema:
        print(f"// enum: {schema_name}")
        print("#[derive(Debug, Clone, Serialize, Deserialize)]")
        t = schema.get("type", "string")
        if t == "string":
            print(f"pub enum {schema_name} {{")
            for v in schema["enum"]:
                if v is None:
                    continue
                variant = v[0].upper() + v[1:] if v else "None"
                print(f"    {variant},")
            print("}")
        else:
            print(f"// TODO: non-string enum ({t}): {schema['enum']}")
        return

    print(gen_struct(schema_name, schema, spec))


if __name__ == "__main__":
    main()
