# Verification evidence

## Baseline

- Before implementation, focused tests for agent definitions, UCAN capability authorization, typed subagent capability resolution, worker-team delegation, review reports, named verifier evidence, judge/veto policy, per-model runtime profiles, native tool admission, lineage, and lifecycle validation passed.
- The baseline established that the existing authority and evidence contracts were healthy. It did not establish capability-bound semantic role assignment, role/model composition, separation of duty, role transitions, or role output authority, which did not yet exist.

## Implemented boundary

- `config/agent-role-profiles.ncl` owns the typed authoring catalog and deterministic generated JSON. One positive and sixteen negative Nickel fixtures cover baseline roles, extension classes, duplicate IDs, unsupported classes, contradictory posture, required capabilities outside the ceiling, missing capability/output/separation/toolset references, unsafe labels and purposes, stale hashes, missing baselines, invalid separation, untrusted sources, and unadmitted project profiles.
- `animus_runtime::agent_roles` is a pure core for catalog validation, canonical BLAKE3 identity, exact assignment and activation, role/model resource composition, exact capability intersection, separation decisions, immutable plans and transitions, handoffs, returned native-tool binding, output admission, and metadata-only receipt projection.
- Assignment, activation, handoff, candidate/review/verification/judgment/coordination/observation emission, selection, and veto are separate exact `GuardedAction::AgentRole` ceremonies. Their resources bind the applicable role, subject/task, actor/lineage, model, policy/grant, separation, freshness, output/evidence, and action identities. No ceremony mints a later capability.
- Effective capabilities are intersected independently across verified grants, host policy, parent delegation, task scope, agent definition, semantic role, model profile, workspace, tool/executor admission, explicit approval, sandbox admission, cancellation state, timeout state, and named finite budgets. Missing sources are reported separately; aliases, wildcards, duplicate stable labels, and cross-assignment reuse fail closed.
- Reviewer, verifier, and judge separation decisions bind every candidate and subject-origin identity used by the selected policy. They report only checked dimensions and do not claim human, organizational, statistical, or global independence.
- Role outputs are bounded evidence. Implementers emit candidates, reviewers emit advisory/tighten-only findings, verifiers report independently admitted named host evidence without verifier-soundness claims, and judges use one admitted rubric without weakening deterministic denial. None establishes authoritative completion.
- `animus-std` owns the feature-gated explicit-path JSON loader. Reusable crates do not read files, inspect environment, evaluate Nickel, call providers/tools/checks, access credentials or clocks, mutate workspaces, persist records, or decide completion.

## Focused evidence

- `scripts/cairn/check-agent-role-profiles --root .`: pass. The rail checked deterministic Nickel parity, one positive and sixteen negative fixtures, runtime purity markers, 32 pure runtime role tests, 3 explicit-loader tests, and a bounded focused receipt.
- `cargo test -p animus-runtime --locked agent_roles`: pass, 32 tests covering canonical catalog identity, distinct assignment/activation, wrong action/resource/subject/lineage, router/semantic-role non-equivalence, independent late-stage ceilings, missing required capabilities, non-union, aliases/wildcards/duplicates, model-resource composition, class outputs, separation identity drift, plan reuse/recomposition/cancellation, metadata-only handoff, native tool binding, typed non-success output, and receipt leaks.
- `cargo test -p animus-std --locked --features agent-role-profiles agentroles`: pass, 3 tests covering current generated input, malformed/unknown/stale input, and missing explicit path without discovery.
- `scripts/config/export-nickel-configs` and `scripts/config/check-nickel-configs`: pass.
- Capability policy export plus `scripts/cairn/check-clash-ucan-authority --root .`: pass, including the complete semantic-role ability vocabulary and authority tests.
- `scripts/cairn/check-composition-conformance --root .`: pass.
- `scripts/cairn/check-adapter-conformance-index --root .`: pass with the `agent-role-profile` family and all required positive/negative case classes.
- `scripts/cairn/check-component-catalog --root .`: pass.
- `scripts/cairn/check-runtime-facade-inventory --root .`: pass.
- `scripts/cairn/check-request-lifecycle --root .`: pass.
- `scripts/cairn/check-dogfood-readiness --root .`: pass.
- `scripts/cairn/check-roadmap --root .`: pass.
- `scripts/cairn/check-evidence-safety --root .`: pass.
- `nix build path:$PWD#checks.x86_64-linux.agent-role-profiles --no-link -L`: pass.

## Widened evidence

- `cargo test --workspace --locked --no-fail-fast`: pass.
- `cargo check --workspace --all-targets --locked`: pass.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: pass.
- `cargo clippy -p animus-std --all-targets --locked --features agent-role-profiles -- -D warnings`: pass.
- `cargo clippy -p animus --all-targets --locked --features agent-role-profiles -- -D warnings`: pass.
- `cargo tigerstyle check`: pass after replacing ambiguous interfaces and boolean bundles with typed inputs, splitting long cores at validation phases, using fixed-width receipt counts, separating every late-stage ceiling, and adding success-path invariant assertions.
- `pre-commit run --all-files`: pass for generated Nickel configs, router conformance, and Octet lints.
- `nix flake check -L`: pass for the supported system checks.

## Bounded portfolio audit

The audit goal was to falsify semantic-role least privilege, immutable binding, scoped separation, and metadata-only evidence before archive. Observable completion required concrete negative tests plus deterministic repository rails; model agreement, documentation, test count, and lifecycle checkboxes were excluded as completion evidence. The budget was four mechanism families, one secondary-model consultation, one adversarial reconciliation round, no web retrieval, and repository-owned validators as the final authority.

- **Specification trace:** mapped every added requirement and scenario to a concrete API, fixture, or test. This exposed the absence of a direct router-judge non-equivalence fixture; `equal_router_and_semantic_judge_labels_do_not_grant_semantic_authority` now proves an equal label and allowed non-role action cannot activate a semantic judge.
- **Authority attack:** inspected every grant boundary and capability source for union, alias, wildcard, or implicit-host fallback. This exposed a combined sandbox/budget source that obscured approval, cancellation, timeout, and budget denials. The resolver now treats approval, sandbox, cancellation, timeout, and named budgets as separate exact ceilings and tests each independently with parent delegation.
- **Identity and transition attack:** compared every mutable input with assignment, plan, transition, output, and receipt hashes. This exposed separation decisions that previously omitted subject-origin actor/run/session/model/workspace/lineage identities from the decision hash. The decision now hashes both candidate and subject-origin identities, policy requirements, supplied state dimensions, and verifier/rubric bindings; a passing alternate origin produces a different decision identity.
- **Evidence attack:** enumerated raw prompt, reasoning, provider, credential, path, verifier, oracle, diff/file, tool, rationale, and transcript channels across outputs, handoffs, and receipts. Typed forbidden-claim and leak classes, bounded collections, exact output resources, and per-field negative fixtures remain fail closed.
- **Secondary review:** VibeThinker suggested union, stale assignment, model-drift, and router-judge risks. Union and model drift were already deterministically covered; assignment revocation is host-owned and not claimed by this pure core. The router-role suggestion was accepted as the direct negative fixture above. The model response was advisory only and did not substitute for repository checks.

The surviving implementation passed the adversarial checks after the two structural corrections above. The serial local lenses are correlated and are not claimed as independent review.

## Claim boundary

This evidence establishes checked authoring shape, exact host-loaded revalidation, scoped role authorization, non-union capability intersection, explicit role/model composition, bounded separation checks, immutable transition decisions, metadata-only handoffs/outputs/receipts, and inventory consistency for supplied in-memory inputs. It does not prove model competence, provider or tool correctness, credential handling, verifier soundness, review or judge correctness, real-world independence, filesystem/process/network confinement, successful mutation, authoritative completion, release eligibility, or whole-system correctness. Hosts remain responsible for current identities and grants, service execution, approvals, sandbox/cancellation/timeout observations, mutation, persistence, clocks, credentials, and the separate authoritative-completion gate.

## Lifecycle evidence

- `nix run path:$PWD#cairn -- validate --root .`: pass before sync with no lifecycle or spec issues.
- `nix run path:$PWD#cairn -- gate proposal add-capability-bound-agent-roles --root .`: pass.
- `nix run path:$PWD#cairn -- gate design add-capability-bound-agent-roles --root .`: pass.
- `nix run path:$PWD#cairn -- gate tasks add-capability-bound-agent-roles --root .`: pass.
- Sync dry-run: unblocked with no reasons; plan `b20ecd5b88fefe1878dee9f19402622f4fc09f130fb21a0de049e5e701a533a3`.
- Sync execution and immediate validation: pass. The accepted runtime spec contains the synced `agent_role_profiles.contract` through `agent_role_profiles.verification.closeout` requirements.
- Archive dry-run: unblocked with no reasons; plan `9140bcdfdab5fdc952835b0406b41ffd3bd4c369ba831ad698fd1602c4e57b02`.
- Archive execution with `CAIRN_ARCHIVE_DATE=2026-07-12`: pass; receipt `3c70263b54c1fa6d5018a8aefa4efee640af303b415af11582c8777e490f779c`.
- Post-archive `nix run path:$PWD#cairn -- validate --root .`: pass with 10 remaining active changes, 13 accepted specs, and no lifecycle or spec issues.
