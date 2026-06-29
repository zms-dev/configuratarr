---
name: add-service
description: >
  Full checklist for adding a new API service crate to configuratarr (sonarr-v3, lidarr-v1,
  prowlarr-v1, jellyfin, etc.). Covers scaffolding, Cargo wiring, the #[service] struct,
  registering in core-config + config-doc-gen, the test layout via core-testkit, and the Nix
  e2e wiring. Auto-triggers when: starting a new service crate, scaffolding <api>-<version>,
  wiring a service into the registry, or setting up a service's tests/Nix.
---

# Adding a New Service Crate

Mirror `crates/radarr-v3/` exactly. For per-resource modeling, use the `add-resource` skill. For specs, `openapi-tools`; for build/test commands, `nix-devshell`.

## 1. Scaffold

```
crates/<api>-<version>/
    Cargo.toml
    spec/<api>.json            # move the OpenAPI spec here from specs/
    src/
        lib.rs                 # pub mod resources; pub mod service; pub use service::<Api>;
        service.rs             # the #[service] struct
        resources/
            mod.rs
            <resource>.rs ...  # + <provider>s/ subdirs for providers
    tests/
        spec_conformance.rs    # thin — uses core_testkit::check
        e2e.rs                 # thin — uses core_testkit::{env_pair, instance}
        testdata/<r>/config.yaml
```

## 2. Cargo.toml

```toml
[package]
name = "<api>-<version>"
version = "0.1.0"
edition = "2024"

[dependencies]
core-lib    = { path = "../core" }
core-macros = { path = "../core-macros" }
anyhow.workspace     = true
serde_json.workspace = true

[dev-dependencies]
core-testkit = { path = "../core-testkit" }
serde-saphyr.workspace = true
tokio.workspace        = true
jsonschema.workspace   = true
```
Local crates by path; everything else `.workspace = true` (never pin a version in a crate manifest). Add `"crates/<api>-<version>"` to `[workspace.members]` in the root `Cargo.toml`.

## 3. Resources

Per resource, follow the `add-resource` skill. FK `i32` → `#[reference(t)]`; credentials → `SecretValue`; read-only → `#[wire(read_only)]`.

## 4. service.rs — `#[service]`

One struct carries connection fields **and** every managed resource. `Vec<R>` = collection, `Option<R>` = singleton (the macro reads each resource's own `SyncKind` from its descriptor).

```rust
#[service(name = "<api>_<version>", health = "/api/v3/system/status", auth = api_key(header = "X-Api-Key"))]
pub struct <Api> {
    pub url: String,
    #[credential(api_key)] pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    pub tags: Vec<Tag>,                       // collections
    pub media_management: Option<MediaManagement>,  // singletons
}
```
Auth schemes: `none`, `api_key(header = "...")`, `bearer`, `basic`, `form_cookie(login_path = "...")` with `#[credential(api_key|bearer|user|pass)]`. The executor implements None/ApiKey/Bearer today; Basic/FormCookie `bail!` (async auth is a pending seam).

Declaring `health = "<path>"` wires the service into `--wait-for-healthy`: the CLI polls that endpoint (authenticated) until it responds before any plan/apply. Omit it and the service simply isn't waited on.

## 5. Register + announce

- `crates/core-config/src/lib.rs` — add a `ServiceInstance` variant + dep:
  `#[serde(rename = "<api>-<version>")] <ApiVersion>(<api_version>::<Api>),` (and the crate dep in `core-config/Cargo.toml`).
- `crates/config-doc-gen/src/main.rs` — add one line to the `docs` array:
  `("<api>-<version>", render_service::<<api_version>::<Api>>("<Human Name>"))` + the crate dep. (Doc-gen is generic — providers/nested types document themselves.)
- `README.md` — flip the service's row in the **Supported services** table to `✅ Supported` (add a row if it's a new app, with its API version + config `type`).

## 6. Tests (thin — harness is shared)

**`spec_conformance.rs`** — `spec()` loader + a `conformance!` line per resource + the teeth test. The harness is `core_testkit::check`.

**`e2e.rs`** — env-guarded, `#[ignore]`:
```rust
use core_testkit::{env_pair, instance};
fn env() -> Option<(String, String)> { env_pair("<SERVICE>_URL", "<SERVICE>_API_KEY") }
async fn run(url, key, resources, opts) -> Report {
    let (svc, value) = instance::<<Api>>(url, key, resources);
    apply(&svc, &value, opts).await.unwrap()
}
```
Keep both files thin — declarations only; the service-agnostic mechanics live in `core-testkit`.

## 7. Nix wiring (five touches — copy the radarr-v3 equivalents)

- `nix/e2e/<api>-<version>.nix` — the NixOS VM test.
- `nix/e2e-shells/<api>-<version>.nix` — local fast-iteration shell (starts the service, exports `<SERVICE>_URL` + `<SERVICE>_API_KEY`).
- `nix/shells.nix` — `e2e-<api> = import ./e2e-shells/<api>-<version>.nix { inherit pkgs; e2eShell = e2e; };`
- `flake.nix` — merge `mkServiceChecks "<api>-<version>" (import ./nix/e2e/<api>-<version>.nix { inherit pkgs; })` with `//`.
- `modules/nixos.nix` — add `"<app>.service"` to the systemd `after` list, so configuratarr runs after the app it configures (ordering only; ignored when the app is remote or absent).

## 8. Verify

```bash
nix develop --command cargo nextest run -p <api>-<version>      # unit + conformance
nix develop --command cargo run -q -p config-doc-gen            # docs/<api>-<version>-config.md
nix develop .#e2e-<api> --command cargo nextest run -p <api>-<version> --test e2e --run-ignored all
```
