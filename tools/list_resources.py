#!/usr/bin/env python3
"""
List all schema names from an OpenAPI spec.

Usage:
  list_resources.py <spec.json> [--provider] [--singleton] [--crud] [--enums] [--no-enums]

Flags:
  --provider    Only schemas with fields+implementation+configContract (provider-style)
  --singleton   Only schemas that appear in GET+PUT but never POST (config singletons)
  --crud        Only schemas that appear in POST+PUT+DELETE (full CRUD)
  --enums       Only enum schemas
  --no-enums    Exclude enum schemas (useful default for resource scanning)
"""
import json
import sys
from pathlib import Path


def load_spec(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def is_enum(schema: dict) -> bool:
    return "enum" in schema


def is_provider(schema: dict) -> bool:
    props = schema.get("properties", {})
    return all(k in props for k in ("fields", "implementation", "configContract"))


def build_schema_path_index(spec: dict) -> dict[str, set[str]]:
    """Map schema ref -> set of HTTP methods used on paths where that schema appears.

    Includes methods like DELETE that don't reference a schema body — if a schema
    appears in GET/PUT on a path, DELETE on the same path is attributed to it too.
    """
    # Collect refs and all methods per path
    path_data: dict[str, tuple[set[str], set[str]]] = {}
    for path, path_item in spec.get("paths", {}).items():
        refs: set[str] = set()
        methods: set[str] = set()
        for method, op in path_item.items():
            if not isinstance(op, dict):
                continue
            methods.add(method.upper())
            rb = op.get("requestBody", {})
            for ct in rb.get("content", {}).values():
                s = ct.get("schema", {})
                if "$ref" in s:
                    refs.add(s["$ref"])
                elif "items" in s and "$ref" in s.get("items", {}):
                    refs.add(s["items"]["$ref"])
            for resp in op.get("responses", {}).values():
                for ct in resp.get("content", {}).values():
                    s = ct.get("schema", {})
                    if "$ref" in s:
                        refs.add(s["$ref"])
                    elif "items" in s and "$ref" in s.get("items", {}):
                        refs.add(s["items"]["$ref"])
        path_data[path] = (refs, methods)

    # Associate all path methods with schemas that appear anywhere on that path
    index: dict[str, set[str]] = {}
    for refs, methods in path_data.values():
        for ref in refs:
            index.setdefault(ref, set()).update(methods)
    return index


def is_singleton(ref: str, index: dict[str, set[str]]) -> bool:
    methods = index.get(ref, set())
    return "GET" in methods and "PUT" in methods and "POST" not in methods and "DELETE" not in methods


def is_crud(ref: str, index: dict[str, set[str]]) -> bool:
    methods = index.get(ref, set())
    return "POST" in methods and "PUT" in methods and "DELETE" in methods


def main():
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path = args[0]
    flags = set(args[1:])

    spec = load_spec(spec_path)
    schemas = spec.get("components", {}).get("schemas", {})
    index = build_schema_path_index(spec)

    want_provider  = "--provider"  in flags
    want_singleton = "--singleton" in flags
    want_crud      = "--crud"      in flags
    want_enums     = "--enums"     in flags
    no_enums       = "--no-enums"  in flags

    # If any positive filter active, start with nothing; else start with all
    active_filter = want_provider or want_singleton or want_crud or want_enums

    results = []
    for name, schema in sorted(schemas.items()):
        ref = f"#/components/schemas/{name}"
        enum = is_enum(schema)

        if no_enums and enum:
            continue

        if active_filter:
            match = False
            if want_enums and enum:
                match = True
            if want_provider and is_provider(schema):
                match = True
            if want_singleton and is_singleton(ref, index):
                match = True
            if want_crud and is_crud(ref, index):
                match = True
            if not match:
                continue

        tags = []
        if enum:
            tags.append("enum")
        elif is_provider(schema):
            tags.append("provider")
        elif is_singleton(ref, index):
            tags.append("singleton")
        elif is_crud(ref, index):
            tags.append("crud")

        tag_str = f"  [{', '.join(tags)}]" if tags else ""
        results.append(f"#/components/schemas/{name}{tag_str}")

    for r in results:
        print(r)


if __name__ == "__main__":
    main()
