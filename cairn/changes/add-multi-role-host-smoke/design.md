# Design: Capability-bound multi-role host smoke

## Context

`animus-std` loads explicit generated role and model-profile projections. `animus-runtime` composes role-bound model plans, evaluates separation, admits assignment and output authority, intersects capability ceilings, rejects completion overclaims, and projects metadata-only receipts.

The current packaged smokes do not compose these contracts as one multi-role workflow.

## Success Contract

The change is complete when one non-default binary activates implementer, reviewer, verifier, and judge assignments over one immutable subject.

Each role must use an exact loaded model profile, a distinct assignment, the role capability ceiling, explicit separation observations, separate output authority, and a metadata-only receipt.

The workflow must admit candidate, finding, verification-evidence, and veto outputs. It must reject reviewer mutation and role-output completion claims.

Completion evidence includes positive and negative fixtures, a source-bound current-head receipt, the focused script, the focused Nix check, role and completion rails, and Cairn gates.

Compilation, isolated role unit tests, prompt labels, task markers, or role outputs without exact assignment and output authority are false-completion cases.

## Decisions

### 1. Add a separate non-default smoke feature

**Choice:** Add `multi-role-smoke`, `animus-multi-role-smoke`, and a focused library module to `animus-std`.

**Rationale:** A separate feature keeps existing smoke claims stable and does not add role coordination to default builds.

### 2. Use accepted role and model-profile contracts

**Choice:** Load both generated projections from explicit paths. Use production role composition, separation, assignment, capability, output, receipt, and completion-candidate APIs.

**Rationale:** The smoke must prove composition through maintained contracts instead of duplicating role semantics.

### 3. Use one bounded four-role workflow

**Choice:** Activate implementer, reviewer, verifier, and judge in order. The verifier and judge both consume bounded evidence from the reviewer subject.

**Rationale:** This ordering permits explicit separation under the accepted profile cohort while keeping one immutable subject identity.

### 4. Keep output authority separate from activation

**Choice:** Require a new scoped grant for each candidate, finding, verification, and veto output.

**Rationale:** Assignment or activation authority must not become influence authority.

### 5. Prove completion non-equivalence through the completion gate

**Choice:** Reject `AgentRoleOutputClaim::AuthoritativeCompletion` and submit the judge output identity to the separate completion-candidate admission API.

**Rationale:** The role layer must not convert a candidate, finding, verification report, or veto into authoritative completion.

### 6. Emit metadata-only output and a current-head rail receipt

**Choice:** Render role, class, profile, model-plan, assignment, separation, capability-count, output, and receipt identities. The focused rail records source and Git-head metadata.

**Rationale:** Operators can index the bounded result without storing role context, outputs, verifier bodies, rationale, credentials, paths, or transcripts.

## Portfolio Search Record

The exact goal is a packaged deterministic host path for four capability-bound roles with completion non-equivalence. Completion requires exact role activation, separation, output authority, safe receipts, and denial coverage.

One agent performed the following serial lenses, so the passes are correlated.

- **Separate deterministic multi-role smoke**: selected. It closes the roadmap gap with existing contracts and no external authority.
- **Extend the Aeneas proof campaign**: rejected because that specialized recipe has no generic judge role and adds proof-specific artifacts.
- **Extend the live provider smoke**: rejected because credentials and network behavior make deterministic closeout unavailable.
- **Add only another role unit test**: falsified because it does not provide a packaged host path or current-head smoke receipt.

The adversarial audit targets role-label spoofing, capability union, same-actor review, stale model bindings, reviewer mutation, missing verifier evidence, missing rubric admission, denied veto authority, completion overclaim, and raw output leaks.

The retrieval budget covers role and model-profile loaders, role composition and output APIs, completion-candidate admission, packaged-smoke patterns, the dogfood roadmap, Nix checks, and lifecycle artifacts. The validation budget covers focused Cargo tests, role and completion rails, the new script and Nix check, dogfood readiness, Cairn validation, and Cairn gates.

## Risks / Trade-offs

- The smoke uses deterministic metadata and no model provider.
- Distinct identities prove only the checked separation dimensions.
- Veto admission does not prove judgment quality or verifier soundness.
- The smoke proves that role outputs are not completion. It does not produce authoritative completion evidence.
