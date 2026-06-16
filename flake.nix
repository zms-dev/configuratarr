{
  description = "Configuratarr - A declarative configuration stack-sync engine for Sonarr, Radarr, Prowlarr, Lidarr, and Readarr.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    let
      outputs = flake-utils.lib.eachDefaultSystem (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          rustToolchain = fenix.packages.${system}.stable.withComponents [
            "rustc"
            "cargo"
            "clippy"
            "rustfmt"
            "rust-src"
          ];
          configuratarr = pkgs.callPackage ./pkgs/default.nix {
            docs = pkgs.callPackage ./modules/docs.nix { };
          };
        in
        {
          packages = {
            inherit configuratarr;
            default = configuratarr;
          };

          formatter = pkgs.nixfmt-tree;

          apps.generate-docs = {
            type = "app";
            program = "${pkgs.writeShellScript "generate-docs" ''
              echo "==> Copying generated NixOS and Home Manager options docs..."
              cp -f ${configuratarr.docs}/NIXOS_OPTIONS.md docs/NIXOS_OPTIONS.md
              cp -f ${configuratarr.docs}/HOME_MANAGER_OPTIONS.md docs/HOME_MANAGER_OPTIONS.md
              echo "==> Done!"
            ''}";
          };

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustToolchain
              python3
              pkg-config
              openssl

              # Dev & Testing Utilities
              cargo-nextest
              shellcheck
              jq
              curl
            ];

            shellHook = ''
              echo "=== Configuratarr Nix DevShell ==="
              rustc --version
              cargo --version

              # Environment variables
              export RUST_BACKTRACE=1
              export RUST_LOG=info
              export CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true
            '';
          };
        }
      );
    in
    outputs
    // {
      nixosModules.default = import ./modules/nixos.nix;
      homeManagerModules.default = import ./modules/home-manager.nix;
    };
}
