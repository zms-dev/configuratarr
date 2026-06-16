# Configuratarr

[![CI](https://github.com/zms-dev/configuratarr/actions/workflows/ci.yml/badge.svg)](https://github.com/zms-dev/configuratarr/actions/workflows/ci.yml)
[![Dependency Updates](https://github.com/zms-dev/configuratarr/actions/workflows/flake-update.yml/badge.svg)](https://github.com/zms-dev/configuratarr/actions/workflows/flake-update.yml)
[![Cachix Cache](https://img.shields.io/badge/Cachix-configuratarr-blue.svg)](https://configuratarr.cachix.org)
[![Nix Built](https://img.shields.io/badge/Nix-Flake-blue.svg?logo=nixos&logoColor=white)](https://nixos.org)

Configuratarr is a declarative, idempotent configuration synchronization engine for the `*arr` stack (Radarr, Sonarr, Lidarr, Prowlarr, Readarr) written in Rust.

It allows you to manage application settings (download clients, indexers, profiles, notifications, folder paths, and global configurations) as code, making it ideal for GitOps workflows, automated deployments, and stateless systems like NixOS.

---

## Motivation

The main issue with managing the `*arr` stack (Sonarr, Radarr, etc.) declaratively is that almost all configuration—indexers, download clients, profiles, and notifications—is stored in a stateful SQLite database. The static `config.xml` file only covers basic boot settings.

Configuratarr solves this by acting as a declarative sync layer. It reads a desired state YAML file, compares it against the running app's API, and makes the necessary REST calls to sync the settings without touching the database directly.

The tool is split into a standalone Rust CLI and a set of Nix modules. Keeping the CLI generic ensures the engine remains portable (usable in Docker, Kubernetes, or non-Nix systems), while the Nix modules act as wrappers to handle systemd orchestration and option generation.

---

## 📚 Documentation

*   [**Configuration Reference (`docs/CONFIG.md`)**](docs/CONFIG.md): YAML schema structure, resource configurations, and secrets resolution.
*   [**CLI Commands Reference (`docs/COMMANDS.md`)**](docs/COMMANDS.md): Syntax and usage guide for sync, status, add, delete, and update commands.
*   [**NixOS Options (`docs/NIXOS_OPTIONS.md`)**](docs/NIXOS_OPTIONS.md): Configuration options for the NixOS service module.
*   [**Home Manager Options (`docs/HOME_MANAGER_OPTIONS.md`)**](docs/HOME_MANAGER_OPTIONS.md): Configuration options for the Home Manager service module.

---

## ✨ Key Features

*   **Idempotent Synchronization**: Compares your local YAML configuration against the active server state, applying only the necessary additions, updates, or deletions.
*   **Dynamic Secret Resolution**: Resolves credentials at runtime from environment variables (`env://`) or decrypted files (`file://`), preventing secrets from leaking into your Nix store or configuration repositories.
*   **Cookie Session Fallback**: Automatically captures authentication cookies if no API key is provided, ensuring seamless first-time bootstraps and credential rotations.
*   **Granular CLI Control**: Direct subcommands to query and mutate singletons or resource lists (e.g., adding download clients, updating UI preferences) across target apps.

---

## 🚀 Getting Started

### 1. Declarative Synchronization

Verify planned changes without mutating server state (dry-run):
```bash
configuratarr sync --config configuratarr.yaml --plan
```

Apply the configuration state to all targets:
```bash
configuratarr sync --config configuratarr.yaml --apply
```

### 2. Direct CLI Operations

List all configured download clients on Radarr:
```bash
configuratarr radarr download-client list
```

Update UI theme preferences directly:
```bash
configuratarr radarr ui update --field theme=dark
```

---

## ❄️ Nix Integration

### NixOS Module
```nix
{ inputs, ... }: {
  imports = [ inputs.configuratarr.nixosModules.default ];

  services.configuratarr = {
    enable = true;
    prune = true;
    settings = {
      # Declarative configuration options
    };
  };
}
```

### Home Manager Module
```nix
{ inputs, ... }: {
  imports = [ inputs.configuratarr.homeManagerModules.default ];

  services.configuratarr = {
    enable = true;
    settings = {
      # Declarative configuration options
    };
  };
}
```

---

## 🛠️ Development

This project uses Nix Flakes to provide a fully reproducible development shell.

### Run Tests
Execute the Rust unit and integration tests:
```bash
nix develop -c cargo test
```

### Regenerate Options Documentation
Rebuild the markdown option reference guides from option schemas:
```bash
nix run .#generate-docs
```
