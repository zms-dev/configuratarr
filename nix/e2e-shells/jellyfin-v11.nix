# Local e2e dev shell for jellyfin-v11.
# Starts Jellyfin in a temp dir, runs the startup wizard, mints an API key,
# exports JELLYFIN_URL + JELLYFIN_API_KEY, and cleans up on exit.
#
# NOTE: the Authorization header form the engine must use is the open e2e risk
# for Jellyfin (see the crate's e2e.rs) — this shell mints a real API key; the
# engine sends it as a raw `Authorization` header value.
{ pkgs, e2eShell }:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (jellyfin-v11) ==="

    _JF_DATA=$(mktemp -d -t configuratarr-jellyfin-XXXXXX)
    _JF_AUTH='Authorization: MediaBrowser Client="cfg-e2e", Device="cfg-e2e", DeviceId="cfg-e2e", Version="1"'

    # The library e2e adds a library; Jellyfin rejects a non-existent path.
    mkdir -p /tmp/configuratarr-e2e-media

    echo "  starting Jellyfin..."
    jellyfin --datadir "$_JF_DATA/data" --cachedir "$_JF_DATA/cache" \
      --nowebclient > "$_JF_DATA/jellyfin.log" 2>&1 &
    _JF_PID=$!

    _jf_wait() {
      local i=0
      while [ $i -lt 60 ]; do
        curl -sf http://localhost:8096/System/Info/Public > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _jf_wait; then
      # Startup wizard — must run in order; POSTing the user before priming the
      # config/user GETs 404s.
      curl -sf http://localhost:8096/Startup/Configuration -H "$_JF_AUTH" > /dev/null
      curl -sf -X POST http://localhost:8096/Startup/Configuration \
        -H 'Content-Type: application/json' -H "$_JF_AUTH" \
        -d '{"UICulture":"en-US","MetadataCountryCode":"US","PreferredMetadataLanguage":"en"}' > /dev/null
      curl -sf http://localhost:8096/Startup/User -H "$_JF_AUTH" > /dev/null
      curl -sf -X POST http://localhost:8096/Startup/User \
        -H 'Content-Type: application/json' -H "$_JF_AUTH" \
        -d '{"Name":"admin","Password":"configuratarre2e"}' > /dev/null
      curl -sf -X POST http://localhost:8096/Startup/Complete -H "$_JF_AUTH" > /dev/null

      _JF_TOKEN=$(curl -sf -X POST http://localhost:8096/Users/AuthenticateByName \
        -H 'Content-Type: application/json' -H "$_JF_AUTH" \
        -d '{"Username":"admin","Pw":"configuratarre2e"}' | jq -r .AccessToken)

      curl -sf -X POST "http://localhost:8096/Auth/Keys?app=configuratarr" \
        -H "Authorization: MediaBrowser Token=\"$_JF_TOKEN\"" > /dev/null
      # `select` matches every `configuratarr` key, and Jellyfin mints a fresh one
      # each entry (plus the auth_key e2e creates more) — so this can emit several
      # lines. Take exactly one: a multi-line value becomes an invalid
      # (newline-bearing) HTTP header and fails the health check.
      _JF_KEY=$(curl -sf http://localhost:8096/Auth/Keys \
        -H "Authorization: MediaBrowser Token=\"$_JF_TOKEN\"" \
        | jq -r '.Items[] | select(.AppName=="configuratarr") | .AccessToken' \
        | head -n1)

      export JELLYFIN_URL="http://localhost:8096"
      export JELLYFIN_API_KEY="$_JF_KEY"

      # Gate handoff on the *authenticated* endpoint the engine actually uses
      # (raw key in `X-Emby-Token` against `/System/Info`, its declared health).
      # `/System/Info/Public` (the only thing `_jf_wait` checked) answers while
      # Jellyfin is still settling after the startup wizard, so the first e2e
      # apply could otherwise catch a post-wizard flap (transient 5xx). Require a
      # few consecutive OKs so a single lucky response doesn't count as ready.
      _jf_wait_ready() {
        local ok=0 i=0
        while [ $i -lt 60 ]; do
          if curl -sf -H "X-Emby-Token: $_JF_KEY" \
            http://localhost:8096/System/Info > /dev/null 2>&1; then
            ok=$((ok + 1)); [ "$ok" -ge 3 ] && return 0
          else
            ok=0
          fi
          sleep 1; i=$((i + 1))
        done
        return 1
      }

      if _jf_wait_ready; then
        echo "  Jellyfin ready — $JELLYFIN_URL"
        echo ""
        echo "  cargo nextest run -p jellyfin-v11 --run-ignored all -j1"
      else
        echo "  Jellyfin authenticated API never stabilised — check $_JF_DATA/jellyfin.log"
        kill "$_JF_PID" 2>/dev/null
      fi
    else
      echo "  Jellyfin failed to start — check $_JF_DATA/jellyfin.log"
      kill "$_JF_PID" 2>/dev/null
    fi

    _jf_cleanup() {
      kill "$_JF_PID" 2>/dev/null
      wait "$_JF_PID" 2>/dev/null
      rm -rf "$_JF_DATA"
    }
    trap _jf_cleanup EXIT
  '';
}
