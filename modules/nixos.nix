{
  config,
  lib,
  pkgs,
  ...
}:

with lib;

let
  cfg = config.services.configuratarr;
  yamlFormat = pkgs.formats.yaml { };
  configFile = yamlFormat.generate "configuratarr.yaml" cfg.settings;
in
{
  options.services.configuratarr = import ./options.nix {
    inherit lib pkgs;
    enableDescription = "Configuratarr declarative configuration service";
  };

  config = mkIf cfg.enable {
    systemd.services.configuratarr = {
      description = "Configuratarr Stack Configuration Sync";
      wantedBy = [ "multi-user.target" ];
      after = [
        "network.target"
        "radarr.service"
        "sonarr.service"
        "prowlarr.service"
        "lidarr.service"
        "readarr.service"
      ];

      serviceConfig = {
        Type = "oneshot";
        ExecStart = ''
          ${cfg.package}/bin/configuratarr sync \
            --config ${configFile} \
            ${lib.optionalString cfg.prune "--prune"} \
            ${lib.optionalString cfg.wait "--wait"} \
            --wait-timeout ${toString cfg.waitTimeout} \
            --apply --auto-approve
        '';
        RemainAfterExit = true;
      };
    };
  };
}
