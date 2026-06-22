#!/usr/bin/env python3
"""
Get full details for a schema from an OpenAPI spec.

Usage:
  get_resource.py <spec.json> "#/components/schemas/Foo"

Output includes:
  - All properties with types, nullability, readOnly, defaults, enum values
  - $ref children resolved one level (shows actual field types)
  - Which paths/methods reference this schema and in what role
"""
import json
import sys


def load_spec(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def resolve_ref(ref: str, spec: dict) -> dict:
    """Resolve a $ref to its schema dict."""
    parts = ref.lstrip("#/").split("/")
    node = spec
    for p in parts:
        node = node[p]
    return node


def describe_type(prop: dict, spec: dict) -> str:
    if "$ref" in prop:
        ref = prop["$ref"]
        name = ref.split("/")[-1]
        resolved = resolve_ref(ref, spec)
        if "enum" in resolved:
            vals = resolved["enum"]
            return f"{name} (enum: {vals})"
        return name
    t = prop.get("type", "")
    if t == "array":
        items = prop.get("items", {})
        if "$ref" in items:
            inner = items["$ref"].split("/")[-1]
            return f"array<{inner}>"
        inner_t = items.get("type", "?")
        return f"array<{inner_t}>"
    if t == "number":
        fmt = prop.get("format", "")
        return f"number({fmt})" if fmt else "number"
    return t or "?"


def find_path_usages(schema_ref: str, spec: dict) -> list[tuple[str, str, str]]:
    """Returns list of (path, method, role) where role is 'request'|'response'."""
    usages = []
    for path, path_item in spec.get("paths", {}).items():
        for method, op in path_item.items():
            if not isinstance(op, dict):
                continue
            m = method.upper()
            found_role = None
            rb = op.get("requestBody", {})
            for ct in rb.get("content", {}).values():
                s = ct.get("schema", {})
                refs = []
                if "$ref" in s:
                    refs.append(s["$ref"])
                if "items" in s and "$ref" in s.get("items", {}):
                    refs.append(s["items"]["$ref"])
                if schema_ref in refs:
                    found_role = "request body"
            for status, resp in op.get("responses", {}).items():
                for ct in resp.get("content", {}).values():
                    s = ct.get("schema", {})
                    refs = []
                    if "$ref" in s:
                        refs.append(s["$ref"])
                    if "items" in s and "$ref" in s.get("items", {}):
                        refs.append(s["items"]["$ref"])
                    if schema_ref in refs and found_role is None:
                        found_role = f"response {status}"
            if found_role:
                usages.append((path, m, found_role))
    return usages


def main():
    args = sys.argv[1:]
    if len(args) < 2 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path, schema_ref = args[0], args[1]
    spec = load_spec(spec_path)
    schemas = spec.get("components", {}).get("schemas", {})

    name = schema_ref.split("/")[-1]
    schema = schemas.get(name)
    if schema is None:
        print(f"ERROR: schema '{schema_ref}' not found", file=sys.stderr)
        sys.exit(1)

    print(f"Schema: {schema_ref}")
    print()

    # Enum shortcut
    if "enum" in schema:
        print(f"Type: enum ({schema.get('type', 'string')})")
        print(f"Values: {schema['enum']}")
        return

    props = schema.get("properties", {})
    required = set(schema.get("required", []))

    if props:
        print("Properties:")
        col = max((len(k) for k in props), default=10) + 2
        for field_name, prop in props.items():
            type_str = describe_type(prop, spec)
            tags = []
            if field_name not in required and prop.get("nullable") is not True:
                if prop.get("type") != "array":
                    tags.append("optional")
            if prop.get("nullable"):
                tags.append("nullable")
            if prop.get("readOnly"):
                tags.append("readOnly")
            if "default" in prop:
                tags.append(f"default={prop['default']!r}")
            tag_str = f"  # {', '.join(tags)}" if tags else ""
            print(f"  {field_name:<{col}} {type_str}{tag_str}")
    else:
        print("Properties: (none)")

    print()
    usages = sorted(find_path_usages(schema_ref, spec))
    if usages:
        print("Used in paths:")
        for path, method, role in usages:
            print(f"  {method:<7} {path}  ({role})")
    else:
        print("Used in paths: (none — may be embedded only)")

    # Classify — check sibling path methods for DELETE (no response body to ref)
    methods_used = {m for _, m, _ in usages}
    request_methods = {m for _, m, r in usages if "request" in r}
    usage_paths = {p for p, _, _ in usages}
    has_delete = any(
        "delete" in spec.get("paths", {}).get(p, {})
        for p in usage_paths
    )
    print()
    if "POST" in request_methods and "PUT" in methods_used and has_delete:
        print("Classification: CRUD resource")
    elif "PUT" in methods_used and "POST" not in request_methods:
        print("Classification: singleton config (GET + PUT only)")
    elif not request_methods:
        print("Classification: read-only / embedded")
    else:
        print(f"Classification: mixed (methods: {sorted(methods_used)})")


if __name__ == "__main__":
    main()
