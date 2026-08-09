# Evidence: Harbor ATIF-v1.7 JSON adapter

## Scope

This evidence covers the optional caller-byte ATIF adapter at Harbor `v0.18.0`, revision `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`, and `ATIF-v1.7`.

Source cohort identity: `b3:78d4a45481206483c89269eeb515854552bb4185b4db41a966047d8f124cfe4f`.

## Baseline before reusable changes

The following current checks passed before implementation:

- `scripts/cairn/check-harbor-evaluation --root .`
- `scripts/cairn/check-neutral-ledger --root .`
- focused runtime batch trajectory tests
- focused runtime subagent lineage tests
- focused adapter multimodal tests
- focused runtime dataset export tests
- `scripts/cairn/check-safe-receipts-artifacts --root .`
- reusable no-default checks
- `wasm32-unknown-unknown` checks

The Harbor rail initially reported a false dependency leak because `cargo tree` printed a worktree path containing `harbor`. The guard now checks normal dependency library names with `cargo tree --edges normal --format '{lib}'`. The same baseline then passed.

## Reviewed upstream cohort

The review used the exact Apache-2.0 Harbor source revision above. It covered:

- `rfcs/0001-trajectory-format.md`
- `src/harbor/models/trajectories`
- `src/harbor/utils/trajectory_validator.py`
- `tests/golden`

The inventory included root, agent, step, text/image content, tool call, observation, per-step and final metrics, copied-context, no-LLM dispatch, embedded and external subagent, continuation, and extension rules.

`session_id` is run-scoped and informational. Embedded resolution uses the document-scoped `trajectory_id`. ATIF StepObject has no terminal outcome field.

## Ownership result

`animus-adapter-atif` owns DTOs, policy validation, caller-byte JSON, semantic validation, caller-supplied graph checks, import/export drafts, explicit losses, identities, and receipt drafts.

Existing owners remain authoritative:

- the neutral ledger owns replay and persistence;
- batch owns order;
- subagent contracts own delegation lineage;
- multimodal contracts own media payload admission;
- dataset export owns training admission;
- Harbor evaluation owns trials, rewards, verifier facts, and outcomes;
- safe receipts and Cairn own their evidence decisions.

The adapter does not acquire or resolve files, paths, URLs, images, continuation documents, subagent documents, Harbor, Python, Pydantic, providers, tools, processes, networks, credentials, clocks, schedulers, or persistence.

## Implemented evidence

Production tests cover:

- checked generated Nickel policy revalidation;
- deterministic decode, encode, and typed semantic round trip;
- duplicate root and nested object keys;
- malformed, unknown, and oversized JSON;
- sequential steps and calendar-aware timestamps;
- source fields and no-LLM contradictions;
- tool correlation and metric relationships;
- multimodal unresolved descriptors;
- copied-context dataset exclusion;
- embedded `trajectory_id` lineage and repeated `session_id`;
- external subagent and continuation descriptors;
- bounded and unsafe extensions;
- caller-supplied DAG, missing-node, and cycle decisions;
- loss-visible import;
- export without invented facts;
- metadata-only receipts; and
- cross-crate Harbor migration without terminal inference.

Positive and negative Nickel fixture inventories are in `fixtures/harbor-atif/`. They retain the exact source revision, Apache-2.0 provenance, expected issue classes, and required non-claims.

## Deterministic checks

The focused rail is `scripts/cairn/check-harbor-atif-adapter --root .`. Its local receipt is `target/cairn/focused-rails/harbor-atif-adapter.md`.

The relevant Nix check is:

```sh
nix build .#checks.x86_64-linux.harbor-atif-adapter --no-link -L
```

`cargo tigerstyle check` reached the repository Octet check but stopped on 43 pre-existing findings in `animus-adapters/src/messaging.rs`. The reported findings were outside this change. The focused crate's strict Clippy, positive and negative tests, no-default build, and Wasm build are the bounded substitute evidence; they do not claim a passing workspace Tiger Style rail.

Final deterministic results:

- Focused adapter unit tests: 7 passed.
- Cross-crate Harbor projection tests: 2 passed.
- Focused runtime Harbor projection tests: 2 passed.
- Strict focused and workspace Clippy: PASS.
- Workspace all-target check: PASS.
- No-default and `wasm32-unknown-unknown` checks: PASS.
- Nickel profile export, drift, and Rust revalidation: PASS.
- Component catalog, runtime facade, evidence safety, Harbor evaluation, neutral ledger, batch, subagent, multimodal, dataset, and evaluation conformance checks: PASS.
- Complete focused rail: PASS.
- `checks.x86_64-linux.harbor-atif-adapter`: PASS after the configured remote builder was unavailable and Nix built locally.
- Cairn validation: PASS with six active changes and no issues.

Lifecycle evidence:

- Proposal gate receipt: `7d09c82344e309e291c48028038d04b99f1b77e67fd85a825e2184e5137e841d`.
- Design gate receipt: `a7f3203fd2586c34e6e7636f07653846ad5899a807ddcd5b259cc87d7def8f6f`.
- Tasks gate receipt before final archive review: `04ee28eaabbf1585a53c4f9ad2f3d038828a46defd7e1e4b5c8036fba7a38542`, with 24 of 25 tasks complete.
- Sync dry-run receipt: `52fc6515b98d6903d7790370d6aad4d99aa6692c687c826b0a0dfea93870fec0`.
- The sync plan adds all seven ATIF requirements and changes the accepted spec to `60ff89ae3f2664d2e2adef86b8752d94be707bbdc30b74e10e9df8e1bba8b3c1`.
- Initial archive dry-run receipt: `b0c327e00825817b35f0a013d5e30295bddf5da3e33c6d9f3798507d6a15abcc`.
- The initial archive review correctly blocked while final closeout task D4 remained open.

- Final tasks gate receipt: `ee12aff0878b8856eaa20184610e84b74cf4fa563d7685295113289eea36b51d`, with 25 of 25 tasks complete.
- Final sync dry-run receipt: `2cf50ee4f1fe92609cafabb8bafcc5d550339f4867ebee34e112ec880739d54f`.
- Final archive dry-run receipt: `b820be4f36bb3b5f9132b9b5af995afd2e23015efe0fe800171d8b68c1690b2f`.

- Accepted-spec sync execution receipt: `5ce24773d68969858be88f01b0bf7fd2615c6c80e4aa51bc84b2eb51406dedbc`.
- The accepted specification contains all seven new `agent_harness_runtime.harbor_atif.*` requirement IDs and has content identity `60ff89ae3f2664d2e2adef86b8752d94be707bbdc30b74e10e9df8e1bba8b3c1`.

- Archive execution receipt: `aef9eba9ba47575d82f4bf27d9a818db28d5ebd52e2f61192677906df2ba23f4`.
- Archive path: `cairn/archive/2026-08-08-add-harbor-atif-json-adapter/`.
- Archive mutation manifest: `c3ea10689a5de2a92149f69c61265468449a51630a31041d0db80745edcf4f6d`.
- Archived evidence identity at mutation: `89ad850c0ffc7ee36af00d205485e82120bc594b963f3cfbf3bbc4322d912a75`.

## Non-claims

This evidence does not establish future ATIF compatibility, arbitrary Harbor parity, byte-preserving input round trips, source authenticity, transcript completeness, timestamp or metric truth, external-resource safety, tool execution, hidden-reasoning safety, privacy, verifier soundness, artifact authenticity, benchmark validity, dataset or training fitness, authoritative completion, production readiness, or release eligibility.
