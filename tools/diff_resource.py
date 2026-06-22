#!/usr/bin/env python3
"""
Diff an OpenAPI schema against an existing Rust struct file.

Usage:
  diff_resource.py <spec.json> "#/components/schemas/Foo" <path/to/file.rs> [StructName]

Compares spec fields against Rust struct fields directly — does NOT generate then diff.
StructName is optional; defaults to the schema name.

Output:
  - Fields in spec missing from Rust struct
  - Fields in Rust struct not in spec
  - Type mismatches (best-effort inference)
"""
import json
import re
import sys
from pathlib import Path


def load_spec(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def resolve_ref(ref: str, spec: dict) -> dict:
    parts = ref.lstrip("#/").split("/")
    node = spec
    for p in parts:
        node = node[p]
    return node


def spec_rust_type(prop: dict, spec: dict) -> str:
    """Best-effort Rust type string from spec property."""
    if "$ref" in prop:
        ref = prop["$ref"]
        name = ref.split("/")[-1]
        resolved = resolve_ref(ref, spec)
        if "enum" in resolved:
            t = name
        else:
            t = name
        return f"Option<{t}>" if prop.get("nullable") else t
    t = prop.get("type", "")
    nullable = prop.get("nullable", False)
    rust = {
        "string": "String",
        "integer": "i32",
        "boolean": "bool",
        "number": "f64",
    }.get(t, t)
    if t == "array":
        items = prop.get("items", {})
        if "$ref" in items:
            inner = items["$ref"].split("/")[-1]
        else:
            inner = {"string": "String", "integer": "i32", "boolean": "bool"}.get(
                items.get("type", "?"), "?"
            )
        return f"Vec<{inner}>"
    return f"Option<{rust}>" if nullable else rust


def get_spec_fields(schema_ref: str, spec: dict) -> dict[str, str]:
    name = schema_ref.split("/")[-1]
    schema = spec.get("components", {}).get("schemas", {}).get(name)
    if schema is None:
        return {}
    return {
        field: spec_rust_type(prop, spec)
        for field, prop in schema.get("properties", {}).items()
    }


def camel_to_snake(name: str) -> str:
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z\d])([A-Z])", r"\1_\2", s)
    return s.lower()


def get_rust_fields(rs_path: str, struct_name: str) -> dict[str, str]:
    """Parse field names and types from a Rust struct definition."""
    text = Path(rs_path).read_text()

    # Find the struct block
    pattern = rf"pub struct {re.escape(struct_name)}\s*(?:<[^>]*>)?\s*\{{([^}}]*(?:\{{[^}}]*\}}[^}}]*)*)\}}"
    m = re.search(pattern, text, re.DOTALL)
    if not m:
        return {}

    body = m.group(1)
    fields = {}
    # Match: pub field_name: Type,
    for fm in re.finditer(r"pub\s+(\w+)\s*:\s*([^,\n]+)", body):
        fname = fm.group(1)
        ftype = fm.group(2).strip().rstrip(",").strip()
        fields[fname] = ftype
    return fields


def main():
    args = sys.argv[1:]
    if len(args) < 3 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path  = args[0]
    schema_ref = args[1]
    rs_path    = args[2]
    struct_name = args[3] if len(args) > 3 else schema_ref.split("/")[-1]

    spec = load_spec(spec_path)
    spec_fields = get_spec_fields(schema_ref, spec)
    if not spec_fields:
        print(f"ERROR: '{schema_ref}' not found or has no properties", file=sys.stderr)
        sys.exit(1)

    rust_fields = get_rust_fields(rs_path, struct_name)
    if not rust_fields:
        print(f"ERROR: struct '{struct_name}' not found in {rs_path}", file=sys.stderr)
        sys.exit(1)

    # Spec uses camelCase, Rust uses snake_case — normalize spec keys
    spec_snake = {camel_to_snake(k): (k, v) for k, v in spec_fields.items()}
    rust_keys  = set(rust_fields)
    spec_keys  = set(spec_snake)

    only_spec = sorted(spec_keys - rust_keys)
    only_rust = sorted(rust_keys - spec_keys)
    in_both   = sorted(spec_keys & rust_keys)

    print(f"Schema: {schema_ref}")
    print(f"Struct: {struct_name}  ({rs_path})")
    print()

    if only_spec:
        print(f"In spec, missing from Rust ({len(only_spec)}):")
        for f in only_spec:
            camel, rust_t = spec_snake[f]
            print(f"  + {f}  ({camel}: {rust_t})")
        print()

    if only_rust:
        print(f"In Rust, not in spec ({len(only_rust)}):")
        for f in only_rust:
            print(f"  - {f}: {rust_fields[f]}")
        print()

    # Type comparison (best-effort, Rust types are complex)
    mismatches = []
    for f in in_both:
        _, spec_t = spec_snake[f]
        rust_t = rust_fields[f]
        # Simple heuristic checks
        spec_opt = spec_t.startswith("Option<")
        rust_opt = rust_t.startswith("Option<") or rust_t.startswith("Vec<")
        if "Vec<" in spec_t and "Vec<" not in rust_t:
            mismatches.append((f, spec_t, rust_t, "spec=array, rust=scalar"))
        elif "Vec<" not in spec_t and "Vec<" in rust_t:
            mismatches.append((f, spec_t, rust_t, "spec=scalar, rust=array"))
        elif spec_opt and not rust_opt and "ResourceRef" not in rust_t:
            mismatches.append((f, spec_t, rust_t, "spec=optional, rust=required"))

    if mismatches:
        print(f"Potential type mismatches ({len(mismatches)}):")
        for f, st, rt, note in mismatches:
            print(f"  ~ {f}:  spec={st}  rust={rt}  ({note})")
        print()

    matched = len(in_both) - len(mismatches)
    print(f"Summary: {matched} matched, {len(only_spec)} missing from Rust, {len(only_rust)} extra in Rust, {len(mismatches)} type warnings")


if __name__ == "__main__":
    main()
