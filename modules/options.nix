{
  lib,
  pkgs,
  enableDescription,
}:
{
  enable = lib.mkEnableOption enableDescription;

  package = lib.mkOption {
    type = lib.types.package;
    default = pkgs.configuratarr;
    description = "The configuratarr package to use.";
  };

  settings = lib.mkOption {
    type = lib.types.attrs;
    default = { };
    description = ''
      Configuration written verbatim to `configuratarr.yaml`.

      Each top-level attribute is an instance you name, and its `type` selects
      the service (e.g. `radarr-v3`); add more attributes to manage more apps.
      See `docs/radarr-v3-config.md` (and the equivalent per-service docs) for
      the available fields of each resource.
    '';
    example = lib.literalExpression ''
      {
        my-radarr = {
          type = "radarr-v3";
          url = "http://localhost:7878";
          api_key = "\''${env.RADARR_API_KEY}";
          tags = [ { label = "managed"; } ];
        };
      }
    '';
  };

  prune = lib.mkOption {
    type = lib.types.bool;
    default = false;
    description = "Pass `--prune` to delete server-side resources absent from the config.";
  };
}
