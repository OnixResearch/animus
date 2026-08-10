# Design

## Goal and completion evidence

Index the existing real-check completion receipt as a separate bounded claim. Completion requires exact receipt metadata, deterministic positive and negative fixtures, generated-index validation, current-head binding, documentation, and archived Cairn evidence. A passing process, task checkbox, or supplied status alone is not completion.

## Portfolio search

The search used three serial, correlated lenses with a budget of three mechanisms, one targeted source pass, one adversarial pass, and focused validation.

1. **Extend the existing check row.** This keeps the format small, but overloads all checks with real-check-only fields and weakens compatibility. Rejected.
2. **Trust a supplied pass row.** This preserves the current generator, but cannot bind selected check ids, source identity, Cairn binding, mode, or receipt head. Falsified by stale and fabricated-pass cases.
3. **Add a dedicated receipt row.** This preserves existing rows and lets one claim require exact typed metadata. Selected.

## Functional core and shell

The Rust checker remains the pure validation core after file text is read. It parses a `real_check_receipt` row and evaluates exact constants, BLAKE3 shape, receipt/index head equality, and claim requirements.

The generator shell reads only the explicit `--real-check-receipt` path. Its parser accepts the exact metadata-only smoke receipt keys and emits one normalized row. It does not run the smoke or infer a release claim.

The smoke receipt adds exact selected check ids and an explicit Cairn binding. It continues to omit commands, output, subject data, credentials, and paths.

## Claim boundary

`bounded-real-check-completion` means that the exact reviewed quick-check receipt passed on the indexed clean head. It does not mean release readiness, production readiness, check sufficiency, check-oracle soundness, executable authenticity, sandboxing, hermeticity, or child correctness.

## Adversarial audit

Reject duplicate receipt rows, missing receipts, dirty index heads, dirty receipt heads, stale heads, fixture mode, non-pass status, unknown schema, wrong rail, malformed BLAKE3, different selected checks, wrong Cairn binding, conflicting claim/non-claim records, and a claim backed only by a generic pass row. Keep current claims backward compatible.
