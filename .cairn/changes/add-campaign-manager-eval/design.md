## Context

Animus adapts Harbor's evaluation mechanisms into Rust:
- `plan_trials` builds a deterministic task-major trial matrix with BLAKE3 trial identities.
- `EvaluationEnvironmentService` and `VerifierService` are the trial ports. `execute_separate_verifier` refuses a verifier that shares the environment's identity, and refuses evidence that does not match the request.
- `reduce_rewards` computes attributed means.
- `project_atif` validates ATIF-v1.7 facts.
- `receipt` builds a metadata-only job receipt.

`docs/harbor-evaluation-harness.md` keeps Harbor's Python application, Hub, and cloud backends out of the dependency graph. The campaign manager needs tasks, an environment, and a verifier over this slice.

## Decisions

### Decision: Evaluate through the native Harbor slice

**Choice:** The lane implements the two Harbor ports and drives them only through `plan_trials`, `execute_separate_verifier`, `reduce_rewards`, `project_atif`, and `receipt`.

**Rationale:**
- The user chose the native lane over real Harbor.
- It runs offline and deterministically in `nix flake check`, needs no Docker or Python, and reuses Animus's existing fail-closed trial and reward rules.

### Decision: Scenario campaigns with hidden signals

**Choice:**
- Each task lists candidates and offers labelled `family/approach`, each with a hidden signal from 0 to 9, and exactly one target.
- Running a candidate reveals its signal. The host appends it to that candidate's label as `[signal N]`.
- Running the target records a finding. The planner's stopping rule (`finding_limit: 1`) then stops the campaign.
- Every candidate costs one selection against the task's selection budget.
- In some tasks the target is only reachable by adding an offer from the family whose signal is strong.

**Rationale:**
- The model sees outcomes only through host labels, as a real host would show them.
- The tasks reward exploration followed by exploitation, and they separate a manager that adds offers from one that never does.

### Decision: The verifier grades the replay, not the environment's claims

**Choice:**
- The environment leaves one artifact per trial: the Choregraph graph, the control branch, the payloads, the envelope, the claimed final state identity, the turn ledger, and the ATIF projection.
- The verifier reloads the session with `ControlSession::load` and refuses a load failure as `Malformed` and a differing final state as `Stale`.
- `found` requires both the task's finding in the replayed findings and a recorded selection of the target's candidate.
- `efficiency` is `(budget + 1 - selections) / budget` for a found target, and 0 otherwise.
- `refusal-rate` counts refused replies and failed model calls in the environment's ledger. Jobs mark it advisory because no replay backs it.
- A reply that asks for no change (`UnchangedState`) is neither admitted nor refused.

**Rationale:** Rewards that decide outcomes come from Campaign-validated history only. Ledger-derived numbers are labelled as such.

### Decision: An in-memory artifact store stands in for the trial filesystem

**Choice:** `ArtifactStore` holds artifacts by trial identity behind a `RefCell`. The environment and the verifier both borrow it.

**Rationale:**
- `execute_separate_verifier` borrows both services at once, and artifacts reach the verifier out of band, as a Harbor verifier reads the trial's filesystem.
- The store is the lane's only shared mutable state.

### Decision: Scripted routes read only the rendered brief

**Choice:**
- `signal-greedy` explores unrun candidates. Once a family shows a signal of 6 or more, or nothing is left unrun, it adds that family's offers and ranks them first.
- `explore-only` explores, then reruns the best candidate, and never adds offers.
- `malformed` never answers in the schema.
- All three parse only the prompt text the manager renders.

**Rationale:** The routes give a positive control, a partial control, and a negative control that exercise the harness end to end without a provider.

### Decision: The live route is opt-in and reviewed

**Choice:**
- Under the `live` feature, `LiveRoute` sends each brief as one text-only, non-streaming request with no tools and no reasoning, through `animus-std-openai`'s reviewed OpenAI or OpenRouter Responses endpoint.
- It plans each request with `plan_provider_dispatch`, using the projection of that request as the reviewed fixture, as `animus-live-steel-agent` does.
- The binary reads the credential from standard input and needs an explicit `--live <endpoint> --model <id>`.

**Rationale:** Live scores are useful but non-deterministic and cost money, so they stay out of CI and require explicit authority.

## Risks / Trade-offs

- The tasks are small and synthetic, so scores rank managers only on them.
- A route may exploit the label format rather than the objective. The tasks keep the signal meaningful, but they do not prove semantic reasoning.
- The git-based flake checks see only tracked files. While other uncommitted work leaves `animus-runtime` referencing an untracked module, `cargo-campaign-manager-eval` and `campaign-manager-eval-octet` build only from a snapshot of the working tree.
