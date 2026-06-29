{
  description = "Configuratarr - A declarative configuration stack-sync engine for Sonarr, Radarr, Prowlarr, Lidarr, and Readarr.";

  nixConfig = {
    extra-substituters = [
      "https://configuratarr.cachix.org"
    ];
    extra-trusted-public-keys = [
      "configuratarr.cachix.org-1:3KlUWHpUzczNIKBFaSCWP3YNEc6N1S3OR3TQxcJmMdY="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      crane,
      flake-utils,
      ...
    }:
    let
      outputs = flake-utils.lib.eachDefaultSystem (
        system:
        let
          pkgs = import nixpkgs { inherit system; };

          mkTool =
            name:
            pkgs.writeShellScriptBin name ''
              exec ${pkgs.python3}/bin/python3 ${toString ./.}/tools/${name}.py "$@"
            '';
          tools = map mkTool [
            "list_resources"
            "get_resource"
            "list_paths"
            "get_path"
            "compare_schemas"
          ];

          inherit
            (import ./nix/crane.nix {
              inherit pkgs fenix crane;
              root = ./.;
            })
            rustToolchain
            craneLib
            commonArgs
            cargoArtifacts
            ;

          mkServiceChecks = import ./nix/mk-service-checks.nix {
            inherit
              pkgs
              craneLib
              commonArgs
              cargoArtifacts
              ;
          };

          configDocGen = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              pname = "config-doc-gen";
              cargoExtraArgs = "-p config-doc-gen --bin config-doc-gen";
            }
          );

          cmdDocGen = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              pname = "cmd-doc-gen";
              cargoExtraArgs = "-p cmd-doc-gen --bin cmd-doc-gen";
            }
          );

          configuratarr = pkgs.callPackage ./nix/package.nix {
            docs = pkgs.callPackage ./modules/docs.nix { };
          };
        in
        {
          packages = {
            inherit configuratarr;
            default = configuratarr;
          };

          # `workspace-tests` runs every crate's unit + integration tests
          # (`cargo nextest`, whole workspace, `#[ignore]`d e2e skipped — those run
          # in each service's `-e2e` VM check). Per-service checks add the VM e2e.
          # Add new services: // mkServiceChecks "sonarr-v3" (import ./nix/e2e/sonarr-v3.nix { inherit pkgs; })
          checks = {
            workspace-tests = craneLib.cargoNextest (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoNextestExtraArgs = "--workspace";
              }
            );

            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--workspace --all-targets -- -D warnings";
              }
            );

            rustfmt = craneLib.cargoFmt { inherit (commonArgs) src; };
          }
          // mkServiceChecks "radarr-v3" (import ./nix/e2e/radarr-v3.nix { inherit pkgs; });

          formatter = pkgs.nixfmt-tree;

          apps.generate-docs = {
            type = "app";
            program = "${pkgs.writeShellScript "generate-docs" ''
              echo "==> Copying generated NixOS and Home Manager options docs..."
              cp -f ${configuratarr.docs}/nixos_options.md docs/nixos_options.md
              cp -f ${configuratarr.docs}/home_manager_options.md docs/home_manager_options.md
              echo "==> Generating service config docs..."
              ${configDocGen}/bin/config-doc-gen --output-dir docs
              echo "==> Generating CLI command docs..."
              ${cmdDocGen}/bin/cmd-doc-gen > docs/commands.md
              echo "==> Done!"
            ''}";
          };

          devShells = import ./nix/shells.nix { inherit pkgs rustToolchain tools; };
        }
      );
    in
    outputs
    // {
      nixosModules.default = import ./modules/nixos.nix;
      homeManagerModules.default = import ./modules/home-manager.nix;
    };
}
