# Evidence

## Baseline

- `scripts/cairn/check-dogfood-readiness --root .` passed before the core change.
- `scripts/cairn/check-release-evidence-index --root .` passed before the core change.
- `scripts/cairn/check-current-head-release-evidence-index --root .` passed before the core change.

## Focused validation

- `rustfmt --check scripts/cairn/check-release-evidence-index.rs scripts/cairn/generate-current-head-release-evidence-index.rs` passed.
- `scripts/cairn/check-dogfood-readiness --root .` passed the claim-hygiene fixtures and the composed release-index fixtures.
- The positive fixture accepted one clean, passing, head-matched packaged-smoke receipt.
- Negative fixtures rejected absent, declared-missing, stale, duplicate, conflicting, and dirty evidence.
- `scripts/cairn/check-current-head-release-evidence-index --root .` generated and accepted the bounded packaged-dogfood claim.
- `scripts/cairn/check-roadmap --root .` passed with one classified active change and no product-safety path errors.

## Nix evidence

- `nix build .#checks.x86_64-linux.dogfood-readiness --no-link -L` passed with all positive and negative fixtures.
- `nix build .#checks.x86_64-linux.release-evidence-index --no-link -L` passed with all positive and negative fixtures.
- `nix build .#checks.x86_64-linux.current-head-release-evidence-index --no-link -L` passed and generated the packaged-dogfood claim.

## Lifecycle evidence

- Cairn validation passed before and after accepted-spec sync.
- Proposal, design, and tasks gates passed.
- The sync preflight applied four added requirements without blockers.
- Accepted requirement IDs appear in `cairn/specs/agent-harness-runtime/spec.md`.

## Adversarial audit

The audit found that a passing receipt and a matching `missing_mode` record could coexist. The checker now rejects that conflict, and a negative fixture protects the branch. Duplicate check records and conflicting claim status also fail with deterministic issue codes.

## Non-claims

This evidence does not establish live-provider, TUI, daemon, plugin, production, release-tagging, or Clankers-parity readiness. The claim covers only the packaged headless dogfood smoke receipt for the recorded clean head.
