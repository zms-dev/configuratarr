{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.configuratarr;
in {
  options.services.configuratarr = {
    enable = mkEnableOption "Configuratarr declarative configuration service";

    package = mkOption {
      type = types.package;
      default = pkgs.configuratarr;
      description = "The configuratarr package to use.";
    };

    settings = mkOption {
      description = "Settings for the Configuratarr synchronization service.";
      default = {};
      type = types.submodule {
        options = {
          configFile = mkOption {
            type = types.path;
            description = "Path to the configuratarr.yaml configuration file.";
          };

          prune = mkOption {
            type = types.bool;
            default = false;
            description = "Whether to prune server-side resources that are not declared in the config file.";
          };

          wait = mkOption {
            type = types.bool;
            default = true;
            description = "Wait for target applications to be online before running synchronization.";
          };

          waitTimeout = mkOption {
            type = types.ints.unsigned;
            default = 30;
            description = "Timeout in seconds to wait for each application to become online.";
          };
        };
      };
    };
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
            --config ${cfg.settings.configFile} \
            ${lib.optionalString cfg.settings.prune "--prune"} \
            ${lib.optionalString cfg.settings.wait "--wait"} \
            --wait-timeout ${toString cfg.settings.waitTimeout} \
            --apply --auto-approve
        '';
        RemainAfterExit = true;
      };
    };
  };
}
