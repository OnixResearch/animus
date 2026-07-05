# Evidence: adapt-clankers-remote-admission-receipts

## Implementation evidence

- Added `crates/animus-adapters/src/remote.rs` with typed remote/headless admission request DTOs, credential-evidence hashes, entrypoint/action enums, local caveat status, credential expiry/verifier status, admission decisions, safe admission receipts, BLAKE3 receipt identity, and receipt leak validation.
- Wired public adapter exports for `RemoteAdmissionRequest`, `RemoteCredentialEvidence`, `RemoteAdmissionDecision`, `RemoteAdmissionReceipt`, related enums/errors, `evaluate_remote_admission`, and `validate_remote_admission_receipt_text`.
- Added positive and negative tests for authorized prompt submission, missing credentials before side effects, wrong audience, expired credentials, revoked credentials, missing grants, caveat denial, verifier unavailable, missing policy, unsupported entrypoints, host-owned hash-only boundaries, and receipt leak fixtures.
- Added `docs/remote-admission-receipts.md` and README link documenting request shape, evaluation order, receipt safety, host-owned boundaries, and non-claims.

## Validation evidence

- Focused implementation check: `nix develop -c cargo fmt --check` — PASS.
- Focused remote-admission tests: `nix develop -c cargo test -p animus-adapters remote --locked` — PASS, 7 matching tests.
- Touched adapter suite: `nix develop -c cargo test -p animus-adapters --locked` — PASS, 260 tests.
- Runtime dependent suite: `nix develop -c cargo test -p animus-runtime --locked` — PASS, 41 tests.
