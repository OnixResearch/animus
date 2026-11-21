## Phase 1: Pure core

- [x] [serial] 1.1 Add the pure `animus_runtime::qualification` module with the `animus.qualification-subject.v1` subject: the Lattice definition identity, the bare-hex profile revision, the task family, the 25 closed components with declared, observed or unavailable basis, and the hosted, local or scripted model facts. Prove it builds under `--no-default-features --features steel`. r[agent_harness_runtime.qualification_attachment.boundary] r[agent_harness_runtime.qualification_attachment.subject]
- [x] [serial] 1.2 Add the canonical JSON-subset codec, `blake3:` attachment identities, the per-class shape rules, and token-or-identity redaction. Cover malformed, non-canonical, unknown-field, authority-field, prefixed-revision, path, prompt and credential cases. r[agent_harness_runtime.qualification_attachment.attachment] r[agent_harness_runtime.qualification_attachment.redaction]
- [x] [serial] 1.3 Add honest denominators, unknown costs, cost per accepted completion and the equal-budget comparison, with positive and negative tests. r[agent_harness_runtime.qualification_attachment.measures]
- [x] [serial] 1.4 Add complete-subject applicability, scoped compatibility decisions and review authority. Cover tests for changed components, changed revisions, expiry, evaluator waivers and no-activation. r[agent_harness_runtime.qualification_attachment.applicability] r[agent_harness_runtime.qualification_attachment.authority]
- [x] [serial] 1.5 Add `validate_attachment`, `qualify` and `replay_attachments` with closed reasons. Cover cross-profile, scope, class, stale, unusable, duplicate and conflicting tests. r[agent_harness_runtime.qualification_attachment.validator] r[agent_harness_runtime.qualification_attachment.replay]

## Phase 2: Producer and documentation

- [x] [serial] 2.1 Make the campaign-manager lane produce attachments from its finished job report. Check in the positive and negative fixtures under `fixtures/qualification-attachment/`, and add tests for byte-exact production, replay without rerunning, conflicts and the equal-budget comparison. r[agent_harness_runtime.qualification_attachment.producer]
- [x] [serial] 2.2 Add `docs/qualification-attachments.md`, link it from the evaluation-feedback docs, the campaign-manager-eval docs and the README, classify the change in `.cairn/roadmap.ncl`, and register the public module in the runtime facade inventory. r[agent_harness_runtime.qualification_attachment.boundary]

## Phase 3: Gates and remaining work

- [x] [serial] 3.1 Run the Animus gates and record the results in `evidence/fixture-qualification.md`. The only failure is the accepted pre-existing roadmap issue for `add-acp-coding-agent`. r[agent_harness_runtime.qualification_attachment.validator]
- [ ] [serial] 3.2 Compose the NVIDIA skill-evidence follow-up from #18 into the attachment: skill-set manifests, scanner coverage, activation, and paired or incomplete arms. Not performed; out of scope for this change. r[agent_harness_runtime.qualification_attachment.attachment]
- [ ] [serial] 3.3 Record a live task-family efficacy experiment separately, labelled `live`. Not performed; it needs a separately scheduled live run. r[agent_harness_runtime.qualification_attachment.producer]
