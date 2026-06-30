# Evidence: Evaluate Clankers provider contract parity

## Source review

- PASS: reviewed sibling Clankers `docs/src/reference/provider-contracts.md` for shared request fields, constructor inventory, provider-native message conversion, parser-entrypoint stream coverage, and fail-closed provider prefixes.
- PASS: reviewed sibling Clankers `AGENTS.md` provider notes for `_session_id`, `extra_params`, request-shape drift, literal fixture, and raw SSE parser-entrypoint guidance.
- PASS: reviewed sibling Clankers provider/router boundary rail evidence for deterministic request-shape, provider-native JSON, and known-prefix fail-closed checks.

## Implementation

- PASS: added `docs/clankers-provider-contract-parity.md` as a future adoption matrix. The note keeps provider clients, router daemons, OAuth, credential lookup, network transport, streaming parser implementation, and live provider checks out of scope.

## Validation

- PASS: `nix run path:<repo>#cairn -- validate --root <repo>`.
- PASS: `TMPDIR=<repo>/.pi/tmp scripts/cairn/check-roadmap --root .`; known remaining blocker reports only.
- PASS: `TMPDIR=<repo>/.pi/tmp scripts/cairn/check-evidence-safety --root .`.
- PASS: `pre-commit run --all-files` after archive and roadmap cleanup.
