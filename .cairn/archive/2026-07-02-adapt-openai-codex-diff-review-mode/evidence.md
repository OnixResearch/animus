# Evidence: adapt OpenAI Codex diff review mode

## Lifecycle gates

- Baseline `nix run .#cairn -- gate proposal adapt-openai-codex-diff-review-mode --root .`: PASS; receipt `da58808b909dcb335bad1f66e21d17332524852b884a2521f4456b4d19d7d433`.
- Baseline `nix run .#cairn -- gate design adapt-openai-codex-diff-review-mode --root .`: PASS; receipt `71ab1f9798b8c54a42d7f09c1d26d8805d3115839215fd5434fd62a3bba5245f`.
- Baseline `nix run .#cairn -- gate tasks adapt-openai-codex-diff-review-mode --root .`: PASS; receipt `3052e1d204ad92530a8f97e8788a46be4e893b1e53aaaf1395952674d52832c5`.
- Closeout `gate proposal`: PASS; receipt `57c3765e069a8b9f8be9466dbb0d5940c8eaa3c4dcd7fb1cf3abd5ae0cfbb783`.
- Closeout `gate design`: PASS; receipt `42570e5e39ffa476294421eab25796f42cfddb3011bdd5a048b9976e1889a263`.
- Closeout `gate tasks`: PASS; receipt `f534f2c4ffaad79f59865b9ea7e19ebfe8e65f0508507eff6b1df6045992455d`.
- `nix run .#cairn -- validate --root .`: PASS after accepted-spec merge.
- `scripts/cairn/check-roadmap --root .`: PASS; independent blocked roadmap entries remain reported for other active changes.

## Focused implementation checks

- Baseline before diff-review edits: `nix develop -c cargo test -p animus-adapters review --locked`: PASS, 3 existing review-filtered tests.
- After implementation: `nix develop -c cargo test -p animus-adapters diff_review --locked`: PASS, 8 dedicated diff-review tests.
- Broader adapter rail: `nix develop -c cargo test -p animus-adapters --locked`: PASS, 305 tests plus doc-tests.
- Formatting: `nix develop -c cargo fmt --all`: PASS.
- Clippy: `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: BLOCKED by pre-existing adapter lint debt outside `diff_review`; captured `.pi/drain/diff-review-clippy.log` contains no `diff_review` diagnostics.

## Implemented coverage

- `DiffReviewRequest::validate` accepts working-copy, commit, branch-comparison, and external-patch selectors only when the active adapter declares support and a bounded review target is present.
- `DiffReviewPolicyProfile::read_only` admits diff inspection, read-only file access, and model review while denying file edits, write-capable tools, approval escalation, commit creation, and pull-request management.
- `DiffReviewFinding`, `DiffReviewSpan`, `DiffReviewEvidence`, and `DiffReviewReport` validate structured findings, invalid spans, duplicate IDs, out-of-scope sources, missing evidence, unsafe raw material, full-file evidence, unbounded logs, and unrelated context.
- `compare_diff_review_rerun` classifies open, fixed, dismissed, and newly introduced findings from prior/current reports.
- `docs/openai-codex-diff-review-mode.md` documents the read-only review contract, evidence bounds, rerun state, and non-goals.

## Accepted spec verification

- `agent_harness_runtime.openai_codex_diff_review_mode.*` requirement IDs are present in `cairn/specs/agent-harness-runtime/spec.md` after manual merge because the sync execute plan did not insert the delta text automatically.
