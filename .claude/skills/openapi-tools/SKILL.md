---
name: openapi-tools
description: >
  Use this skill whenever working with OpenAPI specs in the configuratarr project — inspecting
  schemas, discovering resources, comparing schemas across APIs, checking what fields a resource
  has, generating Rust struct models, or verifying an existing Rust file matches its spec.
  Auto-triggers when: writing a new resource struct, investigating what schemas exist, comparing
  radarr vs sonarr schemas, checking if a Rust file is missing spec fields, or generating
  boilerplate for a new API resource.
---

# OpenAPI Tools

Six Python scripts in `tools/` expose OpenAPI spec inspection and Rust model generation.
All are on PATH in the nix devshell. Always use these before writing or editing resource structs.

## Tools

### Discovery
**`list_resources <spec>`** — all schema names with tags (`[crud]`, `[singleton]`, `[provider]`, `[enum]`).
Flags: `--crud`, `--singleton`, `--provider`, `--enums`, `--no-enums`
```
list_resources specs/radarr-v3.json --singleton
list_resources specs/radarr-v3.json --crud --no-enums
```

**`list_paths <spec> [prefix]`** — all API paths with HTTP methods.
```
list_paths specs/radarr-v3.json /api/v3/config
```

### Schema Inspection
**`get_resource <spec> "<ref>"`** — full schema detail: field types, nullability, readOnly, defaults,
enum values resolved inline, which paths use it, and classification (CRUD/singleton/read-only).
```
get_resource specs/radarr-v3.json "#/components/schemas/DelayProfileResource"
```

**`get_path <spec> <path>`** — full path detail: methods, parameters, request/response schema refs.
```
get_path specs/radarr-v3.json /api/v3/config/naming/{id}
```

**`compare_schemas <spec1> "<ref1>" <spec2> "<ref2>" [--all]`** — field diff between two schemas,
optionally across two different specs. Shows only-in-A, only-in-B, type differences. `--all` also shows identical fields.
```
compare_schemas specs/radarr-v3.json "#/components/schemas/QualityProfileResource" \
                specs/sonarr-v3.json "#/components/schemas/QualityProfileResource"
```

### Diff Against Existing Code
**`diff_resource <spec> "<ref>" <file.rs> [StructName]`** — compares spec fields directly against
a Rust struct. Does NOT generate then diff — parses the Rust file and compares field-by-field.
Shows: missing from Rust, extra in Rust, type mismatches. Use before editing any existing resource file.
```
diff_resource specs/radarr-v3.json "#/components/schemas/IndexerResource" \
              crates/radarr-v3/src/resources/indexer.rs Indexer
```

### Code Generation
**`gen_resource_model <spec> "<ref>"`** — stdout Rust struct. Applies: `rename_all = "camelCase"`,
snake_case aliases, `skip_serializing` for readOnly fields, `default`/`default_true`/`default_<field>`
functions, `Option<i32>` for nullable id, `// TODO` comments for FK integers and ambiguous cases.
Output is a suggestion — decide whether to write fresh or apply as a diff.
```
gen_resource_model specs/radarr-v3.json "#/components/schemas/DelayProfileResource"
```

## Workflow: Writing a New Resource

1. `list_paths specs/<api>.json /api/v3/<resource>` — confirm endpoint methods
2. `get_resource specs/<api>.json "#/components/schemas/<Name>"` — understand all fields
3. `gen_resource_model specs/<api>.json "#/components/schemas/<Name>"` — get starter struct
4. Review TODOs in generated output — fix FK types (`ResourceRef<T>`), add missing defaults
5. Add tests: API camelCase deser, config snake_case deser, serializes camelCase, YAML deser

## Workflow: Verifying an Existing Resource

1. `diff_resource specs/<api>.json "#/components/schemas/<Name>" <file.rs> <StructName>`
2. If fields missing → add them; if extra → verify intentional (e.g., `ResourceRef<T>` aliases)
3. If suspicious → `get_resource` to see readOnly/nullable status on divergent fields

## Workflow: Multi-API Field Comparison

1. `compare_schemas specs/radarr-v3.json "#/.../QualityProfileResource" specs/sonarr-v3.json "#/.../QualityProfileResource"`
2. Fields only in one API → API-specific additions to that crate
3. Same fields, different types → may need separate structs or conditional handling

## Spec Files

All specs live in `specs/`. Current: `specs/radarr-v3.json`.
Schema refs always use full format: `"#/components/schemas/FooResource"`.
