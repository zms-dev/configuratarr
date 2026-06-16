# NixOS Module Options

This document details the configuration options available for the Configuratarr NixOS module.

## services.configuratarr.enable

Whether to enable Configuratarr declarative configuration service.



*Type:*
boolean



*Default:*

```nix
false
```



*Example:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.package



The configuratarr package to use.



*Type:*
package



*Default:*

```nix
<derivation configuratarr-0.1.0>
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.prune



Whether to prune server-side resources that are not declared in the config file.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings



Declarative configuration options for configuratarr.



*Type:*
submodule



*Default:*

```nix
{ }
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr



Configuration for Lidarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.customFormats



Custom formats configuration for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.customFormats.\*.name



Name of the custom format.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients



Download clients for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.enable



Whether to enable the download client.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.implementation



Class name of download client implementation (e.g. QBittorrent, SABnzbd).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.name



Name of the download client.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.priority



Priority order.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
1
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.protocol



Download protocol (usenet or torrent).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.removeCompletedDownloads



Whether to automatically remove completed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.downloadClients.\*.removeFailedDownloads



Whether to automatically remove failed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.host



Host connection configuration for Lidarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.host.apiKey



The API key for authentication.



*Type:*
null or string



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.host.url



The base URL of the service instance.



*Type:*
string



*Example:*

```nix
"http://localhost:8083"
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists



Import lists configuration for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists.\*.enabled



Enable list syncing.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists.\*.implementation



Import list implementation type.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.importLists.\*.name



Name of the import list.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers



Indexers for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.enableAutomaticSearch



Enable automatic search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.enableInteractiveSearch



Enable interactive manual search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.enableRss



Enable RSS feeds parsing.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.downloadClientId



Restrict indexer to specific download client ID (0 for any).



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
0
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.implementation



Indexer implementation class (e.g. Torznab, Newznab).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.name



Name of the indexer.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.priority



Search order priority.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
25
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.indexers.\*.protocol



Indexer protocol (torrent or usenet).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.mediaManagement



Media management configuration for Lidarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.metadataProfiles



Metadata profiles configuration for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.metadataProfiles.\*.name



Name of the metadata profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.naming



Naming configuration for Lidarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.notifications



Notification connections for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.notifications.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.notifications.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.notifications.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.notifications.\*.implementation



Notification service type (e.g. Discord, Telegram, Webhook).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.notifications.\*.name



Name of the notification connection.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.qualityProfiles



Quality profiles configuration for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.qualityProfiles.\*.cutoff



Quality ID that stops the upgrade cycle.



*Type:*
unsigned integer, meaning >=0

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.qualityProfiles.\*.name



Name of the quality profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.qualityProfiles.\*.upgradeAllowed



Allow upgrading media files over time.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.rootFolders



Root folders configuration for Lidarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.rootFolders.\*.path



Absolute directory path.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.lidarr.ui



UI configuration for Lidarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr



Configuration for Prowlarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients



Download clients for Prowlarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.enable



Whether to enable the download client.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.implementation



Class name of download client implementation (e.g. QBittorrent, SABnzbd).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.name



Name of the download client.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.priority



Priority order.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
1
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.protocol



Download protocol (usenet or torrent).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.removeCompletedDownloads



Whether to automatically remove completed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.downloadClients.\*.removeFailedDownloads



Whether to automatically remove failed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.host



Host connection configuration for Prowlarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.host.apiKey



The API key for authentication.



*Type:*
null or string



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.host.url



The base URL of the service instance.



*Type:*
string



*Example:*

```nix
"http://localhost:8083"
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers



Indexers for Prowlarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.enableAutomaticSearch



Enable automatic search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.enableInteractiveSearch



Enable interactive manual search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.enableRss



Enable RSS feeds parsing.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.downloadClientId



Restrict indexer to specific download client ID (0 for any).



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
0
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.implementation



Indexer implementation class (e.g. Torznab, Newznab).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.name



Name of the indexer.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.priority



Search order priority.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
25
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.indexers.\*.protocol



Indexer protocol (torrent or usenet).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.notifications



Notification connections for Prowlarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.notifications.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.notifications.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.notifications.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.notifications.\*.implementation



Notification service type (e.g. Discord, Telegram, Webhook).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.notifications.\*.name



Name of the notification connection.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.prowlarr.ui



UI configuration for Prowlarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr



Configuration for Radarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.customFormats



Custom formats configuration for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.customFormats.\*.name



Name of the custom format.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients



Download clients for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.enable



Whether to enable the download client.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.implementation



Class name of download client implementation (e.g. QBittorrent, SABnzbd).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.name

Name of the download client.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.priority



Priority order.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
1
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.protocol



Download protocol (usenet or torrent).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.removeCompletedDownloads



Whether to automatically remove completed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.downloadClients.\*.removeFailedDownloads



Whether to automatically remove failed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.host



Host connection configuration for Radarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.host.apiKey



The API key for authentication.



*Type:*
null or string



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.host.url



The base URL of the service instance.



*Type:*
string



*Example:*

```nix
"http://localhost:8083"
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists



Import lists configuration for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists.\*.enabled



Enable list syncing.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists.\*.implementation



Import list implementation type.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.importLists.\*.name



Name of the import list.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers



Indexers for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.enableAutomaticSearch



Enable automatic search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.enableInteractiveSearch



Enable interactive manual search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.enableRss



Enable RSS feeds parsing.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.downloadClientId



Restrict indexer to specific download client ID (0 for any).



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
0
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.implementation



Indexer implementation class (e.g. Torznab, Newznab).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.name



Name of the indexer.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.priority



Search order priority.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
25
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.indexers.\*.protocol



Indexer protocol (torrent or usenet).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.mediaManagement



Media management configuration for Radarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.naming



Naming configuration for Radarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.notifications



Notification connections for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.notifications.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.notifications.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.notifications.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.notifications.\*.implementation



Notification service type (e.g. Discord, Telegram, Webhook).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.notifications.\*.name



Name of the notification connection.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.qualityProfiles



Quality profiles configuration for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.qualityProfiles.\*.cutoff



Quality ID that stops the upgrade cycle.



*Type:*
unsigned integer, meaning >=0

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.qualityProfiles.\*.name



Name of the quality profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.qualityProfiles.\*.upgradeAllowed



Allow upgrading media files over time.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.rootFolders



Root folders configuration for Radarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.rootFolders.\*.path



Absolute directory path.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.radarr.ui



UI configuration for Radarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr



Configuration for Readarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients



Download clients for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.enable



Whether to enable the download client.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.implementation



Class name of download client implementation (e.g. QBittorrent, SABnzbd).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.name



Name of the download client.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.priority



Priority order.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
1
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.protocol



Download protocol (usenet or torrent).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.removeCompletedDownloads



Whether to automatically remove completed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.downloadClients.\*.removeFailedDownloads



Whether to automatically remove failed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.host



Host connection configuration for Readarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.host.apiKey



The API key for authentication.



*Type:*
null or string



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.host.url



The base URL of the service instance.



*Type:*
string



*Example:*

```nix
"http://localhost:8083"
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists



Import lists configuration for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists.\*.enabled



Enable list syncing.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists.\*.implementation



Import list implementation type.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.importLists.\*.name



Name of the import list.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers



Indexers for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.enableAutomaticSearch



Enable automatic search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.enableInteractiveSearch



Enable interactive manual search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.enableRss



Enable RSS feeds parsing.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.downloadClientId



Restrict indexer to specific download client ID (0 for any).



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
0
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.implementation



Indexer implementation class (e.g. Torznab, Newznab).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.name



Name of the indexer.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.priority



Search order priority.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
25
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.indexers.\*.protocol



Indexer protocol (torrent or usenet).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.mediaManagement



Media management configuration for Readarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.metadataProfiles



Metadata profiles configuration for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.metadataProfiles.\*.name



Name of the metadata profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.naming



Naming configuration for Readarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.notifications



Notification connections for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.notifications.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.notifications.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.notifications.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.notifications.\*.implementation



Notification service type (e.g. Discord, Telegram, Webhook).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.notifications.\*.name



Name of the notification connection.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.qualityProfiles



Quality profiles configuration for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.qualityProfiles.\*.cutoff



Quality ID that stops the upgrade cycle.



*Type:*
unsigned integer, meaning >=0

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.qualityProfiles.\*.name



Name of the quality profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.qualityProfiles.\*.upgradeAllowed



Allow upgrading media files over time.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.rootFolders



Root folders configuration for Readarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.rootFolders.\*.path



Absolute directory path.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.readarr.ui



UI configuration for Readarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr



Configuration for Sonarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.customFormats



Custom formats configuration for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.customFormats.\*.name



Name of the custom format.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients



Download clients for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.enable



Whether to enable the download client.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.fields.\*.value

The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.implementation



Class name of download client implementation (e.g. QBittorrent, SABnzbd).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.name



Name of the download client.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.priority



Priority order.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
1
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.protocol



Download protocol (usenet or torrent).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.removeCompletedDownloads



Whether to automatically remove completed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.downloadClients.\*.removeFailedDownloads



Whether to automatically remove failed downloads.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.host



Host connection configuration for Sonarr.



*Type:*
null or (submodule)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.host.apiKey



The API key for authentication.



*Type:*
null or string



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.host.url



The base URL of the service instance.



*Type:*
string



*Example:*

```nix
"http://localhost:8083"
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists



Import lists configuration for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists.\*.enabled



Enable list syncing.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists.\*.implementation



Import list implementation type.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.importLists.\*.name



Name of the import list.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers



Indexers for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.enableAutomaticSearch



Enable automatic search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.enableInteractiveSearch



Enable interactive manual search queries.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.enableRss



Enable RSS feeds parsing.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.downloadClientId



Restrict indexer to specific download client ID (0 for any).



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
0
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.implementation



Indexer implementation class (e.g. Torznab, Newznab).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.name



Name of the indexer.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.priority



Search order priority.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
25
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.indexers.\*.protocol



Indexer protocol (torrent or usenet).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.mediaManagement



Media management configuration for Sonarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.naming



Naming configuration for Sonarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.notifications



Notification connections for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.notifications.\*.fields



Implementation-specific configuration fields.



*Type:*
list of (submodule)



*Default:*

```nix
[ ]
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.notifications.\*.fields.\*.name



The name of the configuration field.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.notifications.\*.fields.\*.value



The value of the configuration field.



*Type:*
anything

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.notifications.\*.implementation



Notification service type (e.g. Discord, Telegram, Webhook).



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.notifications.\*.name



Name of the notification connection.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.qualityProfiles



Quality profiles configuration for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.qualityProfiles.\*.cutoff



Quality ID that stops the upgrade cycle.



*Type:*
unsigned integer, meaning >=0

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.qualityProfiles.\*.name



Name of the quality profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.qualityProfiles.\*.upgradeAllowed



Allow upgrading media files over time.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.releaseProfiles



Release profiles configuration for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.releaseProfiles.\*.enabled



Whether the release profile is enabled.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.releaseProfiles.\*.name



Name of the release profile.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.rootFolders



Root folders configuration for Sonarr.



*Type:*
null or (list of (open submodule of attribute set of anything))



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.rootFolders.\*.path



Absolute directory path.



*Type:*
string

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.settings.sonarr.ui



UI configuration for Sonarr.



*Type:*
null or (attribute set)



*Default:*

```nix
null
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.wait



Wait for target applications to be online before running synchronization.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)



## services.configuratarr.waitTimeout



Timeout in seconds to wait for each application to become online.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
30
```

*Declared by:*
 - [../modules/nixos.nix](../modules/nixos.nix)


