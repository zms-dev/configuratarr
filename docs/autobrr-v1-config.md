# Autobrr v1 Configuration

Autobrr v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Api Key

`/api/keys` — an API key issued under a name, with a set of scopes.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Key name — its identity. |
| `scopes` | array of string | no |  | Access scopes granted to the key (e.g. `["read", "write"]`). |

### Notification

`/api/notification` — a notification target.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity. |
| `notification_type` | string | yes |  | Provider kind: `DISCORD`, `TELEGRAM`, `GOTIFY`, `NTFY`, … |
| `enabled` | boolean | yes |  | Whether the target is active. |
| `events` | array of [`notification_event`](#notification-event) | no |  | Events that trigger this notification. |
| `webhook` | string | no |  | Webhook URL (Discord and similar). |
| `api_key` | secret string | no |  | Provider API key / auth token where required. Credential — redacted in plan output. |
| `token` | secret string | no |  | Bot token (Telegram and similar). Credential — redacted in plan output. |
| `channel` | string | no |  | Target channel / chat id. |
| `topic` | string | no |  | Topic (ntfy and similar). |
| `host` | string | no |  | Provider host (self-hosted Gotify/ntfy). |

### Download Client

`/api/download_clients` — a download client autobrr pushes releases to.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.download_client.<name>}`). |
| `client_type` | string | yes |  | Client kind: `QBITTORRENT`, `DELUGE_V2`, `RADARR`, … |
| `enabled` | boolean | yes |  | Whether the client is active. |
| `host` | string | yes |  | Client host (URL or hostname). |
| `port` | integer | no |  | Client port. |
| `tls` | boolean | no |  | Connect over TLS. |
| `tls_skip_verify` | boolean | no |  | Skip TLS certificate verification. |
| `username` | string | no |  | Username, where the client authenticates by user/pass. |
| `password` | secret string | no |  | Password, where the client authenticates by user/pass. Credential — redacted in plan output. |
| `settings` | [`download_client_settings`](#download-client-settings) | no |  | Client-specific settings (auth, rules, delegation). |

### Filter

`/api/filters` — a release-matching filter.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Filter name — its identity. |
| `enabled` | boolean | no |  | Whether the filter is active. |
| `priority` | integer | no |  | Evaluation priority (higher wins). |
| `use_regex` | boolean | no |  | Treat match/except patterns as regular expressions. |
| `years` | string | no |  | Match release years (e.g. `2020-2024`). |
| `seasons` | string | no |  | Match seasons (e.g. `1,3-5`). |
| `episodes` | string | no |  | Match episodes. |
| `match_releases` | string | no |  | Releases must match these terms. |
| `except_releases` | string | no |  | Releases must not match these terms. |
| `match_release_groups` | string | no |  | Match these release groups. |
| `except_release_groups` | string | no |  | Exclude these release groups. |
| `match_categories` | string | no |  | Match these categories. |
| `except_categories` | string | no |  | Exclude these categories. |
| `tags` | string | no |  | Match these tags. |
| `except_tags` | string | no |  | Exclude these tags. |
| `tags_match_logic` | string | no |  | Logic for `tags` (`ANY` / `ALL`). |
| `except_tags_match_logic` | string | no |  | Logic for `except_tags` (`ANY` / `ALL`). |
| `smart_episode` | boolean | no |  | Smarter episode/season handling. |
| `resolutions` | array of string | no |  | Match these resolutions. |
| `codecs` | array of string | no |  | Match these video codecs. |
| `sources` | array of string | no |  | Match these sources. |
| `containers` | array of string | no |  | Match these containers. |
| `match_hdr` | array of string | no |  | Match these HDR formats. |
| `except_hdr` | array of string | no |  | Exclude these HDR formats. |
| `match_other` | array of string | no |  | Match these "other" tags (REPACK, PROPER, …). |
| `except_other` | array of string | no |  | Exclude these "other" tags. |
| `match_language` | array of string | no |  | Match these languages. |
| `except_language` | array of string | no |  | Exclude these languages. |
| `formats` | array of string | no |  | Match these audio formats. |
| `quality` | array of string | no |  | Match these audio qualities. |
| `media` | array of string | no |  | Match these media types. |
| `match_release_types` | array of string | no |  | Match these release types. |
| `origins` | array of string | no |  | Match these origins (INTERNAL, SCENE, …). |
| `except_origins` | array of string | no |  | Exclude these origins. |
| `min_size` | string | no |  | Minimum release size (e.g. `1GB`). |
| `max_size` | string | no |  | Maximum release size. |
| `delay` | integer | no |  | Delay before pushing, seconds. |
| `indexers` | array of [`filter_indexer`](#filter-indexer) | no |  | Indexers this filter is attached to. |
| `actions` | array of [`action`](#action) | no |  | Actions run on a matched release. |
| `external` | array of [`external_filter`](#external-filter) | no |  | External (webhook/exec) checks. |

## Types

### Notification Event

Allowed values: `PUSH_APPROVED` / `PUSH_REJECTED` / `PUSH_ERROR` / `IRC_DISCONNECTED` / `IRC_RECONNECTED` / `APP_UPDATE_AVAILABLE`.

### Download Client Settings

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `apikey` | secret string | no |  | API key (arr-style clients that authenticate by key rather than user/pass). Credential — redacted in plan output. |
| `basic` | [`download_client_basic`](#download-client-basic) | no |  | HTTP basic-auth credentials, if the client's endpoint is protected. |
| `rules` | [`download_client_rules`](#download-client-rules) | no |  | Throughput/queue rules applied before pushing releases. |
| `external_download_client_id` | integer | no |  | Id of another download client to delegate to (proxy setups). |

### Filter Indexer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server indexer id. |
| `name` | string | no |  | Indexer display name. |
| `identifier` | string | no |  | Indexer identifier (e.g. `torrentleech`). |

### Action

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned id. |
| `name` | string | no |  | Display name. |
| `action_type` | string | no |  | Action kind: `QBITTORRENT`, `DELUGE_V2`, `WEBHOOK`, `EXEC`, `TEST`, … |
| `enabled` | boolean | no |  | Whether the action is active. |
| `category` | string | no |  | Category to file the release under (client actions). |
| `tags` | string | no |  | Tags to apply (client actions). |
| `save_path` | string | no |  | Save path override (client actions). |
| `reannounce_interval` | integer | no |  | Reannounce interval, seconds. |
| `reannounce_max_attempts` | integer | no |  | Max reannounce attempts. |
| `client_id` | integer | no |  | Download client this action pushes to (`${ref.download_client.<name>}`). References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |
| `webhook_host` | string | no |  | Webhook URL (`WEBHOOK` type). |
| `webhook_method` | string | no |  | HTTP method for the webhook. |
| `webhook_type` | string | no |  | Webhook payload content type. |
| `webhook_data` | string | no |  | Request body sent to the webhook. |
| `webhook_headers` | array of string | no |  | Extra webhook headers (`Key: value`). |
| `exec_cmd` | string | no |  | Command to run (`EXEC` type). |
| `exec_args` | string | no |  | Arguments passed to the command. |
| `watch_folder` | string | no |  | Folder to watch for `.torrent` files (`WATCH_FOLDER` type). |

### External Filter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned id. |
| `name` | string | no |  | Display name. |
| `index` | integer | no |  | Evaluation order among external checks. |
| `external_type` | string | no |  | Check kind: `EXEC` or `WEBHOOK`. |
| `enabled` | boolean | no |  | Whether the check is active. |
| `webhook_host` | string | no |  | Webhook URL (`WEBHOOK` type). |
| `webhook_method` | string | no |  | HTTP method for the webhook. |
| `webhook_data` | string | no |  | Request body sent to the webhook. |
| `webhook_expect_status` | integer | no |  | HTTP status the webhook must return to pass. |
| `exec_cmd` | string | no |  | Command to run (`EXEC` type). |
| `exec_args` | string | no |  | Arguments passed to the command. |
| `exec_expect_status` | integer | no |  | Exit status the command must return to pass. |

### Download Client Basic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `auth` | boolean | no |  | Whether basic auth is required. |
| `username` | string | no |  | Basic-auth username. |
| `password` | secret string | no |  | Basic-auth password. Credential — redacted in plan output. |

### Download Client Rules

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Whether the rules are enforced. |
| `max_active_downloads` | integer | no |  | Cap on simultaneously active downloads (0 = unlimited). |
| `ignore_slow_torrents` | boolean | no |  | Skip pushing when existing torrents are slow. |
| `ignore_slow_torrents_condition` | string | no |  | When the slow-torrent check applies (`MAX_DOWNLOAD_SPEED` / `MAX_UPLOAD_SPEED`). |
| `download_speed_threshold` | integer | no |  | Download-speed threshold (KB/s) for the slow check. |
| `upload_speed_threshold` | integer | no |  | Upload-speed threshold (KB/s) for the slow check. |

