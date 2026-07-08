# Evidence: adapt-openclaw-node-capability-hosts

## Implementation

- Added `animus_adapters::node` as a pure no-std DTO validation module for OpenClaw-inspired host-owned node capability descriptors.
- Added positive and negative fixtures covering node advertisements, gateway-owned claim rejection, pairing metadata, metadata drift, unsupported commands, permission mismatch, injected service availability, idempotency, canonical exec plans, per-node approval evidence, safe results, receipt leak rejection, and ambient execution fallback denial.
- Added docs in `docs/openclaw-node-capability-hosts.md`, README discovery text, and the Nickel-owned runtime facade inventory entry.

## Checks

- Baseline before edits: `cargo test -p animus-adapters gateway` — passed, 23 selected tests.
- Baseline before edits: `scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- Baseline before edits: `nix run path:$PWD#cairn -- validate --root .` — passed.
- `cargo fmt` — passed.
- `cargo test -p animus-adapters node` — passed, 3 selected tests.
- `cargo test -p animus-adapters` — passed, 480 tests.
- `cargo check --workspace --all-targets` — passed.
- `scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- `nix run path:$PWD#cairn -- validate --root .` — passed.

## Non-claims

No iOS, Android, macOS, WebSocket node, camera, screen recording, location, SMS, device pairing store, approval persistence, platform API, remote command execution, node daemon, socket transport, or production peripheral integration is implemented or claimed.
