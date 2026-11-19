# Frozen local selection policies

`crates/animus-frozen-policy` is the shared System One leaf (Animus #15). A frozen policy answers one bounded question: given a finite vector of integer features, which action from a declared, finite set should a consumer prefer? The leaf admits identity-bound artifacts, decides in checked fixed point, and issues bounded receipts. It has two consumers:

- the campaign manager's evaluation lane ([campaign-manager-eval.md](campaign-manager-eval.md#frozen-policy-tree-route));
- Lattice's optional typed policy operation (Lattice #23).

## Boundary

- The leaf is `no_std` + `alloc`. It depends on `blake3`, `serde` and `serde_json` without their default features, and on no other Animus crate.
- It reads no files, environment, clocks, randomness or network, and it never trains or updates weights.
- Hosts load the artifact bytes, enforce wall-clock deadlines and cancellation, and persist receipts.
- A selection is a preference among actions the caller has already declared eligible. It is not task success, and it grants no authority.

## Artifact: `animus.frozen-policy.v1`

| Key | Contents |
| --- | --- |
| `schema` | `"animus.frozen-policy.v1"` |
| `runtime` | `{crate: "animus-frozen-policy", artifact_api: 1}` |
| `feature_schema` | `{id, normalization: "min-max-per-mille-v1", features: [{name, min, max}]}`, with 1–64 features and `min < max` within ±10⁹ |
| `action_schema` | `{id, actions: [label]}`, with 1–64 unique labels of at most 64 bytes in `[A-Za-z0-9._:-]` |
| `model` | `{kind: "decision-tree-v1", nodes}` or `{kind: "mlp-per-mille-v1", layers}` |
| `bounds` | `{score_min, score_max, abstain_below?, operation_budget}`, with scores within ±10⁶ and a budget of 1 to 10⁶ |
| `provenance` | `{kind: "fixture" \| "trained" \| "distilled", producer, evaluation, non_claims: [..]}` |

The model kinds:

- **Normalization:** each raw feature becomes `(value - min) * 1000 / (max - min)`, rounding down. A value outside `[min, max]` is refused, never clamped.
- **`decision-tree-v1`:** `nodes[0]` is the root. A node is either `{"split": {feature, threshold, below, above}}`, which continues at `below` when the normalized feature is at most `threshold` (0 to 1000), or `{"leaf": {scores}}`, which holds one score per action. Every child index exceeds its parent's, every non-root node has exactly one parent, and a tree has at most 255 nodes and 16 splits on any path.
- **`mlp-per-mille-v1`:** one to four layers `{activation: "relu" | "identity", biases, weights}`, each at most 64 outputs wide, computing `out = bias + Σ weight·in / 1000` in checked `i64`. The last layer is `identity` and has one output per action.
- **Overflow:** overflow, or an output outside `[score_min, score_max]`, is `score-out-of-range`.

Identities:

| Identity | Domain | Covers |
| --- | --- | --- |
| Artifact | `animus.frozen-policy.v1` | The canonical compact encoding, keys sorted at every level |
| Model (weights digest) | `animus.frozen-policy-model.v1` | The model section alone, so it ignores provenance |

Both are `b3:` followed by 64 lowercase hexadecimal characters. They use the same framed BLAKE3 as `animus_eval_core::hash_identity`. Neither key order nor spacing changes an identity, and a file name never serves as one.

Authoring contract: `config/frozen-policy-contract.ncl`. Examples: `config/frozen-policy-examples.ncl` (one tree, one MLP), exported to `config/generated/frozen-policy-example-{tree,mlp}.json`.

## API

```rust
pub fn policy_identity(bytes: &[u8]) -> Result<String, AdmissionError>;
pub fn admit_policy(bytes: &[u8], expected_identity: &str) -> Result<AdmittedPolicy, AdmissionError>;
pub fn decide(policy: Result<&AdmittedPolicy, &AdmissionError>, request: &DecisionRequest) -> Decision;
pub fn verify_receipt(bytes: &[u8]) -> Result<DecisionReceipt, ReceiptError>;

pub struct PolicyBinding { pub action_schema: String, pub artifact: String, pub feature_schema: String }
pub struct DecisionRequest {
    pub binding: PolicyBinding,
    pub features: Vec<i64>,          // raw values, declared order
    pub eligible: Vec<String>,       // the consumer's admitted action subset
    pub fallback: Option<String>,    // must be eligible
    pub max_operations: u64,         // deterministic deadline
}
pub struct Decision { pub outcome: Outcome, pub receipt: DecisionReceipt }
pub enum Outcome {
    Selected { action: String, score: i64 },
    Fallback { action: String, failure: Failure },
    Refused { failure: Failure },
}
```

Pass `Err(&AdmissionError::Unavailable)` to `decide` when no artifact is loaded.

`AdmissionError` codes:

- `unavailable`
- `too-large` (over 64 KiB)
- `malformed`
- `schema`
- `runtime`
- `feature-schema`
- `action-schema`
- `model`
- `parameters`
- `bounds`
- `provenance`
- `identity`

## Decision order and failures

1. **Request shape.** A refusal here is `malformed-input` with the detail `binding`, `feature-count`, `no-eligible-action`, `too-many-actions`, `duplicate-action` or `fallback-not-eligible`. A request with a malformed shape is always `Refused`, even when it declares a fallback.
2. **Artifact slot.** `unavailable`, or `incompatible-artifact` with the admission code as detail.
3. **Binding against the admitted artifact.** `stale-binding` with the detail `artifact`, `feature-schema` or `action-schema`.
4. **Input against the artifact.** `malformed-input` (`feature-count` or `unknown-action`), then `input-out-of-range` with the detail `feature-<index>`.
5. **Operation budget.** `operation-budget` when the artifact's worst-case operations exceed `max_operations`.
6. **Inference.** `score-out-of-range`.
7. **Selection.** The best eligible score wins, and equal scores go to the earliest declared action. If that score is below `abstain_below`, the failure is `abstained`.

When a request with a valid shape fails at steps 2–7, it answers `Fallback` with its declared fallback; without one it is `Refused`. Inference never runs after a refusal at steps 1–5.

## Receipt: `animus.frozen-policy-receipt.v1`

`DecisionReceipt { body, identity }`, where `identity` is the `b3:` identity of the canonical body under the receipt schema. The body (keys sorted):

| Field | Contents |
| --- | --- |
| `action` | The answering action: the selection or the fallback |
| `action_set` | The `animus.frozen-policy-action-set.v1` identity over the action schema and the sorted eligible labels |
| `artifact`, `model`, `policy_kind` | The admitted identities and kind, when an artifact was admitted |
| `artifact_api`, `runtime_crate`, `runtime_version` | The runtime revision |
| `binding` | The requested pins; empty identities when the binding was unsafe |
| `cost` | `{fallback_decisions, inference_operations, normalization_operations}` |
| `eligible`, `features` | As requested; empty when the request's shape was refused |
| `failure`, `failure_detail` | The failure code and its bounded detail |
| `input` | The `animus.frozen-policy-input.v1` identity over the feature schema and the raw features |
| `outcome` | `selected`, `fallback` or `refused` |
| `schema` | The receipt schema |
| `score`, `scores` | The selected score, and one score per declared action |

`DecisionReceipt::encode()` returns the compact encoding to retain. `verify_receipt` admits at most 16 KiB, checks the schema, runtime, bounds, outcome consistency and identity, and returns the receipt; replay uses it and never re-runs inference. A later model change changes later receipts and leaves earlier ones valid.

## Non-claims

- Every artifact in this repository is a hand-authored fixture. RSI Lab owns training, distillation and export (RSI Lab #80).
- The operation budget is a deterministic stand-in for a deadline. Measured wall-clock time, memory and model-load cost are host observations.
- Recurrent state, online updates and other model kinds are out of scope until a consumer qualifies them (Lattice #12).
- Shared serialization does not establish that one artifact transfers across domains. Each consumer qualifies its own policy and outcome.
