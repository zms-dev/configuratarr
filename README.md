# Configuratarr

Configuratarr is a declarative, idempotent configuration synchronization engine for the `*arr` stack (Radarr, Sonarr, Lidarr, Prowlarr, Readarr) written in Rust. 

It allows you to manage application settings (download clients, indexers, profiles, notifications, folder paths, and global configurations) as code, making it ideal for GitOps workflows, automated deployments, and stateless systems like NixOS.

---

## Documentation

*   [**Configuration Reference (`docs/CONFIG.md`)**](docs/CONFIG.md): Documentation on the YAML config structure, application settings, and the dynamic secrets injection engine.
*   [**CLI Commands Reference (`docs/COMMANDS.md`)**](docs/COMMANDS.md): Detailed usage and syntax guide for all command-line subcommands (sync, status, list, add, delete, update, show).
*   [**NixOS Options (`docs/NIXOS_OPTIONS.md`)**](docs/NIXOS_OPTIONS.md): Configuration options for the NixOS service module.
*   [**Home Manager Options (`docs/HOME_MANAGER_OPTIONS.md`)**](docs/HOME_MANAGER_OPTIONS.md): Configuration options for the Home Manager service module.

---

## Key Features

*   **Idempotent Synchronization**: Computes the difference between your local configuration file and the server's current state, performing only the necessary additions, updates, or deletions.
*   **Dynamic Secret Resolution**: Pulls sensitive parameters (like API keys and passwords) at runtime from environment variables (`env://`) or decrypted files (`file://`), preventing credentials from being leaked in your configuration files or Nix store.
*   **Cookie Session Fallback**: Captures authentication cookies automatically if an API key is omitted, bypassing credential requirements on first-time boot or credential rotations.
*   **Comprehensive CLI Control**: Exposes direct subcommands to list, add, delete, update, and show settings for any specific resource collection or global singleton on any target application.

---

## Basic Usage

### Declarative Synchronization
Run a dry-run to view planned changes without mutating server state:
```bash
configuratarr sync --config configuratarr.yaml --plan
```

Apply the configuration changes to all instances:
```bash
configuratarr sync --config configuratarr.yaml --apply
```

### Direct CLI Mutations
List all configured download clients on Radarr:
```bash
configuratarr radarr download-client list
```

Update a global singleton setting directly:
```bash
configuratarr radarr ui update --field theme=dark
```

For more CLI command examples, see the [CLI Commands Reference](docs/COMMANDS.md).

---

## Development

The project uses Nix Flakes to provide a reproducible development shell.

### Running Tests
Execute the automated unit and integration tests using cargo:
```bash
cargo test
```

Regenerate command-line documentation from the clap schema:
```bash
GENERATE_DOCS=1 cargo test
```
