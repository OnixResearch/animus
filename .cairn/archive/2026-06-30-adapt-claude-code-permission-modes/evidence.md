# Evidence: adapt Claude Code permission modes

## Scope

- Change: `adapt-claude-code-permission-modes`
- Requirement IDs: `r[agent_harness_runtime.claude_code_permissions.*]`
- Non-claims: no Claude Code implementation is copied; local allow is a UX caveat and does not grant shell, network, filesystem, model, Steel, or tool authority beyond existing Animus authority checks.
- Worktree note: repository had substantial pre-existing staged/untracked files before this drain. This change did not commit because doing so would risk including user-owned staged work.

## Lifecycle gates

- Baseline `nix run .#cairn -- gate proposal adapt-claude-code-permission-modes --root .`: PASS.
- Baseline `nix run .#cairn -- gate design adapt-claude-code-permission-modes --root .`: PASS.
- Baseline `nix run .#cairn -- gate tasks adapt-claude-code-permission-modes --root .`: PASS.
- `nix run .#cairn -- validate --root .`: PASS (`valid: true`, `changes: 66`, `specs_validated: 67`).
- `scripts/cairn/check-roadmap --root .`: PASS; `adapt-claude-code-permission-modes` is unblocked and appears only as a prerequisite for dependent active changes.
- Closeout `gate proposal`, `gate design`, and `gate tasks`: PASS; task gate receipt `57ac4d68015ed4f38e41f2e2f1b2488d6f9cb393292f9335e77b222f05f53e1a`.
- `nix run .#cairn -- sync adapt-claude-code-permission-modes --root .`: PASS dry-run, `blocked: false`, one `sync_delta_spec` action.
- `nix run .#cairn -- sync adapt-claude-code-permission-modes --root . --execute`: completed, but accepted spec content did not change; `claude_code_permissions` IDs were absent. Requirement text was manually merged into `cairn/specs/agent-harness-runtime/spec.md` per repo Cairn workflow guidance.
- `nix run .#cairn -- validate --root .` after manual merge: PASS (`valid: true`, `changes: 66`, `specs_validated: 67`).
- Accepted spec verification: `claude_code_permissions` IDs are present under `cairn/specs/agent-harness-runtime/spec.md`.
- `CAIRN_ARCHIVE_DATE=<date> nix run .#cairn -- archive adapt-claude-code-permission-modes --root . --execute`: PASS; archived to `cairn/archive/2026-06-30-adapt-claude-code-permission-modes`.
- Roadmap closeout: active roadmap classification now references accepted `agent_harness_runtime.claude_code_permissions.scope` instead of the archived change ID; `scripts/cairn/check-roadmap --root .`: PASS.

## Focused implementation checks

- Baseline `nix develop -c cargo test -p animus-adapters effect::authority`: PASS; 17 authority tests passed before permission-mode implementation.
- `nix develop -c cargo test -p animus-adapters effect::permission`: PASS; 8 permission tests cover valid policies, ask pauses, approved/denied prompt tokens, stale token rejection, local allow without authority, safe receipts, malformed descriptors, untrusted descriptors, and receipt leak sentinels.
- `nix develop -c cargo test -p animus-adapters`: PASS; 139 unit tests and doc-tests passed.
- `nix develop -c cargo fmt --check`: PASS.
- `nix develop -c cargo check --workspace --all-targets`: PASS.
- `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings`: BLOCKED by pre-existing staged workspace receipt code: `crates/animus-adapters/src/workspace/receipt/mod.rs:203` `clippy::needless_pass_by_value` on `IdentityInput`. After preserving that user-owned staged file, the only Clippy diagnostic in `.pi/animus-adapters-clippy-permission-2.log` is this pre-existing blocker.

## Implemented coverage

- Typed permission policy descriptors validate action class, resource label, outcome, source identity, trust scope/status, precedence, and redacted prompt-summary policy.
- Permission resolution is pure over action, authority status, selected policy, prompt decision, session label, and redacted summary.
- Ask mode returns `PromptRequired` before dispatch and requires host-mediated scoped prompt tokens.
- Stale prompt tokens fail closed when action, resource, session, policy hash, or token do not match.
- Safe permission receipts store action, BLAKE3 resource/policy hashes, authority/local/prompt status, reason labels, and redaction markers while rejecting raw prompts, credentials, provider payloads, raw tool bodies, full paths, and unbounded outputs.
