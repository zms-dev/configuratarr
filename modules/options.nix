{
  lib,
  pkgs,
  enableDescription,
}:

let
  hostConfigSubmodule = lib.types.submodule {
    options = {
      url = lib.mkOption {
        type = lib.types.str;
        description = "The base URL of the service instance.";
        example = "http://localhost:8083";
      };

      apiKey = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = null;
        description = "The API key for authentication.";
      };
    };
  };

  apiFieldSubmodule = lib.types.submodule {
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "The name of the configuration field.";
      };
      value = lib.mkOption {
        type = lib.types.anything;
        description = "The value of the configuration field.";
      };
    };
  };

  downloadClientSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the download client.";
      };
      enable = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Whether to enable the download client.";
      };
      protocol = lib.mkOption {
        type = lib.types.str;
        description = "Download protocol (usenet or torrent).";
      };
      priority = lib.mkOption {
        type = lib.types.ints.unsigned;
        default = 1;
        description = "Priority order.";
      };
      implementation = lib.mkOption {
        type = lib.types.str;
        description = "Class name of download client implementation (e.g. QBittorrent, SABnzbd).";
      };
      removeCompletedDownloads = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Whether to automatically remove completed downloads.";
      };
      removeFailedDownloads = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Whether to automatically remove failed downloads.";
      };
      fields = lib.mkOption {
        type = lib.types.listOf apiFieldSubmodule;
        default = [ ];
        description = "Implementation-specific configuration fields.";
      };
    };
  };

  indexerSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the indexer.";
      };
      enableRss = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Enable RSS feeds parsing.";
      };
      enableAutomaticSearch = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Enable automatic search queries.";
      };
      enableInteractiveSearch = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Enable interactive manual search queries.";
      };
      protocol = lib.mkOption {
        type = lib.types.str;
        description = "Indexer protocol (torrent or usenet).";
      };
      priority = lib.mkOption {
        type = lib.types.ints.unsigned;
        default = 25;
        description = "Search order priority.";
      };
      downloadClientId = lib.mkOption {
        type = lib.types.ints.unsigned;
        default = 0;
        description = "Restrict indexer to specific download client ID (0 for any).";
      };
      implementation = lib.mkOption {
        type = lib.types.str;
        description = "Indexer implementation class (e.g. Torznab, Newznab).";
      };
      fields = lib.mkOption {
        type = lib.types.listOf apiFieldSubmodule;
        default = [ ];
        description = "Implementation-specific configuration fields.";
      };
    };
  };

  notificationSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the notification connection.";
      };
      implementation = lib.mkOption {
        type = lib.types.str;
        description = "Notification service type (e.g. Discord, Telegram, Webhook).";
      };
      fields = lib.mkOption {
        type = lib.types.listOf apiFieldSubmodule;
        default = [ ];
        description = "Implementation-specific configuration fields.";
      };
    };
  };

  qualityProfileSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the quality profile.";
      };
      upgradeAllowed = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Allow upgrading media files over time.";
      };
      cutoff = lib.mkOption {
        type = lib.types.ints.unsigned;
        description = "Quality ID that stops the upgrade cycle.";
      };
    };
  };

  customFormatSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the custom format.";
      };
    };
  };

  rootFolderSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      path = lib.mkOption {
        type = lib.types.str;
        description = "Absolute directory path.";
      };
    };
  };

  importListSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the import list.";
      };
      enabled = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Enable list syncing.";
      };
      implementation = lib.mkOption {
        type = lib.types.str;
        description = "Import list implementation type.";
      };
      fields = lib.mkOption {
        type = lib.types.listOf apiFieldSubmodule;
        default = [ ];
        description = "Implementation-specific configuration fields.";
      };
    };
  };

  releaseProfileSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the release profile.";
      };
      enabled = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Whether the release profile is enabled.";
      };
    };
  };

  metadataProfileSubmodule = lib.types.submodule {
    freeformType = lib.types.attrsOf lib.types.anything;
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Name of the metadata profile.";
      };
    };
  };

  # Common option definitions to avoid duplication
  commonOptions = name: {
    host = lib.mkOption {
      type = lib.types.nullOr hostConfigSubmodule;
      default = null;
      description = "Host connection configuration for ${name}.";
    };

    downloadClients = lib.mkOption {
      type = lib.types.nullOr (lib.types.listOf downloadClientSubmodule);
      default = null;
      description = "Download clients for ${name}.";
    };

    indexers = lib.mkOption {
      type = lib.types.nullOr (lib.types.listOf indexerSubmodule);
      default = null;
      description = "Indexers for ${name}.";
    };

    notifications = lib.mkOption {
      type = lib.types.nullOr (lib.types.listOf notificationSubmodule);
      default = null;
      description = "Notification connections for ${name}.";
    };

    ui = lib.mkOption {
      type = lib.types.nullOr lib.types.attrs;
      default = null;
      description = "UI configuration for ${name}.";
    };
  };

  # Options shared by all apps except Prowlarr
  mediaAppOptions =
    name:
    (commonOptions name)
    // {
      qualityProfiles = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf qualityProfileSubmodule);
        default = null;
        description = "Quality profiles configuration for ${name}.";
      };

      rootFolders = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf rootFolderSubmodule);
        default = null;
        description = "Root folders configuration for ${name}.";
      };

      importLists = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf importListSubmodule);
        default = null;
        description = "Import lists configuration for ${name}.";
      };

      naming = lib.mkOption {
        type = lib.types.nullOr lib.types.attrs;
        default = null;
        description = "Naming configuration for ${name}.";
      };

      mediaManagement = lib.mkOption {
        type = lib.types.nullOr lib.types.attrs;
        default = null;
        description = "Media management configuration for ${name}.";
      };
    };

  # App-specific submodules
  prowlarrConfigSubmodule = lib.types.submodule {
    options = commonOptions "Prowlarr";
  };

  radarrConfigSubmodule = lib.types.submodule {
    options = (mediaAppOptions "Radarr") // {
      customFormats = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf customFormatSubmodule);
        default = null;
        description = "Custom formats configuration for Radarr.";
      };
    };
  };

  sonarrConfigSubmodule = lib.types.submodule {
    options = (mediaAppOptions "Sonarr") // {
      customFormats = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf customFormatSubmodule);
        default = null;
        description = "Custom formats configuration for Sonarr.";
      };
      releaseProfiles = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf releaseProfileSubmodule);
        default = null;
        description = "Release profiles configuration for Sonarr.";
      };
    };
  };

  lidarrConfigSubmodule = lib.types.submodule {
    options = (mediaAppOptions "Lidarr") // {
      customFormats = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf customFormatSubmodule);
        default = null;
        description = "Custom formats configuration for Lidarr.";
      };
      metadataProfiles = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf metadataProfileSubmodule);
        default = null;
        description = "Metadata profiles configuration for Lidarr.";
      };
    };
  };

  readarrConfigSubmodule = lib.types.submodule {
    options = (mediaAppOptions "Readarr") // {
      metadataProfiles = lib.mkOption {
        type = lib.types.nullOr (lib.types.listOf metadataProfileSubmodule);
        default = null;
        description = "Metadata profiles configuration for Readarr.";
      };
    };
  };

  configSubmodule = lib.types.submodule {
    options = {
      radarr = lib.mkOption {
        type = lib.types.nullOr radarrConfigSubmodule;
        default = null;
        description = "Configuration for Radarr.";
      };
      sonarr = lib.mkOption {
        type = lib.types.nullOr sonarrConfigSubmodule;
        default = null;
        description = "Configuration for Sonarr.";
      };
      prowlarr = lib.mkOption {
        type = lib.types.nullOr prowlarrConfigSubmodule;
        default = null;
        description = "Configuration for Prowlarr.";
      };
      lidarr = lib.mkOption {
        type = lib.types.nullOr lidarrConfigSubmodule;
        default = null;
        description = "Configuration for Lidarr.";
      };
      readarr = lib.mkOption {
        type = lib.types.nullOr readarrConfigSubmodule;
        default = null;
        description = "Configuration for Readarr.";
      };
    };
  };
in
{
  enable = lib.mkEnableOption enableDescription;

  package = lib.mkOption {
    type = lib.types.package;
    default = pkgs.configuratarr;
    description = "The configuratarr package to use.";
  };

  settings = lib.mkOption {
    type = configSubmodule;
    default = { };
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
