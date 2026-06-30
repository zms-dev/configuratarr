# nixosTest for prowlarr-v1. Called with the pre-built e2e test binary.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "prowlarr-v1-e2e";
  nodes.machine = {
    services.prowlarr.enable = true;
    environment.systemPackages = [ e2eBin ];
  };
  testScript = ''
    machine.wait_for_unit("prowlarr.service")
    machine.wait_for_open_port(9696, timeout=60)
    api_key = machine.wait_until_succeeds(
      "grep -oP '(?<=<ApiKey>)[^<]+' /var/lib/prowlarr/config.xml",
      timeout=30,
    ).strip()
    machine.succeed(
      f"PROWLARR_URL=http://localhost:9696 PROWLARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (global tag list,
      # prune deletes all); parallel runs race. Run serially.
      f"prowlarr-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
