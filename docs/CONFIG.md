# Configuration Reference (`configuratarr.yaml`)

This document defines the structure and parameters of the YAML configuration file used by **Configuratarr** to manage your `*arr` stack applications.

---

## 1. Secret Resolution & Variable Injection

Configuratarr supports dynamic secret injection at runtime. This allows you to keep API keys, passwords, and other credentials out of your configuration files and Git repositories.

If a string value starts with either `env://` or `file://`, Configuratarr resolves it dynamically:

*   **`env://<VAR_NAME>`**: Resolves the value of the environment variable `<VAR_NAME>`.
    *   *Example:* `username: "env://QB_USER"`
*   **`file://<PATH>`**: Reads the content of the file at the specified absolute `<PATH>` (and trims trailing whitespace).
    *   *Example:* `apiKey: "file:///run/secrets/radarr_api_key"`

### Automatic Type Coercion
Since environment variables and files are read as text, Configuratarr parses and coerces the resolved strings into their correct JSON/YAML types before executing the configuration synchronization:
*   `"true"` or `"false"` strings are parsed into booleans (`true` / `false`).
*   Whole numbers (e.g., `"7878"`) are parsed into integers (`7878`).
*   Decimal numbers are parsed into floats.
*   Other values remain strings.

---

## 2. Configuration Schema

The root configuration file supports application-specific namespaces: `radarr`, `sonarr`, `prowlarr`, `lidarr`, and `readarr`. Under each application namespace, the settings are divided into three areas:

1.  **Connection Settings (`host`)**:
    *   `url`: The base URL of the service instance.
    *   `apiKey`: The API key (optional; if omitted, cookie-based session login is performed).
2.  **Global Configuration (Singletons)**:
    *   `ui`: Visual settings, calendar start day, and date formats.
    *   `naming`: File and directory renaming formats.
    *   `mediaManagement`: File importing, recycling, permissions, and hardlink options.
3.  **Idempotent Lists (Resources)**:
    *   `downloadClients`, `indexers`, `notifications`, `qualityProfiles`, `customFormats`, `rootFolders`, `importLists`, `metadataProfiles`, `releaseProfiles`. Configuratarr compares these lists against the server state and performs additions, updates, or deletions to synchronize them.

---

## 3. Comprehensive Configuration Example

The following is a complete configuration example demonstrating settings across Radarr, Sonarr, Prowlarr, and Lidarr.

```yaml
# ==============================================================================
# Radarr Settings
# ==============================================================================
radarr:
  host:
    url: "http://radarr.caddy.server.internal"
    apiKey: "file:///run/secrets/radarr_api_key" # Dynamically loaded secret file

  # UI Settings (Singleton)
  ui:
    theme: "dark"
    timeFormat: "24h"
    shortDateFormat: "YYYY-MM-DD"
    showRelativeDates: true

  # Media Management Settings (Singleton)
  mediaManagement:
    renameMovies: true
    copyUsingHardlinks: true # Enabled to prevent disk space duplication
    recycleBin: "/data/recycle-bin"
    recycleBinCleanupDays: 7
    setPermissionsLinux: true
    chmodFolder: "0755"

  # Movie Renaming Naming Format (Singleton)
  naming:
    renameMovies: true
    standardMovieFormat: "{Movie Title} ({Release Year}) {Quality Title}"
    movieFolderFormat: "{Movie Title} ({Release Year})"

  # Download Clients (List)
  downloadClients:
    - name: "qBittorrent"
      enable: true
      protocol: "torrent"
      priority: 1
      implementation: "QBittorrent"
      removeCompletedDownloads: true # Automatically remove completed items from client
      removeFailedDownloads: true
      fields:
        - name: "Host"
          value: "qbittorrent.lan"
        - name: "Port"
          value: 8080
        - name: "Username"
          value: "env://QB_USER" # Resolved from environment variable
        - name: "Password"
          value: "file:///run/secrets/qb_pass" # Password value (sync comparison bypassed)
        - name: "MovieCategory"
          value: "radarr"

  # Indexers (List)
  indexers:
    - name: "Prowlarr Torznab Proxy"
      enableRss: true
      enableAutomaticSearch: true
      enableInteractiveSearch: true
      protocol: "torrent"
      priority: 25
      downloadClientId: 0 # 0 maps to any download client
      implementation: "Torznab"
      fields:
        - name: "BaseUrl"
          value: "http://prowlarr.lan/1"
        - name: "ApiKey"
          value: "file:///run/secrets/prowlarr_api_key"
        - name: "Categories"
          value: [2000, 5000]

  # Target Directories for Movies (List)
  rootFolders:
    - path: "/data/media/movies"

  # Quality Profiles (List)
  qualityProfiles:
    - name: "HD Upgrader"
      upgradeAllowed: true
      cutoff: 1 # Upgrade stops when Bluray-1080p is reached
      items:
        - name: "Bluray-1080p"
          allowed: true
        - name: "WEBDL-1080p"
          allowed: true
        - name: "SDTV"
          allowed: false

  # Custom Formats (List)
  customFormats:
    - name: "HDR10"
      includeCustomFormatWhenRenaming: false
      specifications:
        - name: "HDR10 Release Title"
          implementation: "ReleaseTitleSpecification"
          negate: false
          required: true
          fields:
            - name: "value"
              value: "\\b(HDR10|HDR)\\b"

  # Watchlist Import Lists (List)
  importLists:
    - name: "Trakt Popular Watchlist"
      enabled: true
      enableAuto: true
      monitor: "all"
      rootFolderPath: "/data/media/movies"
      qualityProfileId: 1
      searchOnAdd: false
      minimumAvailability: "announced"
      listType: "trakt"
      listOrder: 1
      minRefreshInterval: "24:00:00"
      implementation: "TraktList"
      fields:
        - name: "Username"
          value: "my_trakt_username"
        - name: "ListType"
          value: "popular"

  # System Notifications (List)
  notifications:
    - name: "Discord Notifications"
      implementation: "Discord"
      onGrab: true
      onDownload: true
      onUpgrade: true
      fields:
        - name: "WebHookUrl"
          value: "file:///run/secrets/discord_webhook"

# ==============================================================================
# Sonarr Settings
# ==============================================================================
sonarr:
  host:
    url: "http://sonarr.caddy.server.internal"
    apiKey: "file:///run/secrets/sonarr_api_key"

  ui:
    theme: "light"

  naming:
    renameEpisodes: true
    standardEpisodeFormat: "{Series Title} - S{season:00}E{episode:00} - {Episode Title} {Quality Title}"

  rootFolders:
    - path: "/data/media/tv"

  # Release Profiles (Sonarr v3 only)
  releaseProfiles:
    - name: "1080p Release Profile"
      enabled: true
      required: ["[1080p]"]
      ignored: ["[x264]"]
      fields: []

# ==============================================================================
# Prowlarr Settings
# ==============================================================================
prowlarr:
  host:
    url: "http://prowlarr.caddy.server.internal"
    apiKey: "file:///run/secrets/prowlarr_api_key"

  ui:
    theme: "dark"
    firstDayOfWeek: 1 # Calendar starts on Monday

# ==============================================================================
# Lidarr Settings
# ==============================================================================
lidarr:
  host:
    url: "http://lidarr.caddy.server.internal"
    # No apiKey: Uses session cookie authentication

  rootFolders:
    - path: "/data/media/music"

  # Metadata Profiles (Lidarr only)
  metadataProfiles:
    - name: "Standard Monitoring Profile"
      monitoringLevel: "standard"
      fields: []
```
