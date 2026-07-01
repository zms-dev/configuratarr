# Dev shells. Returns { default, e2e }.
{
  pkgs,
  rustToolchain,
  tools,
}:
let
  default = pkgs.mkShell {
    buildInputs =
      with pkgs;
      [
        rustToolchain
        python3
        pkg-config
        openssl
        cargo-nextest
        shellcheck
        jq
        curl
      ]
      ++ tools;

    shellHook = ''
      echo "=== Configuratarr Nix DevShell ==="
      rustc --version
      cargo --version

      export RUST_BACKTRACE=1
      export RUST_LOG=info
      export CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true
    '';
  };
in
let
  e2e = pkgs.mkShell {
    inputsFrom = [ default ];
    buildInputs = with pkgs; [
      radarr
      sonarr
      prowlarr
      lidarr
      readarr
    ];
    shellHook = ''
      echo "=== Configuratarr E2E DevShell ==="
    '';
  };
in
{
  inherit default e2e;

  e2e-radarr = import ./e2e-shells/radarr-v3.nix {
    inherit pkgs;
    e2eShell = e2e;
  };

  e2e-sonarr = import ./e2e-shells/sonarr-v3.nix {
    inherit pkgs;
    e2eShell = e2e;
  };

  e2e-prowlarr = import ./e2e-shells/prowlarr-v1.nix {
    inherit pkgs;
    e2eShell = e2e;
  };

  e2e-lidarr = import ./e2e-shells/lidarr-v1.nix {
    inherit pkgs;
    e2eShell = e2e;
  };
}
