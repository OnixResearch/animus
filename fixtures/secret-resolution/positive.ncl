let policy_value = (import "../../config/secret-resolution-policy.ncl").secret_resolution_policy in
{
  policy = policy_value,
  cases = [
    {name = "explicit-environment", profile = "environment-host", scope = "environment-provider", declaration = "ENV_API_KEY", provider = 'environment},
    {name = "explicit-systemd", profile = "systemd-host", scope = "systemd-provider", declaration = "SYSTEMD_API_KEY", provider = 'systemd_credential},
    {name = "bounded-sops", profile = "sops-host", scope = "sops-provider", declaration = "SOPS_API_KEY", provider = 'sops},
  ],
}
