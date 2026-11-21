## Why

[Animus #18](https://github.com/OnixResearch/animus/issues/18) asks for a bounded answer to a question that availability, declared capability, and serving readiness do not answer: what evidence supports choosing this exact agent configuration for a given task family? Evidence about one prompt, harness, model policy, tool surface, or environment must not silently qualify a changed profile. Lattice publishes `qualified` support only with a canonical `qualification_ref`, and nothing supplies one yet (`qualification_unbound`, owner `animus#18`). Lattice #25 consumes the attachment during explicitly admitted revision review and promotion.

## What Changes

- Add the pure `animus_runtime::qualification` module beside `eval_feedback`. It provides the `animus.qualification-subject.v1` subject, which carries the Lattice definition identity, the bare-hex Lattice profile revision, the task family, and every closed component as declared, observed, or unavailable. r[agent_harness_runtime.qualification_attachment.boundary] r[agent_harness_runtime.qualification_attachment.subject]
- Add the canonical `animus.qualification-attachment.v1` attachment. It holds four independent evidence classes, with exact source, protocol, corpus, and verifier identities, honest denominators, scope, limitations, missing modes, provenance, freshness, and bounded references. Its identity is `blake3:` over the canonical bytes. r[agent_harness_runtime.qualification_attachment.attachment] r[agent_harness_runtime.qualification_attachment.redaction]
- Add measures that keep unknown costs unknown, plus an equal-budget comparison over independently accepted completions only. r[agent_harness_runtime.qualification_attachment.measures]
- Extend evaluation-feedback freshness to the complete subject. Cross-revision reuse requires an explicit, scoped compatibility decision and never yields a qualified answer. r[agent_harness_runtime.qualification_attachment.applicability] r[agent_harness_runtime.qualification_attachment.authority]
- Add the pure consumer validator `validate_attachment(bytes, expected)`, set qualification, and replay. All return closed reasons that equal Lattice's `NotQualified` reasons. r[agent_harness_runtime.qualification_attachment.validator] r[agent_harness_runtime.qualification_attachment.replay]
- Make the campaign-manager evaluation lane produce one attachment per scripted route from its existing job report, and check in positive and negative fixtures under `fixtures/qualification-attachment/`. r[agent_harness_runtime.qualification_attachment.producer]

## Impact

- **Files**: `crates/animus-runtime/src/eval_feedback/qualification/`, `crates/animus-runtime/src/{lib.rs,eval_feedback/mod.rs}`, `crates/animus-campaign-manager-eval/src/{evidence/,lib.rs,matrix/mod.rs}`, `crates/animus-campaign-manager-eval/tests/{qualification.rs,attachment.rs,support/mod.rs}`, `fixtures/qualification-attachment/`, `docs/qualification-attachments.md`, `docs/ornith-eval-feedback-loop.md`, `docs/campaign-manager-eval.md`, `README.md`, `.cairn/roadmap.ncl`, `.cairn/runtime-facade-inventory.ncl`.
- **Testing**:
  - `cargo test -p animus-runtime --lib qualification`
  - `cargo test -p animus-campaign-manager-eval --test qualification --test attachment`
  - `cargo check -p animus-runtime --no-default-features --features steel`
  - workspace check and clippy
  - Cairn validation
- **Compatibility**: additive. No dependency is added, and no existing public type changes.
- **Non-goals**: no new evaluator, activation controller, live run, or deployment qualification. The NVIDIA skill-evidence follow-up in #18 remains open.
