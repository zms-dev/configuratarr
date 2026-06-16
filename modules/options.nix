{ lib, pkgs, enableDescription }:

let
  yamlFormat = pkgs.formats.yaml { };
in {
  enable = lib.mkEnableOption enableDescription;

  package = lib.mkOption {
    type = lib.types.package;
    default = pkgs.configuratarr;
    description = "The configuratarr package to use.";
  };

  settings = lib.mkOption {
    type = yamlFormat.type;
    default = {};
    description = "Declarative configuration options for configuratarr.";
  };

  prune = lib.mkOption {
    type = lib.types.bool;
    default = false;
    description = "Whether to prune server-side resources that are not declared in the config file.";
  };

  wait = lib.mkOption {
    type = lib.types.bool;
    default = true;
    description = "Wait for target applications to be online before running synchronization.";
  };

  waitTimeout = lib.mkOption {
    type = lib.types.ints.unsigned;
    default = 30;
    description = "Timeout in seconds to wait for each application to become online.";
  };
}
