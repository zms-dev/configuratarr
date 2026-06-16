# Command-Line Help for `configuratarr`

This document contains the help content for the `configuratarr` command-line program.

**Command Overview:**

* [`configuratarr`↴](#configuratarr)
* [`configuratarr sync`↴](#configuratarr-sync)
* [`configuratarr radarr`↴](#configuratarr-radarr)
* [`configuratarr radarr sync`↴](#configuratarr-radarr-sync)
* [`configuratarr radarr status`↴](#configuratarr-radarr-status)
* [`configuratarr radarr download-client`↴](#configuratarr-radarr-download-client)
* [`configuratarr radarr download-client list`↴](#configuratarr-radarr-download-client-list)
* [`configuratarr radarr download-client delete`↴](#configuratarr-radarr-download-client-delete)
* [`configuratarr radarr download-client add`↴](#configuratarr-radarr-download-client-add)
* [`configuratarr radarr download-client update`↴](#configuratarr-radarr-download-client-update)
* [`configuratarr radarr indexer`↴](#configuratarr-radarr-indexer)
* [`configuratarr radarr indexer list`↴](#configuratarr-radarr-indexer-list)
* [`configuratarr radarr indexer delete`↴](#configuratarr-radarr-indexer-delete)
* [`configuratarr radarr indexer add`↴](#configuratarr-radarr-indexer-add)
* [`configuratarr radarr indexer update`↴](#configuratarr-radarr-indexer-update)
* [`configuratarr radarr root-folder`↴](#configuratarr-radarr-root-folder)
* [`configuratarr radarr root-folder list`↴](#configuratarr-radarr-root-folder-list)
* [`configuratarr radarr root-folder delete`↴](#configuratarr-radarr-root-folder-delete)
* [`configuratarr radarr root-folder add`↴](#configuratarr-radarr-root-folder-add)
* [`configuratarr radarr root-folder update`↴](#configuratarr-radarr-root-folder-update)
* [`configuratarr radarr quality-profile`↴](#configuratarr-radarr-quality-profile)
* [`configuratarr radarr quality-profile list`↴](#configuratarr-radarr-quality-profile-list)
* [`configuratarr radarr quality-profile delete`↴](#configuratarr-radarr-quality-profile-delete)
* [`configuratarr radarr quality-profile add`↴](#configuratarr-radarr-quality-profile-add)
* [`configuratarr radarr quality-profile update`↴](#configuratarr-radarr-quality-profile-update)
* [`configuratarr radarr custom-format`↴](#configuratarr-radarr-custom-format)
* [`configuratarr radarr custom-format list`↴](#configuratarr-radarr-custom-format-list)
* [`configuratarr radarr custom-format delete`↴](#configuratarr-radarr-custom-format-delete)
* [`configuratarr radarr custom-format add`↴](#configuratarr-radarr-custom-format-add)
* [`configuratarr radarr custom-format update`↴](#configuratarr-radarr-custom-format-update)
* [`configuratarr radarr metadata-profile`↴](#configuratarr-radarr-metadata-profile)
* [`configuratarr radarr metadata-profile list`↴](#configuratarr-radarr-metadata-profile-list)
* [`configuratarr radarr metadata-profile delete`↴](#configuratarr-radarr-metadata-profile-delete)
* [`configuratarr radarr metadata-profile add`↴](#configuratarr-radarr-metadata-profile-add)
* [`configuratarr radarr metadata-profile update`↴](#configuratarr-radarr-metadata-profile-update)
* [`configuratarr radarr release-profile`↴](#configuratarr-radarr-release-profile)
* [`configuratarr radarr release-profile list`↴](#configuratarr-radarr-release-profile-list)
* [`configuratarr radarr release-profile delete`↴](#configuratarr-radarr-release-profile-delete)
* [`configuratarr radarr release-profile add`↴](#configuratarr-radarr-release-profile-add)
* [`configuratarr radarr release-profile update`↴](#configuratarr-radarr-release-profile-update)
* [`configuratarr radarr ui`↴](#configuratarr-radarr-ui)
* [`configuratarr radarr ui show`↴](#configuratarr-radarr-ui-show)
* [`configuratarr radarr ui update`↴](#configuratarr-radarr-ui-update)
* [`configuratarr radarr naming`↴](#configuratarr-radarr-naming)
* [`configuratarr radarr naming show`↴](#configuratarr-radarr-naming-show)
* [`configuratarr radarr naming update`↴](#configuratarr-radarr-naming-update)
* [`configuratarr radarr media-management`↴](#configuratarr-radarr-media-management)
* [`configuratarr radarr media-management show`↴](#configuratarr-radarr-media-management-show)
* [`configuratarr radarr media-management update`↴](#configuratarr-radarr-media-management-update)
* [`configuratarr sonarr`↴](#configuratarr-sonarr)
* [`configuratarr sonarr sync`↴](#configuratarr-sonarr-sync)
* [`configuratarr sonarr status`↴](#configuratarr-sonarr-status)
* [`configuratarr sonarr download-client`↴](#configuratarr-sonarr-download-client)
* [`configuratarr sonarr download-client list`↴](#configuratarr-sonarr-download-client-list)
* [`configuratarr sonarr download-client delete`↴](#configuratarr-sonarr-download-client-delete)
* [`configuratarr sonarr download-client add`↴](#configuratarr-sonarr-download-client-add)
* [`configuratarr sonarr download-client update`↴](#configuratarr-sonarr-download-client-update)
* [`configuratarr sonarr indexer`↴](#configuratarr-sonarr-indexer)
* [`configuratarr sonarr indexer list`↴](#configuratarr-sonarr-indexer-list)
* [`configuratarr sonarr indexer delete`↴](#configuratarr-sonarr-indexer-delete)
* [`configuratarr sonarr indexer add`↴](#configuratarr-sonarr-indexer-add)
* [`configuratarr sonarr indexer update`↴](#configuratarr-sonarr-indexer-update)
* [`configuratarr sonarr root-folder`↴](#configuratarr-sonarr-root-folder)
* [`configuratarr sonarr root-folder list`↴](#configuratarr-sonarr-root-folder-list)
* [`configuratarr sonarr root-folder delete`↴](#configuratarr-sonarr-root-folder-delete)
* [`configuratarr sonarr root-folder add`↴](#configuratarr-sonarr-root-folder-add)
* [`configuratarr sonarr root-folder update`↴](#configuratarr-sonarr-root-folder-update)
* [`configuratarr sonarr quality-profile`↴](#configuratarr-sonarr-quality-profile)
* [`configuratarr sonarr quality-profile list`↴](#configuratarr-sonarr-quality-profile-list)
* [`configuratarr sonarr quality-profile delete`↴](#configuratarr-sonarr-quality-profile-delete)
* [`configuratarr sonarr quality-profile add`↴](#configuratarr-sonarr-quality-profile-add)
* [`configuratarr sonarr quality-profile update`↴](#configuratarr-sonarr-quality-profile-update)
* [`configuratarr sonarr custom-format`↴](#configuratarr-sonarr-custom-format)
* [`configuratarr sonarr custom-format list`↴](#configuratarr-sonarr-custom-format-list)
* [`configuratarr sonarr custom-format delete`↴](#configuratarr-sonarr-custom-format-delete)
* [`configuratarr sonarr custom-format add`↴](#configuratarr-sonarr-custom-format-add)
* [`configuratarr sonarr custom-format update`↴](#configuratarr-sonarr-custom-format-update)
* [`configuratarr sonarr metadata-profile`↴](#configuratarr-sonarr-metadata-profile)
* [`configuratarr sonarr metadata-profile list`↴](#configuratarr-sonarr-metadata-profile-list)
* [`configuratarr sonarr metadata-profile delete`↴](#configuratarr-sonarr-metadata-profile-delete)
* [`configuratarr sonarr metadata-profile add`↴](#configuratarr-sonarr-metadata-profile-add)
* [`configuratarr sonarr metadata-profile update`↴](#configuratarr-sonarr-metadata-profile-update)
* [`configuratarr sonarr release-profile`↴](#configuratarr-sonarr-release-profile)
* [`configuratarr sonarr release-profile list`↴](#configuratarr-sonarr-release-profile-list)
* [`configuratarr sonarr release-profile delete`↴](#configuratarr-sonarr-release-profile-delete)
* [`configuratarr sonarr release-profile add`↴](#configuratarr-sonarr-release-profile-add)
* [`configuratarr sonarr release-profile update`↴](#configuratarr-sonarr-release-profile-update)
* [`configuratarr sonarr ui`↴](#configuratarr-sonarr-ui)
* [`configuratarr sonarr ui show`↴](#configuratarr-sonarr-ui-show)
* [`configuratarr sonarr ui update`↴](#configuratarr-sonarr-ui-update)
* [`configuratarr sonarr naming`↴](#configuratarr-sonarr-naming)
* [`configuratarr sonarr naming show`↴](#configuratarr-sonarr-naming-show)
* [`configuratarr sonarr naming update`↴](#configuratarr-sonarr-naming-update)
* [`configuratarr sonarr media-management`↴](#configuratarr-sonarr-media-management)
* [`configuratarr sonarr media-management show`↴](#configuratarr-sonarr-media-management-show)
* [`configuratarr sonarr media-management update`↴](#configuratarr-sonarr-media-management-update)
* [`configuratarr prowlarr`↴](#configuratarr-prowlarr)
* [`configuratarr prowlarr sync`↴](#configuratarr-prowlarr-sync)
* [`configuratarr prowlarr status`↴](#configuratarr-prowlarr-status)
* [`configuratarr prowlarr download-client`↴](#configuratarr-prowlarr-download-client)
* [`configuratarr prowlarr download-client list`↴](#configuratarr-prowlarr-download-client-list)
* [`configuratarr prowlarr download-client delete`↴](#configuratarr-prowlarr-download-client-delete)
* [`configuratarr prowlarr download-client add`↴](#configuratarr-prowlarr-download-client-add)
* [`configuratarr prowlarr download-client update`↴](#configuratarr-prowlarr-download-client-update)
* [`configuratarr prowlarr indexer`↴](#configuratarr-prowlarr-indexer)
* [`configuratarr prowlarr indexer list`↴](#configuratarr-prowlarr-indexer-list)
* [`configuratarr prowlarr indexer delete`↴](#configuratarr-prowlarr-indexer-delete)
* [`configuratarr prowlarr indexer add`↴](#configuratarr-prowlarr-indexer-add)
* [`configuratarr prowlarr indexer update`↴](#configuratarr-prowlarr-indexer-update)
* [`configuratarr prowlarr root-folder`↴](#configuratarr-prowlarr-root-folder)
* [`configuratarr prowlarr root-folder list`↴](#configuratarr-prowlarr-root-folder-list)
* [`configuratarr prowlarr root-folder delete`↴](#configuratarr-prowlarr-root-folder-delete)
* [`configuratarr prowlarr root-folder add`↴](#configuratarr-prowlarr-root-folder-add)
* [`configuratarr prowlarr root-folder update`↴](#configuratarr-prowlarr-root-folder-update)
* [`configuratarr prowlarr quality-profile`↴](#configuratarr-prowlarr-quality-profile)
* [`configuratarr prowlarr quality-profile list`↴](#configuratarr-prowlarr-quality-profile-list)
* [`configuratarr prowlarr quality-profile delete`↴](#configuratarr-prowlarr-quality-profile-delete)
* [`configuratarr prowlarr quality-profile add`↴](#configuratarr-prowlarr-quality-profile-add)
* [`configuratarr prowlarr quality-profile update`↴](#configuratarr-prowlarr-quality-profile-update)
* [`configuratarr prowlarr custom-format`↴](#configuratarr-prowlarr-custom-format)
* [`configuratarr prowlarr custom-format list`↴](#configuratarr-prowlarr-custom-format-list)
* [`configuratarr prowlarr custom-format delete`↴](#configuratarr-prowlarr-custom-format-delete)
* [`configuratarr prowlarr custom-format add`↴](#configuratarr-prowlarr-custom-format-add)
* [`configuratarr prowlarr custom-format update`↴](#configuratarr-prowlarr-custom-format-update)
* [`configuratarr prowlarr metadata-profile`↴](#configuratarr-prowlarr-metadata-profile)
* [`configuratarr prowlarr metadata-profile list`↴](#configuratarr-prowlarr-metadata-profile-list)
* [`configuratarr prowlarr metadata-profile delete`↴](#configuratarr-prowlarr-metadata-profile-delete)
* [`configuratarr prowlarr metadata-profile add`↴](#configuratarr-prowlarr-metadata-profile-add)
* [`configuratarr prowlarr metadata-profile update`↴](#configuratarr-prowlarr-metadata-profile-update)
* [`configuratarr prowlarr release-profile`↴](#configuratarr-prowlarr-release-profile)
* [`configuratarr prowlarr release-profile list`↴](#configuratarr-prowlarr-release-profile-list)
* [`configuratarr prowlarr release-profile delete`↴](#configuratarr-prowlarr-release-profile-delete)
* [`configuratarr prowlarr release-profile add`↴](#configuratarr-prowlarr-release-profile-add)
* [`configuratarr prowlarr release-profile update`↴](#configuratarr-prowlarr-release-profile-update)
* [`configuratarr prowlarr ui`↴](#configuratarr-prowlarr-ui)
* [`configuratarr prowlarr ui show`↴](#configuratarr-prowlarr-ui-show)
* [`configuratarr prowlarr ui update`↴](#configuratarr-prowlarr-ui-update)
* [`configuratarr prowlarr naming`↴](#configuratarr-prowlarr-naming)
* [`configuratarr prowlarr naming show`↴](#configuratarr-prowlarr-naming-show)
* [`configuratarr prowlarr naming update`↴](#configuratarr-prowlarr-naming-update)
* [`configuratarr prowlarr media-management`↴](#configuratarr-prowlarr-media-management)
* [`configuratarr prowlarr media-management show`↴](#configuratarr-prowlarr-media-management-show)
* [`configuratarr prowlarr media-management update`↴](#configuratarr-prowlarr-media-management-update)
* [`configuratarr lidarr`↴](#configuratarr-lidarr)
* [`configuratarr lidarr sync`↴](#configuratarr-lidarr-sync)
* [`configuratarr lidarr status`↴](#configuratarr-lidarr-status)
* [`configuratarr lidarr download-client`↴](#configuratarr-lidarr-download-client)
* [`configuratarr lidarr download-client list`↴](#configuratarr-lidarr-download-client-list)
* [`configuratarr lidarr download-client delete`↴](#configuratarr-lidarr-download-client-delete)
* [`configuratarr lidarr download-client add`↴](#configuratarr-lidarr-download-client-add)
* [`configuratarr lidarr download-client update`↴](#configuratarr-lidarr-download-client-update)
* [`configuratarr lidarr indexer`↴](#configuratarr-lidarr-indexer)
* [`configuratarr lidarr indexer list`↴](#configuratarr-lidarr-indexer-list)
* [`configuratarr lidarr indexer delete`↴](#configuratarr-lidarr-indexer-delete)
* [`configuratarr lidarr indexer add`↴](#configuratarr-lidarr-indexer-add)
* [`configuratarr lidarr indexer update`↴](#configuratarr-lidarr-indexer-update)
* [`configuratarr lidarr root-folder`↴](#configuratarr-lidarr-root-folder)
* [`configuratarr lidarr root-folder list`↴](#configuratarr-lidarr-root-folder-list)
* [`configuratarr lidarr root-folder delete`↴](#configuratarr-lidarr-root-folder-delete)
* [`configuratarr lidarr root-folder add`↴](#configuratarr-lidarr-root-folder-add)
* [`configuratarr lidarr root-folder update`↴](#configuratarr-lidarr-root-folder-update)
* [`configuratarr lidarr quality-profile`↴](#configuratarr-lidarr-quality-profile)
* [`configuratarr lidarr quality-profile list`↴](#configuratarr-lidarr-quality-profile-list)
* [`configuratarr lidarr quality-profile delete`↴](#configuratarr-lidarr-quality-profile-delete)
* [`configuratarr lidarr quality-profile add`↴](#configuratarr-lidarr-quality-profile-add)
* [`configuratarr lidarr quality-profile update`↴](#configuratarr-lidarr-quality-profile-update)
* [`configuratarr lidarr custom-format`↴](#configuratarr-lidarr-custom-format)
* [`configuratarr lidarr custom-format list`↴](#configuratarr-lidarr-custom-format-list)
* [`configuratarr lidarr custom-format delete`↴](#configuratarr-lidarr-custom-format-delete)
* [`configuratarr lidarr custom-format add`↴](#configuratarr-lidarr-custom-format-add)
* [`configuratarr lidarr custom-format update`↴](#configuratarr-lidarr-custom-format-update)
* [`configuratarr lidarr metadata-profile`↴](#configuratarr-lidarr-metadata-profile)
* [`configuratarr lidarr metadata-profile list`↴](#configuratarr-lidarr-metadata-profile-list)
* [`configuratarr lidarr metadata-profile delete`↴](#configuratarr-lidarr-metadata-profile-delete)
* [`configuratarr lidarr metadata-profile add`↴](#configuratarr-lidarr-metadata-profile-add)
* [`configuratarr lidarr metadata-profile update`↴](#configuratarr-lidarr-metadata-profile-update)
* [`configuratarr lidarr release-profile`↴](#configuratarr-lidarr-release-profile)
* [`configuratarr lidarr release-profile list`↴](#configuratarr-lidarr-release-profile-list)
* [`configuratarr lidarr release-profile delete`↴](#configuratarr-lidarr-release-profile-delete)
* [`configuratarr lidarr release-profile add`↴](#configuratarr-lidarr-release-profile-add)
* [`configuratarr lidarr release-profile update`↴](#configuratarr-lidarr-release-profile-update)
* [`configuratarr lidarr ui`↴](#configuratarr-lidarr-ui)
* [`configuratarr lidarr ui show`↴](#configuratarr-lidarr-ui-show)
* [`configuratarr lidarr ui update`↴](#configuratarr-lidarr-ui-update)
* [`configuratarr lidarr naming`↴](#configuratarr-lidarr-naming)
* [`configuratarr lidarr naming show`↴](#configuratarr-lidarr-naming-show)
* [`configuratarr lidarr naming update`↴](#configuratarr-lidarr-naming-update)
* [`configuratarr lidarr media-management`↴](#configuratarr-lidarr-media-management)
* [`configuratarr lidarr media-management show`↴](#configuratarr-lidarr-media-management-show)
* [`configuratarr lidarr media-management update`↴](#configuratarr-lidarr-media-management-update)
* [`configuratarr readarr`↴](#configuratarr-readarr)
* [`configuratarr readarr sync`↴](#configuratarr-readarr-sync)
* [`configuratarr readarr status`↴](#configuratarr-readarr-status)
* [`configuratarr readarr download-client`↴](#configuratarr-readarr-download-client)
* [`configuratarr readarr download-client list`↴](#configuratarr-readarr-download-client-list)
* [`configuratarr readarr download-client delete`↴](#configuratarr-readarr-download-client-delete)
* [`configuratarr readarr download-client add`↴](#configuratarr-readarr-download-client-add)
* [`configuratarr readarr download-client update`↴](#configuratarr-readarr-download-client-update)
* [`configuratarr readarr indexer`↴](#configuratarr-readarr-indexer)
* [`configuratarr readarr indexer list`↴](#configuratarr-readarr-indexer-list)
* [`configuratarr readarr indexer delete`↴](#configuratarr-readarr-indexer-delete)
* [`configuratarr readarr indexer add`↴](#configuratarr-readarr-indexer-add)
* [`configuratarr readarr indexer update`↴](#configuratarr-readarr-indexer-update)
* [`configuratarr readarr root-folder`↴](#configuratarr-readarr-root-folder)
* [`configuratarr readarr root-folder list`↴](#configuratarr-readarr-root-folder-list)
* [`configuratarr readarr root-folder delete`↴](#configuratarr-readarr-root-folder-delete)
* [`configuratarr readarr root-folder add`↴](#configuratarr-readarr-root-folder-add)
* [`configuratarr readarr root-folder update`↴](#configuratarr-readarr-root-folder-update)
* [`configuratarr readarr quality-profile`↴](#configuratarr-readarr-quality-profile)
* [`configuratarr readarr quality-profile list`↴](#configuratarr-readarr-quality-profile-list)
* [`configuratarr readarr quality-profile delete`↴](#configuratarr-readarr-quality-profile-delete)
* [`configuratarr readarr quality-profile add`↴](#configuratarr-readarr-quality-profile-add)
* [`configuratarr readarr quality-profile update`↴](#configuratarr-readarr-quality-profile-update)
* [`configuratarr readarr custom-format`↴](#configuratarr-readarr-custom-format)
* [`configuratarr readarr custom-format list`↴](#configuratarr-readarr-custom-format-list)
* [`configuratarr readarr custom-format delete`↴](#configuratarr-readarr-custom-format-delete)
* [`configuratarr readarr custom-format add`↴](#configuratarr-readarr-custom-format-add)
* [`configuratarr readarr custom-format update`↴](#configuratarr-readarr-custom-format-update)
* [`configuratarr readarr metadata-profile`↴](#configuratarr-readarr-metadata-profile)
* [`configuratarr readarr metadata-profile list`↴](#configuratarr-readarr-metadata-profile-list)
* [`configuratarr readarr metadata-profile delete`↴](#configuratarr-readarr-metadata-profile-delete)
* [`configuratarr readarr metadata-profile add`↴](#configuratarr-readarr-metadata-profile-add)
* [`configuratarr readarr metadata-profile update`↴](#configuratarr-readarr-metadata-profile-update)
* [`configuratarr readarr release-profile`↴](#configuratarr-readarr-release-profile)
* [`configuratarr readarr release-profile list`↴](#configuratarr-readarr-release-profile-list)
* [`configuratarr readarr release-profile delete`↴](#configuratarr-readarr-release-profile-delete)
* [`configuratarr readarr release-profile add`↴](#configuratarr-readarr-release-profile-add)
* [`configuratarr readarr release-profile update`↴](#configuratarr-readarr-release-profile-update)
* [`configuratarr readarr ui`↴](#configuratarr-readarr-ui)
* [`configuratarr readarr ui show`↴](#configuratarr-readarr-ui-show)
* [`configuratarr readarr ui update`↴](#configuratarr-readarr-ui-update)
* [`configuratarr readarr naming`↴](#configuratarr-readarr-naming)
* [`configuratarr readarr naming show`↴](#configuratarr-readarr-naming-show)
* [`configuratarr readarr naming update`↴](#configuratarr-readarr-naming-update)
* [`configuratarr readarr media-management`↴](#configuratarr-readarr-media-management)
* [`configuratarr readarr media-management show`↴](#configuratarr-readarr-media-management-show)
* [`configuratarr readarr media-management update`↴](#configuratarr-readarr-media-management-update)

## `configuratarr`

Declarative configuration sync engine for the *arr stack

**Usage:** `configuratarr [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `sync` — Synchronize all configured applications from the configuration file
* `radarr` — Manage Radarr configuration
* `sonarr` — Manage Sonarr configuration
* `prowlarr` — Manage Prowlarr configuration
* `lidarr` — Manage Lidarr configuration
* `readarr` — Manage Readarr configuration

###### **Options:**

* `--url <URL>` — Starr application base URL override (e.g. http://127.0.0.1:7878)
* `--api-key <API_KEY>` — Starr application API key override
* `--wait` — Wait for the target application to become online before execution
* `--wait-timeout <WAIT_TIMEOUT>` — Timeout in seconds to wait for system status check to pass

  Default value: `30`



## `configuratarr sync`

Synchronize all configured applications from the configuration file

**Usage:** `configuratarr sync [OPTIONS] <--plan|--apply>`

###### **Options:**

* `--config <CONFIG>`

  Default value: `configuratarr.yaml`
* `--prune`
* `--plan` — Calculate diff and print dry-run
* `--apply` — Commit the changes to the server
* `--auto-approve`



## `configuratarr radarr`

Manage Radarr configuration

**Usage:** `configuratarr radarr <COMMAND>`

###### **Subcommands:**

* `sync` — Synchronize specific application settings from configuration file
* `status` — Check connection and online status
* `download-client` — Manage Download Clients
* `indexer` — Manage Indexers
* `root-folder` — Manage Root Folders
* `quality-profile` — Manage Quality Profiles
* `custom-format` — Manage Custom Formats
* `metadata-profile` — Manage Metadata Profiles
* `release-profile` — Manage Release Profiles
* `ui` — Manage UI Configuration
* `naming` — Manage Naming Configuration
* `media-management` — Manage Media Management Configuration



## `configuratarr radarr sync`

Synchronize specific application settings from configuration file

**Usage:** `configuratarr radarr sync [OPTIONS] <--plan|--apply>`

###### **Options:**

* `--config <CONFIG>`

  Default value: `configuratarr.yaml`
* `--prune`
* `--plan` — Calculate diff and print dry-run
* `--apply` — Commit the changes to the server
* `--auto-approve`



## `configuratarr radarr status`

Check connection and online status

**Usage:** `configuratarr radarr status`



## `configuratarr radarr download-client`

Manage Download Clients

**Usage:** `configuratarr radarr download-client <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr download-client list`

List all server-configured resources

**Usage:** `configuratarr radarr download-client list`



## `configuratarr radarr download-client delete`

Delete a resource by name

**Usage:** `configuratarr radarr download-client delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr download-client add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr download-client add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr download-client update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr download-client update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr indexer`

Manage Indexers

**Usage:** `configuratarr radarr indexer <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr indexer list`

List all server-configured resources

**Usage:** `configuratarr radarr indexer list`



## `configuratarr radarr indexer delete`

Delete a resource by name

**Usage:** `configuratarr radarr indexer delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr indexer add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr indexer add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr indexer update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr indexer update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr root-folder`

Manage Root Folders

**Usage:** `configuratarr radarr root-folder <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr root-folder list`

List all server-configured resources

**Usage:** `configuratarr radarr root-folder list`



## `configuratarr radarr root-folder delete`

Delete a resource by name

**Usage:** `configuratarr radarr root-folder delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr root-folder add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr root-folder add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr root-folder update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr root-folder update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr quality-profile`

Manage Quality Profiles

**Usage:** `configuratarr radarr quality-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr quality-profile list`

List all server-configured resources

**Usage:** `configuratarr radarr quality-profile list`



## `configuratarr radarr quality-profile delete`

Delete a resource by name

**Usage:** `configuratarr radarr quality-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr quality-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr quality-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr quality-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr quality-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr custom-format`

Manage Custom Formats

**Usage:** `configuratarr radarr custom-format <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr custom-format list`

List all server-configured resources

**Usage:** `configuratarr radarr custom-format list`



## `configuratarr radarr custom-format delete`

Delete a resource by name

**Usage:** `configuratarr radarr custom-format delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr custom-format add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr custom-format add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr custom-format update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr custom-format update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr metadata-profile`

Manage Metadata Profiles

**Usage:** `configuratarr radarr metadata-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr metadata-profile list`

List all server-configured resources

**Usage:** `configuratarr radarr metadata-profile list`



## `configuratarr radarr metadata-profile delete`

Delete a resource by name

**Usage:** `configuratarr radarr metadata-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr metadata-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr metadata-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr metadata-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr metadata-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr release-profile`

Manage Release Profiles

**Usage:** `configuratarr radarr release-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr radarr release-profile list`

List all server-configured resources

**Usage:** `configuratarr radarr release-profile list`



## `configuratarr radarr release-profile delete`

Delete a resource by name

**Usage:** `configuratarr radarr release-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr radarr release-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr radarr release-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr release-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr radarr release-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr radarr ui`

Manage UI Configuration

**Usage:** `configuratarr radarr ui <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr radarr ui show`

Show current configuration

**Usage:** `configuratarr radarr ui show`



## `configuratarr radarr ui update`

Update configuration values

**Usage:** `configuratarr radarr ui update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr radarr naming`

Manage Naming Configuration

**Usage:** `configuratarr radarr naming <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr radarr naming show`

Show current configuration

**Usage:** `configuratarr radarr naming show`



## `configuratarr radarr naming update`

Update configuration values

**Usage:** `configuratarr radarr naming update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr radarr media-management`

Manage Media Management Configuration

**Usage:** `configuratarr radarr media-management <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr radarr media-management show`

Show current configuration

**Usage:** `configuratarr radarr media-management show`



## `configuratarr radarr media-management update`

Update configuration values

**Usage:** `configuratarr radarr media-management update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr sonarr`

Manage Sonarr configuration

**Usage:** `configuratarr sonarr <COMMAND>`

###### **Subcommands:**

* `sync` — Synchronize specific application settings from configuration file
* `status` — Check connection and online status
* `download-client` — Manage Download Clients
* `indexer` — Manage Indexers
* `root-folder` — Manage Root Folders
* `quality-profile` — Manage Quality Profiles
* `custom-format` — Manage Custom Formats
* `metadata-profile` — Manage Metadata Profiles
* `release-profile` — Manage Release Profiles
* `ui` — Manage UI Configuration
* `naming` — Manage Naming Configuration
* `media-management` — Manage Media Management Configuration



## `configuratarr sonarr sync`

Synchronize specific application settings from configuration file

**Usage:** `configuratarr sonarr sync [OPTIONS] <--plan|--apply>`

###### **Options:**

* `--config <CONFIG>`

  Default value: `configuratarr.yaml`
* `--prune`
* `--plan` — Calculate diff and print dry-run
* `--apply` — Commit the changes to the server
* `--auto-approve`



## `configuratarr sonarr status`

Check connection and online status

**Usage:** `configuratarr sonarr status`



## `configuratarr sonarr download-client`

Manage Download Clients

**Usage:** `configuratarr sonarr download-client <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr download-client list`

List all server-configured resources

**Usage:** `configuratarr sonarr download-client list`



## `configuratarr sonarr download-client delete`

Delete a resource by name

**Usage:** `configuratarr sonarr download-client delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr download-client add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr download-client add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr download-client update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr download-client update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr indexer`

Manage Indexers

**Usage:** `configuratarr sonarr indexer <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr indexer list`

List all server-configured resources

**Usage:** `configuratarr sonarr indexer list`



## `configuratarr sonarr indexer delete`

Delete a resource by name

**Usage:** `configuratarr sonarr indexer delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr indexer add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr indexer add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr indexer update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr indexer update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr root-folder`

Manage Root Folders

**Usage:** `configuratarr sonarr root-folder <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr root-folder list`

List all server-configured resources

**Usage:** `configuratarr sonarr root-folder list`



## `configuratarr sonarr root-folder delete`

Delete a resource by name

**Usage:** `configuratarr sonarr root-folder delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr root-folder add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr root-folder add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr root-folder update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr root-folder update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr quality-profile`

Manage Quality Profiles

**Usage:** `configuratarr sonarr quality-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr quality-profile list`

List all server-configured resources

**Usage:** `configuratarr sonarr quality-profile list`



## `configuratarr sonarr quality-profile delete`

Delete a resource by name

**Usage:** `configuratarr sonarr quality-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr quality-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr quality-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr quality-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr quality-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr custom-format`

Manage Custom Formats

**Usage:** `configuratarr sonarr custom-format <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr custom-format list`

List all server-configured resources

**Usage:** `configuratarr sonarr custom-format list`



## `configuratarr sonarr custom-format delete`

Delete a resource by name

**Usage:** `configuratarr sonarr custom-format delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr custom-format add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr custom-format add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr custom-format update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr custom-format update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr metadata-profile`

Manage Metadata Profiles

**Usage:** `configuratarr sonarr metadata-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr metadata-profile list`

List all server-configured resources

**Usage:** `configuratarr sonarr metadata-profile list`



## `configuratarr sonarr metadata-profile delete`

Delete a resource by name

**Usage:** `configuratarr sonarr metadata-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr metadata-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr metadata-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr metadata-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr metadata-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr release-profile`

Manage Release Profiles

**Usage:** `configuratarr sonarr release-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr sonarr release-profile list`

List all server-configured resources

**Usage:** `configuratarr sonarr release-profile list`



## `configuratarr sonarr release-profile delete`

Delete a resource by name

**Usage:** `configuratarr sonarr release-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr sonarr release-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr sonarr release-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr release-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr sonarr release-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr sonarr ui`

Manage UI Configuration

**Usage:** `configuratarr sonarr ui <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr sonarr ui show`

Show current configuration

**Usage:** `configuratarr sonarr ui show`



## `configuratarr sonarr ui update`

Update configuration values

**Usage:** `configuratarr sonarr ui update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr sonarr naming`

Manage Naming Configuration

**Usage:** `configuratarr sonarr naming <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr sonarr naming show`

Show current configuration

**Usage:** `configuratarr sonarr naming show`



## `configuratarr sonarr naming update`

Update configuration values

**Usage:** `configuratarr sonarr naming update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr sonarr media-management`

Manage Media Management Configuration

**Usage:** `configuratarr sonarr media-management <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr sonarr media-management show`

Show current configuration

**Usage:** `configuratarr sonarr media-management show`



## `configuratarr sonarr media-management update`

Update configuration values

**Usage:** `configuratarr sonarr media-management update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr prowlarr`

Manage Prowlarr configuration

**Usage:** `configuratarr prowlarr <COMMAND>`

###### **Subcommands:**

* `sync` — Synchronize specific application settings from configuration file
* `status` — Check connection and online status
* `download-client` — Manage Download Clients
* `indexer` — Manage Indexers
* `root-folder` — Manage Root Folders
* `quality-profile` — Manage Quality Profiles
* `custom-format` — Manage Custom Formats
* `metadata-profile` — Manage Metadata Profiles
* `release-profile` — Manage Release Profiles
* `ui` — Manage UI Configuration
* `naming` — Manage Naming Configuration
* `media-management` — Manage Media Management Configuration



## `configuratarr prowlarr sync`

Synchronize specific application settings from configuration file

**Usage:** `configuratarr prowlarr sync [OPTIONS] <--plan|--apply>`

###### **Options:**

* `--config <CONFIG>`

  Default value: `configuratarr.yaml`
* `--prune`
* `--plan` — Calculate diff and print dry-run
* `--apply` — Commit the changes to the server
* `--auto-approve`



## `configuratarr prowlarr status`

Check connection and online status

**Usage:** `configuratarr prowlarr status`



## `configuratarr prowlarr download-client`

Manage Download Clients

**Usage:** `configuratarr prowlarr download-client <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr download-client list`

List all server-configured resources

**Usage:** `configuratarr prowlarr download-client list`



## `configuratarr prowlarr download-client delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr download-client delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr download-client add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr download-client add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr download-client update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr download-client update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr indexer`

Manage Indexers

**Usage:** `configuratarr prowlarr indexer <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr indexer list`

List all server-configured resources

**Usage:** `configuratarr prowlarr indexer list`



## `configuratarr prowlarr indexer delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr indexer delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr indexer add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr indexer add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr indexer update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr indexer update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr root-folder`

Manage Root Folders

**Usage:** `configuratarr prowlarr root-folder <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr root-folder list`

List all server-configured resources

**Usage:** `configuratarr prowlarr root-folder list`



## `configuratarr prowlarr root-folder delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr root-folder delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr root-folder add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr root-folder add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr root-folder update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr root-folder update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr quality-profile`

Manage Quality Profiles

**Usage:** `configuratarr prowlarr quality-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr quality-profile list`

List all server-configured resources

**Usage:** `configuratarr prowlarr quality-profile list`



## `configuratarr prowlarr quality-profile delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr quality-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr quality-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr quality-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr quality-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr quality-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr custom-format`

Manage Custom Formats

**Usage:** `configuratarr prowlarr custom-format <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr custom-format list`

List all server-configured resources

**Usage:** `configuratarr prowlarr custom-format list`



## `configuratarr prowlarr custom-format delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr custom-format delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr custom-format add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr custom-format add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr custom-format update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr custom-format update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr metadata-profile`

Manage Metadata Profiles

**Usage:** `configuratarr prowlarr metadata-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr metadata-profile list`

List all server-configured resources

**Usage:** `configuratarr prowlarr metadata-profile list`



## `configuratarr prowlarr metadata-profile delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr metadata-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr metadata-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr metadata-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr metadata-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr metadata-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr release-profile`

Manage Release Profiles

**Usage:** `configuratarr prowlarr release-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr prowlarr release-profile list`

List all server-configured resources

**Usage:** `configuratarr prowlarr release-profile list`



## `configuratarr prowlarr release-profile delete`

Delete a resource by name

**Usage:** `configuratarr prowlarr release-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr prowlarr release-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr prowlarr release-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr release-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr prowlarr release-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr prowlarr ui`

Manage UI Configuration

**Usage:** `configuratarr prowlarr ui <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr prowlarr ui show`

Show current configuration

**Usage:** `configuratarr prowlarr ui show`



## `configuratarr prowlarr ui update`

Update configuration values

**Usage:** `configuratarr prowlarr ui update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr prowlarr naming`

Manage Naming Configuration

**Usage:** `configuratarr prowlarr naming <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr prowlarr naming show`

Show current configuration

**Usage:** `configuratarr prowlarr naming show`



## `configuratarr prowlarr naming update`

Update configuration values

**Usage:** `configuratarr prowlarr naming update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr prowlarr media-management`

Manage Media Management Configuration

**Usage:** `configuratarr prowlarr media-management <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr prowlarr media-management show`

Show current configuration

**Usage:** `configuratarr prowlarr media-management show`



## `configuratarr prowlarr media-management update`

Update configuration values

**Usage:** `configuratarr prowlarr media-management update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr lidarr`

Manage Lidarr configuration

**Usage:** `configuratarr lidarr <COMMAND>`

###### **Subcommands:**

* `sync` — Synchronize specific application settings from configuration file
* `status` — Check connection and online status
* `download-client` — Manage Download Clients
* `indexer` — Manage Indexers
* `root-folder` — Manage Root Folders
* `quality-profile` — Manage Quality Profiles
* `custom-format` — Manage Custom Formats
* `metadata-profile` — Manage Metadata Profiles
* `release-profile` — Manage Release Profiles
* `ui` — Manage UI Configuration
* `naming` — Manage Naming Configuration
* `media-management` — Manage Media Management Configuration



## `configuratarr lidarr sync`

Synchronize specific application settings from configuration file

**Usage:** `configuratarr lidarr sync [OPTIONS] <--plan|--apply>`

###### **Options:**

* `--config <CONFIG>`

  Default value: `configuratarr.yaml`
* `--prune`
* `--plan` — Calculate diff and print dry-run
* `--apply` — Commit the changes to the server
* `--auto-approve`



## `configuratarr lidarr status`

Check connection and online status

**Usage:** `configuratarr lidarr status`



## `configuratarr lidarr download-client`

Manage Download Clients

**Usage:** `configuratarr lidarr download-client <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr download-client list`

List all server-configured resources

**Usage:** `configuratarr lidarr download-client list`



## `configuratarr lidarr download-client delete`

Delete a resource by name

**Usage:** `configuratarr lidarr download-client delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr download-client add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr download-client add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr download-client update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr download-client update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr indexer`

Manage Indexers

**Usage:** `configuratarr lidarr indexer <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr indexer list`

List all server-configured resources

**Usage:** `configuratarr lidarr indexer list`



## `configuratarr lidarr indexer delete`

Delete a resource by name

**Usage:** `configuratarr lidarr indexer delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr indexer add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr indexer add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr indexer update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr indexer update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr root-folder`

Manage Root Folders

**Usage:** `configuratarr lidarr root-folder <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr root-folder list`

List all server-configured resources

**Usage:** `configuratarr lidarr root-folder list`



## `configuratarr lidarr root-folder delete`

Delete a resource by name

**Usage:** `configuratarr lidarr root-folder delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr root-folder add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr root-folder add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr root-folder update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr root-folder update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr quality-profile`

Manage Quality Profiles

**Usage:** `configuratarr lidarr quality-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr quality-profile list`

List all server-configured resources

**Usage:** `configuratarr lidarr quality-profile list`



## `configuratarr lidarr quality-profile delete`

Delete a resource by name

**Usage:** `configuratarr lidarr quality-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr quality-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr quality-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr quality-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr quality-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr custom-format`

Manage Custom Formats

**Usage:** `configuratarr lidarr custom-format <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr custom-format list`

List all server-configured resources

**Usage:** `configuratarr lidarr custom-format list`



## `configuratarr lidarr custom-format delete`

Delete a resource by name

**Usage:** `configuratarr lidarr custom-format delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr custom-format add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr custom-format add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr custom-format update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr custom-format update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr metadata-profile`

Manage Metadata Profiles

**Usage:** `configuratarr lidarr metadata-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr metadata-profile list`

List all server-configured resources

**Usage:** `configuratarr lidarr metadata-profile list`



## `configuratarr lidarr metadata-profile delete`

Delete a resource by name

**Usage:** `configuratarr lidarr metadata-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr metadata-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr metadata-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr metadata-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr metadata-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr release-profile`

Manage Release Profiles

**Usage:** `configuratarr lidarr release-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr lidarr release-profile list`

List all server-configured resources

**Usage:** `configuratarr lidarr release-profile list`



## `configuratarr lidarr release-profile delete`

Delete a resource by name

**Usage:** `configuratarr lidarr release-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr lidarr release-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr lidarr release-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr release-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr lidarr release-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr lidarr ui`

Manage UI Configuration

**Usage:** `configuratarr lidarr ui <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr lidarr ui show`

Show current configuration

**Usage:** `configuratarr lidarr ui show`



## `configuratarr lidarr ui update`

Update configuration values

**Usage:** `configuratarr lidarr ui update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr lidarr naming`

Manage Naming Configuration

**Usage:** `configuratarr lidarr naming <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr lidarr naming show`

Show current configuration

**Usage:** `configuratarr lidarr naming show`



## `configuratarr lidarr naming update`

Update configuration values

**Usage:** `configuratarr lidarr naming update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr lidarr media-management`

Manage Media Management Configuration

**Usage:** `configuratarr lidarr media-management <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr lidarr media-management show`

Show current configuration

**Usage:** `configuratarr lidarr media-management show`



## `configuratarr lidarr media-management update`

Update configuration values

**Usage:** `configuratarr lidarr media-management update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr readarr`

Manage Readarr configuration

**Usage:** `configuratarr readarr <COMMAND>`

###### **Subcommands:**

* `sync` — Synchronize specific application settings from configuration file
* `status` — Check connection and online status
* `download-client` — Manage Download Clients
* `indexer` — Manage Indexers
* `root-folder` — Manage Root Folders
* `quality-profile` — Manage Quality Profiles
* `custom-format` — Manage Custom Formats
* `metadata-profile` — Manage Metadata Profiles
* `release-profile` — Manage Release Profiles
* `ui` — Manage UI Configuration
* `naming` — Manage Naming Configuration
* `media-management` — Manage Media Management Configuration



## `configuratarr readarr sync`

Synchronize specific application settings from configuration file

**Usage:** `configuratarr readarr sync [OPTIONS] <--plan|--apply>`

###### **Options:**

* `--config <CONFIG>`

  Default value: `configuratarr.yaml`
* `--prune`
* `--plan` — Calculate diff and print dry-run
* `--apply` — Commit the changes to the server
* `--auto-approve`



## `configuratarr readarr status`

Check connection and online status

**Usage:** `configuratarr readarr status`



## `configuratarr readarr download-client`

Manage Download Clients

**Usage:** `configuratarr readarr download-client <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr download-client list`

List all server-configured resources

**Usage:** `configuratarr readarr download-client list`



## `configuratarr readarr download-client delete`

Delete a resource by name

**Usage:** `configuratarr readarr download-client delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr download-client add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr download-client add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr download-client update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr download-client update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr indexer`

Manage Indexers

**Usage:** `configuratarr readarr indexer <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr indexer list`

List all server-configured resources

**Usage:** `configuratarr readarr indexer list`



## `configuratarr readarr indexer delete`

Delete a resource by name

**Usage:** `configuratarr readarr indexer delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr indexer add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr indexer add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr indexer update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr indexer update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr root-folder`

Manage Root Folders

**Usage:** `configuratarr readarr root-folder <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr root-folder list`

List all server-configured resources

**Usage:** `configuratarr readarr root-folder list`



## `configuratarr readarr root-folder delete`

Delete a resource by name

**Usage:** `configuratarr readarr root-folder delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr root-folder add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr root-folder add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr root-folder update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr root-folder update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr quality-profile`

Manage Quality Profiles

**Usage:** `configuratarr readarr quality-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr quality-profile list`

List all server-configured resources

**Usage:** `configuratarr readarr quality-profile list`



## `configuratarr readarr quality-profile delete`

Delete a resource by name

**Usage:** `configuratarr readarr quality-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr quality-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr quality-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr quality-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr quality-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr custom-format`

Manage Custom Formats

**Usage:** `configuratarr readarr custom-format <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr custom-format list`

List all server-configured resources

**Usage:** `configuratarr readarr custom-format list`



## `configuratarr readarr custom-format delete`

Delete a resource by name

**Usage:** `configuratarr readarr custom-format delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr custom-format add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr custom-format add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr custom-format update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr custom-format update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr metadata-profile`

Manage Metadata Profiles

**Usage:** `configuratarr readarr metadata-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr metadata-profile list`

List all server-configured resources

**Usage:** `configuratarr readarr metadata-profile list`



## `configuratarr readarr metadata-profile delete`

Delete a resource by name

**Usage:** `configuratarr readarr metadata-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr metadata-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr metadata-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr metadata-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr metadata-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr release-profile`

Manage Release Profiles

**Usage:** `configuratarr readarr release-profile <COMMAND>`

###### **Subcommands:**

* `list` — List all server-configured resources
* `delete` — Delete a resource by name
* `add` — Add a new resource configuration dynamically
* `update` — Update an existing resource configuration or add it if it does not exist



## `configuratarr readarr release-profile list`

List all server-configured resources

**Usage:** `configuratarr readarr release-profile list`



## `configuratarr readarr release-profile delete`

Delete a resource by name

**Usage:** `configuratarr readarr release-profile delete <NAME>`

###### **Arguments:**

* `<NAME>`



## `configuratarr readarr release-profile add`

Add a new resource configuration dynamically

**Usage:** `configuratarr readarr release-profile add [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr release-profile update`

Update an existing resource configuration or add it if it does not exist

**Usage:** `configuratarr readarr release-profile update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g., --field name=test --field enable=true).



## `configuratarr readarr ui`

Manage UI Configuration

**Usage:** `configuratarr readarr ui <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr readarr ui show`

Show current configuration

**Usage:** `configuratarr readarr ui show`



## `configuratarr readarr ui update`

Update configuration values

**Usage:** `configuratarr readarr ui update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr readarr naming`

Manage Naming Configuration

**Usage:** `configuratarr readarr naming <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr readarr naming show`

Show current configuration

**Usage:** `configuratarr readarr naming show`



## `configuratarr readarr naming update`

Update configuration values

**Usage:** `configuratarr readarr naming update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



## `configuratarr readarr media-management`

Manage Media Management Configuration

**Usage:** `configuratarr readarr media-management <COMMAND>`

###### **Subcommands:**

* `show` — Show current configuration
* `update` — Update configuration values



## `configuratarr readarr media-management show`

Show current configuration

**Usage:** `configuratarr readarr media-management show`



## `configuratarr readarr media-management update`

Update configuration values

**Usage:** `configuratarr readarr media-management update [OPTIONS]`

###### **Options:**

* `--field <KEY=VALUE>` — Key-value pair in KEY=value format. Can be specified multiple times (e.g. --field theme=light).



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
