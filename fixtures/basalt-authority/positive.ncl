# Safe synthetic Basalt authority adapter scenarios.
{
  scenarios = [
    "exact-agent-definition-to-audience-binding",
    "document-read-action-mapping",
    "document-write-action-mapping",
    "raw-holder-signature-verification",
    "injected-key-proof-revocation-caveat-replay-time-limits",
    "all-local-gates-intersect",
    "safe-decision-commit-before-effect",
    "live-authority-retained-through-effect",
    "metadata-only-allow-receipt",
    "replaceable-generic-provider-port",
  ],
  feature = "basalt-authority-adapter",
  default_enabled = false,
  live_services = false,
}
