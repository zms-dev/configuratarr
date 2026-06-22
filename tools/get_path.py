#!/usr/bin/env python3
"""
Get full details for an API path from an OpenAPI spec.

Usage:
  get_path.py <spec.json> <path>

Example:
  get_path.py radarr-v3.json /api/v3/config/naming
"""
import json
import sys


def load_spec(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def extract_schema_ref(schema: dict) -> str | None:
    if "$ref" in schema:
        return schema["$ref"]
    if schema.get("type") == "array" and "$ref" in schema.get("items", {}):
        return f"array<{schema['items']['$ref']}>"
    return None


def main():
    args = sys.argv[1:]
    if len(args) < 2 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path, target_path = args[0], args[1]
    spec = load_spec(spec_path)
    paths = spec.get("paths", {})

    path_item = paths.get(target_path)
    if path_item is None:
        # Try prefix match
        matches = [p for p in paths if p.startswith(target_path)]
        if not matches:
            print(f"ERROR: path '{target_path}' not found", file=sys.stderr)
            sys.exit(1)
        if len(matches) > 1:
            print(f"Multiple matches (be more specific):")
            for m in sorted(matches):
                print(f"  {m}")
            sys.exit(1)
        target_path = matches[0]
        path_item = paths[target_path]

    print(f"Path: {target_path}")
    print()

    http_methods = [m for m in ("get", "post", "put", "delete", "patch") if m in path_item]

    for method in http_methods:
        op = path_item[method]
        print(f"  {method.upper()}")

        # Parameters
        params = op.get("parameters", [])
        if params:
            for p in params:
                loc = p.get("in", "?")
                pname = p.get("name", "?")
                pschema = p.get("schema", {})
                ptype = pschema.get("type", "?")
                default = pschema.get("default")
                req = p.get("required", False)
                default_str = f" = {default!r}" if default is not None else ""
                req_str = " (required)" if req else ""
                print(f"    param [{loc}] {pname}: {ptype}{default_str}{req_str}")

        # Request body
        rb = op.get("requestBody", {})
        if rb:
            for ct, ct_val in rb.get("content", {}).items():
                ref = extract_schema_ref(ct_val.get("schema", {}))
                if ref:
                    print(f"    request body: {ref}")

        # Responses
        for status, resp in op.get("responses", {}).items():
            for ct, ct_val in resp.get("content", {}).items():
                ref = extract_schema_ref(ct_val.get("schema", {}))
                if ref:
                    print(f"    response {status}: {ref}")
                elif resp.get("description"):
                    print(f"    response {status}: {resp['description']}")

        print()


if __name__ == "__main__":
    main()
