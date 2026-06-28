# Proposal: Evaluate Clankers provider contract parity for Animus

## Why

Clankers has provider/router parity rails that prevent request-shape drift, lost session metadata, lossy message conversion, and helper-only stream parser claims. Animus does not have real provider backends yet, but it should capture these as later adoption criteria for any provider abstraction beyond the current model adapter contract.

## What Changes

- Document provider contract parity as a future Animus adoption pattern.
- Define required shared request fields when provider/router abstractions exist.
- Require constructor inventory or serialization parity checks when multiple provider request DTOs coexist.
- Require provider-native message conversion tests rather than lossy generic JSON conversions.
- Require stream/SSE parser entrypoint tests when streaming providers are added.
- Require fail-closed behavior for known-but-unavailable provider prefixes.

## Impact

- **Files**: docs, future provider-contract tests/fixtures, optional rails, Cairn artifacts.
- **Testing**: future request-shape parity, constructor inventory, stream parser, and fail-closed routing tests.
- **Non-claims**: this does not add provider clients, router daemons, OAuth, credentials, network transport, or live provider checks.