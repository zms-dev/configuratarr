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
    enableDescription = "Configuratarr user configuration service";
  };

  config = mkIf cfg.enable {
    systemd.user.services.configuratarr = {
      Unit = {
        Description = "Configuratarr Stack Configuration Sync";
        After = [ "network.target" ];
      };

      Service = {
        Type = "oneshot";
        ExecStart = ''
          ${cfg.package}/bin/configuratarr sync \
            --config ${configFile} \
            ${lib.optionalString cfg.prune "--prune"} \
            ${lib.optionalString cfg.wait "--wait"} \
            --wait-timeout ${toString cfg.waitTimeout} \
            --apply --auto-approve
        '';
      };
    };
  };
}
