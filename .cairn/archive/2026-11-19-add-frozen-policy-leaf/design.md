## Context

`add-frozen-local-scorer` (archived 2026-11-18) put a frozen linear scorer into `animus-campaign-manager-eval`. Its artifact (`animus.campaign-manager-local-scorer.v1`), scoring, dispositions and records belong to that AGPL evaluation crate and parse its rendered brief. Animus #15 asks for a simple tree or small MLP, a `stale` disposition, records of the actual policy input and output with runtime revisions, and a shared leaf once a second real consumer appears. Lattice #23 is that consumer. It needs finite selection with typed failures and bounded receipts. It cannot depend on Campaign, the manager, or the evaluation lane.

## Decisions

### Decision: one pure leaf crate, `animus-frozen-policy`

**Choice:**
- New workspace member, `no_std` + `alloc`, licensed MIT OR Apache-2.0.
- Dependencies: `blake3`, `serde` and `serde_json`, none with default features.
- It is not a default member.
- The flake checks it for `wasm32-unknown-unknown`, which keeps it `no_std`.

**Rationale:** Lattice can vendor one small crate. The leaf has no filesystem, process, clock, randomness, network or inference-runtime authority, so it never needs a host to decide. It does not depend on `animus-eval-core`, whose source directory exceeds Octet's module-file limit for any dependent crate. Its `domain_identity` reproduces `animus_eval_core::hash_identity` byte for byte: framed BLAKE3 with a `b3:` prefix. A test pins that equality, so identities from either crate agree.

### Decision: artifact `animus.frozen-policy.v1`

**Choice:** A JSON document with exactly these keys:

| Key | Contents |
| --- | --- |
| `schema` | The artifact schema. |
| `runtime` | `{crate: "animus-frozen-policy", artifact_api: 1}` |
| `feature_schema` | `{id, normalization: "min-max-per-mille-v1", features: [{name, min, max}]}`, with 1–64 features and raw bounds within ±10⁹. |
| `action_schema` | `{id, actions: [labels]}`, with 1–64 unique safe labels. |
| `model` | `{kind: "decision-tree-v1", nodes}` or `{kind: "mlp-per-mille-v1", layers}` |
| `bounds` | `{score_min, score_max, abstain_below?, operation_budget}` |
| `provenance` | `{kind: fixture\|trained\|distilled, producer, evaluation, non_claims}` |

- The artifact identity is `domain_identity("animus.frozen-policy.v1", [compact canonical JSON])`. The canonical form re-encodes the typed document, whose keys are declared in sorted order, so neither key order nor spacing changes the identity.
- The model identity is `domain_identity("animus.frozen-policy-model.v1", [model JSON])`: a weights digest that ignores provenance.
- Admission takes the bytes and the selected identity. It refuses an artifact before any decision with these typed `AdmissionError`s, checked in this order: `Unavailable`, `TooLarge`, `Malformed` (including unknown fields and unknown kinds), `Schema`, `Runtime`, `FeatureSchema`, `ActionSchema`, `Bounds`, `Provenance`, `Model`, `Parameters`, then worst-case operations over the budget (`Bounds`), then `Identity`.
- A file name or model name is never identity.

**Rationale:** The document binds everything Animus #15 lists: weights, model digest, feature and action schemas, normalization, bounds, runtime compatibility and evaluation provenance. RSI Lab #80 can export to it without Animus code.

### Decision: two frozen model kinds in checked fixed point

**Choice:** Inputs are normalized to per mille with `(value - min) * 1000 / (max - min)`, rounding down. A value outside `[min, max]` is refused, never clamped.

- `decision-tree-v1`:
  - Node 0 is the root.
  - A split `{feature, threshold ∈ [0, 1000], below, above}` continues at `below` when `normalized[feature] <= threshold`.
  - Every child index exceeds its parent's, and every non-root node has exactly one parent, so the tree is acyclic, connected and terminating.
  - The tree has at most 255 nodes and 16 splits on any path.
  - A leaf holds one score per action.
  - Inference operations are the splits visited; the worst case is the deepest path.
- `mlp-per-mille-v1`:
  - One to four dense layers, each at most 64 wide.
  - `out = bias + Σ weight·in / 1000`, in checked `i64` with truncating division.
  - Hidden layers use `relu` or `identity`; the output layer uses `identity`, with one output per action.
  - Inference operations are the weight-input products, a fixed worst case.
- Weights, biases, leaf scores and score bounds lie within ±10⁶.
- Overflow, or any output outside `[score_min, score_max]`, is `ScoreOutOfRange`.

**Rationale:** These are the issue's "simple tree or small MLP", with integer determinism and no inference runtime. Recurrent state stays out until a history-dependent task justifies it (Lattice #12).

### Decision: selection, binding and failures

**Choice:** `decide(policy: Result<&AdmittedPolicy, &AdmissionError>, request: &DecisionRequest) -> Decision`.

`DecisionRequest` has five fields:
- `binding: PolicyBinding {action_schema, artifact, feature_schema}`, the consumer profile's pins;
- `features: Vec<i64>`, in declared order;
- `eligible: Vec<String>`, the consumer's admitted subset of actions;
- `fallback: Option<String>`, which must be eligible;
- `max_operations: u64`, the deterministic deadline.

Checks run in this order:
1. The request's own shape: `MalformedInput(Binding | FeatureCount | NoEligibleAction | TooManyActions | DuplicateAction | FallbackNotEligible)`.
2. The artifact slot: `Unavailable`, or `IncompatibleArtifact(AdmissionError)`.
3. The binding against the admitted artifact: `Stale(Artifact | FeatureSchema | ActionSchema)`.
4. The input against the artifact: `MalformedInput(FeatureCount | UnknownAction)`, then `InputOutOfRange {feature}`.
5. The worst-case operations against the request's budget: `OperationBudget`.
6. Inference: `ScoreOutOfRange`.
7. Selection: the best eligible score, with ties going to the earliest declared action, then `Abstained` when that score is below `abstain_below`.

The outcome is one of:
- `Selected {action, score}`;
- `Fallback {action, failure}`, when the request's shape was valid and a fallback was declared;
- `Refused {failure}`.

Inference never runs after a refusal in steps 1–5.

**Rationale:** Lattice #23 needs these typed outcomes, a declared bounded fallback, and selection that cannot leave the consumer's declared action set. A wall-clock timeout cannot be measured purely, so the operation budget is its deterministic stand-in. Hosts still enforce real deadlines and cancellation around the call.

### Decision: receipts that replay without inference

**Choice:** Every `Decision` carries a `DecisionReceipt {body, identity}`. The body has schema `animus.frozen-policy-receipt.v1` and binds:
- the requested binding, or empty identities if the binding itself was unsafe;
- the admitted artifact, model identity and policy kind;
- the runtime crate, version and artifact API;
- the input identity, which is `animus.frozen-policy-input.v1` over the feature schema and the raw features;
- the eligible-set identity, which is `animus.frozen-policy-action-set.v1` over the action schema and the sorted labels;
- the raw features and eligible labels, which are empty when the request's own shape was refused;
- the per-action scores;
- the outcome, action and score;
- the failure code and its detail;
- the cost units: normalizations, inference operations, and whether the fallback answered.

The identity is the body's domain identity. `encode()` gives compact canonical JSON. `verify_receipt(bytes)` admits at most 16 KiB and checks the schema, runtime, bounds, outcome consistency and identity, then returns the receipt. The receipt holds every decided field, so replay never re-runs inference.

**Rationale:** Animus #15 asks to record the actual policy input and output, model and runtime revisions, and measured cost, and says that replay must not regenerate bytes by inference. Lattice #23 asks for persisted invocation, input and candidate-set identities, and playback without the accelerator. A model change alters later receipts and leaves earlier ones valid.

### Decision: the campaign route is the leaf's first consumer

**Choice:** `ClassifierRoute` (`model:frozen-policy:<16 hex>`, in `routes/frozen/leaf`) projects the six existing brief features of each listed candidate as raw values in `[0, 1000]`. It decides each candidate through the leaf, with the classes `follow-signal`, `explore` and `exploit` (`animus.campaign-manager-candidate-class.v1`) all eligible and no per-candidate fallback.

- The route owns the class meaning: `follow-signal` gives the observed signal, or 0 for an unexecuted candidate; `explore` gives 10, and `exploit` gives 20.
- A refused candidate stops the brief. The whole brief then answers with `explore-only`'s reply, recorded under the mapped disposition: `MalformedBrief`, `OutOfRange`, `OperationBudget`, `IncompatibleArtifact` (also for an artifact with other class labels), `Unavailable`, `Stale` or `Abstained`.
- `DecisionRecord.receipts` keeps the leaf receipt identities. Records stay bounded at 4,096.
- The fixture `config/campaign-manager-frozen-policy-fixture.ncl` is a seven-node hand-authored tree, written before any comparison run without reading the evaluation tasks.
- Random arms draw uniformly from the route's own rank range `[0, 20]`.

**Rationale:** This qualifies the tree route through the existing `ModelAdapter` and `advise` seam, with the same grant as `explore-only`. The linear scorer and its archived artifact stay unchanged; the archived protocol and evidence bind them.

### Decision: second predeclared protocol, same rule

**Choice:**
- `config/campaign-manager-frozen-policy-eval.ncl` (`animus.campaign-manager-frozen-policy-eval.v1`) selects the tree fixture's identity.
- It keeps the baseline (`explore-only`), the reference (`signal-greedy`), seeds 1–8, the primary outcome (`found`) and the adoption rule (gain ≥ 0.100, random parity, no fallback). Its non-claims add that equal total compute is not measured.
- `parse_comparison_protocol` accepts both schemas, and `ScorerArm` names the arm.
- The comparison shell and report are generic over the scorer arm.
- `--frozen-policy` runs the protocol and prints the report, whose header is `FROZEN_POLICY_COMPARISON`.
- The scorer line adds `stale`, `abstained` and `leaf_receipts` counts. The linear report gains the same zero-valued fields.

**Rationale:** The arms, seeds and rule were fixed before the arm ran, and both scorers face the same rule.

## Risks / Trade-offs

- A hand-authored tree encodes the scripted heuristic, so its result is not learning; the report and evidence say so.
- Three synthetic tasks are a small sample, so the result is a qualification of the route boundary, not an efficacy claim.
- Per-candidate receipts add hashing and JSON encoding to every decision. The debug-build latency measured in the comparison is reported as a non-deterministic observation.
- The `stale` disposition is reachable on the campaign route only through a mismatched route binding, because admission already pins the identity. Lattice's long-lived profile is where staleness occurs in practice.
- `DecisionRecord` gained a field and `Disposition` gained two variants: a source change for any external code that constructs records or matches exhaustively. The crate has no such external caller in this workspace.
