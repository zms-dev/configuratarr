---
name: add-resource
description: >
  Procedure + reference for adding a resource to an existing service crate in configuratarr
  (e.g. a new Radarr resource, or a resource in a sibling *arr crate). Covers the three
  archetypes (collection / singleton / provider), the macro-per-codec mapping, the field
  attribute reference, wiring into mod.rs + the service struct, and the conformance test.
  Auto-triggers when: adding/modeling a resource struct, choosing a codec or archetype,
  deciding which field attribute to use, or wiring a new resource into a service.
---

# Adding a Resource

Mirror `crates/radarr-v3/` exactly — it is the canonical template. Every decision there was deliberate. Do not invent alternatives.

## 0. Inspect the spec first

Use the `openapi-tools` skill before writing any struct: `list_paths` to confirm the endpoint, `get_resource` for the full field list + which fields are `readOnly`, `gen_resource_model` for a starter struct.

## 1. Pick the archetype

| Archetype | When | `sync` |
|---|---|---|
| **Collection** | named, CRUD (tags, profiles) | `crud` |
| **Singleton** | one object, no key (config/*) | `singleton` |
| **Provider** | dynamic `fields[]` blob (download clients, indexers, notifications, import lists) | `crud` + composition |

## 2. The codec is selected by *which macro* you use

| Macro | CodecKind | For |
|---|---|---|
| `#[resource(...)]` | Standard | flat JSON (camelCase wire keys) |
| `#[fields_blob(...)]` | FieldsBlob | provider variant configs (`fields:[{name,value}]`) |
| `#[tagged(by = "...")]` | TaggedByImpl | discriminator-dispatched provider enum |
| `#[wire_enum(rename_all = "...")]` | StringEnum | unit enum → bare string |
| `#[nested]` | Standard, Embedded | shared sub-struct hoisted via `#[flatten]` |

**Macro ordering gotcha:** the resource macro is the **outermost** attribute; field helper attrs (`#[id]`, `#[key]`, `#[wire(...)]`, …) sit **below** it, or they leak through unstripped and rustc rejects them.

## 3. Archetype shapes

### Collection — `tag.rs`
```rust
#[resource(
    sync = crud,
    list   = get("/api/v3/tag"),
    create = post("/api/v3/tag"),
    update = put("/api/v3/tag/${self.id}"),
    delete = delete("/api/v3/tag/${self.id}"),
)]
pub struct Tag {
    #[id]  pub id: Option<i32>,
    #[key] pub label: String,        // natural key — ${ref.tag.<label>}
}
```

### Singleton — `ui_config.rs`
```rust
#[resource(
    sync = singleton,
    read   = get("/api/v3/config/ui"),
    update = put("/api/v3/config/ui/${self.id}"),   // write path is /{id}
)]
pub struct UiConfig {
    #[id] pub id: Option<i32>,
    pub first_day_of_week: i32,
    pub theme: Option<String>,
}
```

### Provider — three composed pieces

**Envelope** (`provider.rs`, shared) — `#[nested]` struct with identity + tag refs + read-only metadata:
```rust
#[nested]
pub struct Provider {
    #[id]  pub id: Option<i32>,
    #[key] pub name: String,
    #[reference(tag)] pub tags: Vec<i32>,
    #[wire(name = "implementationName", read_only)] pub implementation_name: Option<String>,
    #[wire(read_only)] pub message: Option<ProviderMessage>,
    #[wire(read_only)] pub presets: Vec<Json>,
}
```

**Outer resource** (`download_client.rs`) — flattens envelope + typed config, adds its own fields:
```rust
#[resource(sync = crud, list = get("/api/v3/downloadclient"), create = post(...), update = put(".../${self.id}"), delete = delete(".../${self.id}"))]
pub struct DownloadClient {
    #[flatten] pub common: Provider,
    #[flatten] pub config: DownloadClientProvider,
    pub enable: bool,
    pub protocol: DownloadProtocol,
    #[default(1)] pub priority: i32,
}
```

**Tagged enum** (`download_clients/mod.rs`) — one `#[variant("<Impl>")]` per implementation, `#[fallback]` preserves unmodelled impls via `RawProvider`:
```rust
#[tagged(by = "implementation")]
pub enum DownloadClientProvider {
    #[variant("QBittorrent")] QBittorrent(QBittorrentConfig),
    // ... one per impl, one file each under download_clients/
    #[fallback] Unknown(RawProvider),
}
```

**Variant** (`download_clients/qbittorrent.rs`) — each typed field → one `{name,value}` blob entry:
```rust
#[fields_blob(implementation = "QBittorrent", config_contract = "QBittorrentSettings", protocol = "torrent")]
pub struct QBittorrentConfig {
    pub host: Option<String>,
    pub password: Option<SecretValue>,                    // credential — redacted
    #[wire(name = "useSsl")] pub use_ssl: Option<bool>,   // override irregular API casing
}
```

## 4. Field attribute reference

| Situation | Attribute |
|---|---|
| Server-assigned id (read-only on write) | `#[id]` |
| Natural key (`${ref.<type>.<key>}` + diff key) | `#[key]` |
| FK to another resource by name | `#[reference(<type>)]` on `i32` / `Option<i32>` / `Vec<i32>` |
| Wire name ≠ snake→camel | `#[wire(name = "...")]` |
| Present on read, never sent | `#[wire(read_only)]` |
| Nested struct hoisted to parent keys | `#[flatten]` |
| Non-zero default for an absent config key | `#[default(expr)]` |
| Credential | field type `SecretValue` (no attribute — inferred) |

Endpoints are **always explicit** (`verb("/path")`); paths may carry `${self.<field>}` (resolved against the live/merged value at apply, e.g. `/api/v3/tag/${self.id}`).

## 5. Wire it in

1. `pub mod <resource>;` in `src/resources/mod.rs`.
2. Add the field to the `#[service]` struct (`Vec<R>` = collection, `Option<R>` = singleton).
3. Add a `tests/testdata/<resource>/config.yaml` fixture (realistic snake_case user config; may use `${ref}` / `${env}`). **Cover every non-scalar field** — `Option<Nested>`, `Vec<Nested>`, nested objects — not just the scalar happy path. A field the fixture omits is a field no test exercises: that's exactly how `quality_profile.items` (a `Vec<Nested>`) silently encoded as `[]` for ages. (You don't need to include read-only fields — they're never written.)
4. Add one line to `tests/spec_conformance.rs`:
   `conformance!(<name>, resources::<r>::<Type>, "<SchemaName>Resource", "testdata/<r>/config.yaml");`
5. Add a `///` doc comment to the struct + each field (feeds `config-doc-gen`).

## 6. Verify

```bash
nix develop --command cargo test -p <crate> --test spec_conformance   # wire shape vs OpenAPI schema
nix develop --command cargo run -q -p config-doc-gen                  # regenerate docs
```
Conformance validates the encoded payload against the spec schema (`additionalProperties: false` catches mis-casing / hallucinated / mis-typed fields) **and** checks the codec round-trips (decode→encode is byte-stable — catches a field that decodes but doesn't re-encode, which schema validation alone misses since a dropped array still validates as an array). Both only fire on fields the fixture actually includes — hence the "cover every non-scalar field" rule above. For lifecycle/FK behavior, add an e2e case (see the `add-service` skill's e2e section).
