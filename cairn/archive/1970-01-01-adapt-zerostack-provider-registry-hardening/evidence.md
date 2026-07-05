# Evidence: adapt-zerostack-provider-registry-hardening

## Implementation evidence

- Added pure `animus-runtime::provider_hardening` DTOs and helpers for OpenAI-family API style resolution, redacted custom header source descriptors, provider/model-scoped extra body fields, normalized model class filtering, and safe provider selection receipts.
- Kept credential expansion, HTTP clients, provider config, environment access, and client construction host-owned.
- Re-exported the provider-hardening runtime surface from `animus-runtime`.
- Added focused positive and negative tests for first-party API style, custom gateway defaults, ambiguous API style, invalid header names, missing shell-resolved header sources, extra body provider/model scope, non-agent model rejection, and sensitive endpoint hashing.
- Documented the contract in `docs/zerostack-provider-registry-hardening.md` and linked it from `README.md`.
- Added provider-hardening public exports to `cairn/runtime-facade-inventory.ncl`; a provider-specific grep now only shows pre-existing adapter inventory drift.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime provider_hardening` passed: 5 focused tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 218 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `scripts/cairn/check-evidence-safety --root .` passed.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-zerostack-provider-registry-hardening --root .` passed.
- `nix run .#cairn -- gate design adapt-zerostack-provider-registry-hardening --root .` passed.
- `nix run .#cairn -- gate tasks adapt-zerostack-provider-registry-hardening --root .` passed.
- Scoped Clippy remains blocked by pre-existing runtime issues, but grep of `.pi/runtime-clippy.log` showed no provider-hardening findings after new-module fixes.

## Broader rail blockers

- Full workspace Clippy remains blocked by pre-existing adapter warnings-as-errors.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift for provider exports outside this runtime module.

## Requirement coverage

- `r[agent_harness_runtime.zerostack_provider_hardening.api_style]`
- `r[agent_harness_runtime.zerostack_provider_hardening.api_style.gateway]`
- `r[agent_harness_runtime.zerostack_provider_hardening.headers]`
- `r[agent_harness_runtime.zerostack_provider_hardening.headers.missing]`
- `r[agent_harness_runtime.zerostack_provider_hardening.extra_body]`
- `r[agent_harness_runtime.zerostack_provider_hardening.extra_body.scope]`
- `r[agent_harness_runtime.zerostack_provider_hardening.model_filter]`
- `r[agent_harness_runtime.zerostack_provider_hardening.model_filter.reject]`
- `r[agent_harness_runtime.zerostack_provider_hardening.receipts]`
- `r[agent_harness_runtime.zerostack_provider_hardening.receipts.endpoint]`
