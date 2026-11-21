# Qualification attachments

A qualification attachment answers one bounded question: what evidence supports choosing this exact agent profile for one task family? It records evidence about the evaluated subject and grants no authority. It never activates a revision, widens capabilities, bypasses current revocation, or replaces per-candidate acceptance ([Animus #18](https://github.com/OnixResearch/animus/issues/18)).

The pure core is `animus_runtime::qualification` (`crates/animus-runtime/src/eval_feedback/qualification/`). It compiles under every `animus-runtime` feature set, including `--no-default-features --features steel`, and uses only `blake3`. It has no clock, filesystem, network, process, credential, or provider authority. The campaign-manager evaluation lane is the first producer ([campaign-manager-eval.md](campaign-manager-eval.md#qualification-attachments)).

## Subject

`animus.qualification-subject.v1` binds the complete evaluated subject.

| Field | Rule |
|---|---|
| `lattice_definition_identity` | `blake3:` + 64 lowercase hex: Lattice `lattice-worker-definition-v1` over owner, basis, and name |
| `lattice_profile_revision` | Bare 64 lowercase hex: Lattice `Request::worker_profile_identity()`, equal to the execution binding's `profile_revision` |
| `task_family` | Lattice capability name: `[a-z0-9][a-z0-9._:-]*`, at most 64 bytes |
| `components` | All 25 component kinds, in wire order, each `{ kind, basis, identity }` |
| `model` | `{ deployment, version_basis, version, weights_identity }` |

Any other spelling of a subject key is `malformed`. A new executable, prompt, tool list, or route keeps the definition identity and changes the revision. Per-attempt prompts and workspace snapshots are run inputs, so they change neither.

The component kinds are:

- `agent_definition`, `semantic_role`
- `prompt`, `skills`, `strategy`, `memory_policy`
- `harness`, `tool_schemas`, `tool_implementation`, `executable`, `environment`
- `model_route`, `context_controls`, `output_controls`, `sampling_controls`
- `permissions`, `isolation`, `network`
- `task_protocol`, `corpus`, `verifier`, `selection_policy`
- `budget`, `topology`, `evaluation_config`

Each component's `basis` is `declared`, `observed`, or `unavailable`. An unavailable component has `identity: null`, never an empty or placeholder hash. `component_identity(kind, bytes)` is the framed BLAKE3 identity under domain `animus.qualification-component.v1`.

Model `version_basis` takes one of four values:

- `provider_declared`: hosted models only. Carries `version` (the facts the provider exposes) and never a weight identity.
- `verified_artifact`: local or scripted routes only. Requires `weights_identity`.
- `declared`: local or scripted routes only. Carries `version`.
- `unavailable`: carries neither.

## Attachment

`animus.qualification-attachment.v1` documents are canonical JSON-subset bytes:

- no whitespace and no string escapes;
- object keys sorted by byte order and unique;
- unsigned integers only;
- strings at most 128 bytes, lists at most 64 items, nesting at most 6 levels deep;
- at most 16,384 bytes in total.

Decoding re-encodes the document and requires byte equality, so each attachment has exactly one spelling. The attachment identity is `blake3:` + hex of `blake3::hash(canonical_bytes)`. Lattice uses it verbatim as `qualification_ref`.

| Field | Content |
|---|---|
| `schema` | `animus.qualification-attachment.v1` |
| `subject` | The subject above |
| `evidence` | `{ class, protocol_identity, verifier_identity, corpus_identity, source_identity }` |
| `outcomes` | `{ planned, accepted, rejected, failed, cancelled, incomplete }`, where every count except `planned` sums to `planned`. Required for `contract_conformance` and `task_family_evaluation`; `null` otherwise |
| `measures` | Task-family measures. Required for `task_family_evaluation`; `null` otherwise |
| `scope` | `{ partitions, limitations, missing_modes }`: sorted unique tokens, with at least one limitation |
| `provenance` | `{ label: fixture\|live, producer, producer_revision, observed_at_ms, valid_until_ms }` |
| `references` | Sorted unique `{ kind, identity }` artifact or ledger references |

The four evidence classes are independent facts, never a ladder:

| Class | `protocol_identity` | `verifier_identity` | `outcomes` |
|---|---|---|---|
| `declared_support` | Declaration schema | Declaring owner | none |
| `contract_conformance` | Contract-test protocol | Independent checker | Checks |
| `task_family_evaluation` | Task protocol | Independent verifier | Trials |
| `deployment_admission` | Admission policy | Admitting host | none |

For `contract_conformance` and `task_family_evaluation`, the protocol, verifier, and corpus identities must equal the subject's observed `task_protocol`, `verifier`, and `corpus` components.

Every string is either a token or a `blake3:` identity. A token follows the task-family rule above and is refused if a credential marker (`sk-`, `xoxb-`, `xoxp-`, `akia`, `bearer`, `password`, `passwd`, `private-key`, `api-key`) starts the token or follows a `.`, `:` or `-`. Raw prompts, hidden tests, credentials, provider bodies, and host paths cannot be represented, and attempts to include them are `malformed`. Unknown fields are `malformed` too. That includes authority fields such as `activate_revision`, throughput fields, and self-scored success fields.

### Example

`fixtures/qualification-attachment/positive/signal-greedy.json` is the complete attachment that the scripted campaign-manager job produces for `model:scripted:signal-greedy`. Its identity is `blake3:46830a82f59932dbe48b167c23db8d999d93680edadbb037cc8cb309ee953658`. The file holds the canonical bytes plus one trailing newline, and the identity covers the bytes without that newline. Abbreviated and pretty-printed:

```json
{
  "evidence": {
    "class": "task_family_evaluation",
    "corpus_identity": "blake3:022d60ee09c428c76b179e19e6d11e91a2f84178e56887b08b85b9a096ec513c",
    "protocol_identity": "blake3:2f0dc142c82e7c873abc70cfefef446cb8fbff838a21d801419ecb1812563f1c",
    "source_identity": "blake3:978126be0b7dc4976a6691b552baf9a8742896f5f91f8d2de50590918909b946",
    "verifier_identity": "blake3:799a4742d3a0e79074d3e0cf05f77e94cded46c964f1db01ed983c585de9c060"
  },
  "measures": {
    "costs": { "manager": null, "model": null, "tool": null, "unit": "campaign-cost-units", "unknown_remainder": true, "verifier": null },
    "covered_time_ms": { "queue": null, "repair": null, "verification": null },
    "false_completion_attempts": null, "invalid_actions": null, "repetitions": 1,
    "review_interventions": null, "task_coverage": { "covered": 3, "declared": 3 }, "time_to_acceptance_ms": null
  },
  "outcomes": { "accepted": 3, "cancelled": 0, "failed": 0, "incomplete": 0, "planned": 3, "rejected": 0 },
  "provenance": { "label": "fixture", "observed_at_ms": 1790000000000, "producer": "animus-campaign-manager-eval",
                  "producer_revision": "animus-4071c65e-qualification", "valid_until_ms": 1792592000000 },
  "references": [ { "identity": "blake3:29395f91…", "kind": "atif-trajectory" }, "… 2 more" ],
  "schema": "animus.qualification-attachment.v1",
  "scope": {
    "limitations": [ "not-advice-quality-beyond-these-tasks", "not-automatic-completion", "…" ],
    "missing_modes": [ "context-output-sampling-controls", "cost-accounting", "live-model-route", "prompt-identity",
                       "review-burden", "time-to-acceptance", "tool-implementation-identity" ],
    "partitions": [ "task:cache-key-collision", "task:config-parser-crash", "task:json-decoder-hang" ]
  },
  "subject": {
    "components": [ { "basis": "observed", "identity": "blake3:30344adb…", "kind": "agent_definition" },
                    { "basis": "unavailable", "identity": null, "kind": "context_controls" }, "… 23 more" ],
    "lattice_definition_identity": "blake3:f8521e22a5a9a42502144d16591744d9e829176fb73f3773b5a19c9c7a5935da",
    "lattice_profile_revision": "82b6ebc17a6748b19279f346df775f3f7b74e36e55f11867ff95f5bc8cd5549a",
    "model": { "deployment": "scripted", "version": "model:scripted:signal-greedy", "version_basis": "declared", "weights_identity": null },
    "schema": "animus.qualification-subject.v1",
    "task_family": "campaign.exploration"
  }
}
```

## Measures

Failed, cancelled, and incomplete trials stay in `planned`. A shrunk denominator is `malformed`: `measures.task_coverage.declared × repetitions` must equal `planned`. An unmeasured cost or time is `null`, never `0`. If any cost is `null`, `unknown_remainder` must be `true`.

- `cost_per_accepted` returns `Defined { known_total, accepted }` only when every cost is known and something was accepted. Otherwise it returns `NoAcceptedCompletion` or `UnknownCost`.
- `compare_equal_budget(baseline, candidate)` compares independently accepted completions only. It refuses unless the task family, the denominator, and the `budget`, `corpus`, `environment`, `evaluation_config`, `isolation`, `network`, `permissions`, `selection_policy`, `task_protocol`, `topology`, and `verifier` identities are identical and available. Token throughput and self-written scores are not fields, so they cannot select a route.

## Consumer API

```rust
pub fn validate_attachment(bytes: &[u8], expected: &ExpectedSubject<'_>) -> Result<Attachment, Rejection>;
pub fn qualify(attachments: &[&[u8]], expected: &ExpectedSubject<'_>) -> QualificationAnswer;
pub fn replay_attachments(attachments: &[&[u8]]) -> ReplayReport;
pub fn decode_attachment(bytes: &[u8]) -> Result<Attachment, Rejection>;
pub fn encode_attachment(document: &AttachmentDocument) -> Result<Attachment, Rejection>;
pub fn classify_applicability(attachment: &Attachment, current: &QualificationSubject,
    policy: &ApplicabilityPolicy<'_>, decision: Option<&CompatibilityDecision>) -> Applicability;
pub fn review_qualification(applicability: &Applicability) -> EvalFeedbackReviewDecision;
```

`ExpectedSubject` carries:

- `lattice_definition_identity`, `lattice_profile_revision`, and `task_family`;
- `required_class`;
- `now_ms`;
- `required_components`;
- `current_components` (`{ kind, identity: Option }` for components outside the profile revision).

`validate_attachment` checks in stage order and returns the first failure as `Rejection { reason, field }`:

| Stage | `reason` | When |
|---|---|---|
| 1 | `malformed` | Bytes, canonical form, shape, spelling, bounds, or redaction fail, including a mis-spelled expected subject |
| 2 | `subject_mismatch` | The definition identity or profile revision differs |
| 3 | `scope_mismatch` | The task family differs |
| 4 | `insufficient_class` | The class is not the required class; classes never substitute for each other |
| 5 | `unusable` | A required component is unavailable in the evidence, a current component is unavailable now, or `now_ms < observed_at_ms` |
| 6 | `stale` | `now_ms >= valid_until_ms`, or a supplied current component identity changed |

`qualify` collapses exact duplicates. Distinct valid attachments that claim one `source_identity` for the same subject keys, task family, and class are `conflicting`. Otherwise `qualify` returns `Qualified { qualification_ref, evidence_class, protocol_identity, verifier_identity, observed_at_ms }` for the most recent valid attachment. With no valid attachment it returns `NotQualified(furthest reason)`, and with no input it returns `Unavailable`. `replay_attachments` decodes stored bytes only. It reports accepted identities, duplicates, rejections, and conflicting sources. It runs nothing and fills in nothing.

## Applicability and authority

`classify_applicability` extends the `eval_feedback` freshness check to the complete subject:

- `Current`: every key and component is equal and the evidence is fresh.
- `Historical { changed, revision_changed, expired }`: any component, the revision, or freshness changed. The original attachment stays valid historical evidence.
- `Unusable { component }`: a required component is unavailable.
- `Inapplicable(subject_mismatch | scope_mismatch)`: the definition or task family differs, and no decision can change that.
- `CompatibleReuse { decision_identity, changed }`: a `CompatibilityDecision` names this exact attachment identity, revision pair, task family, and class, and waives every changed component and none of `budget`, `corpus`, `evaluation_config`, `task_protocol`, or `verifier`.

Reuse keeps the original claims and is never `Current`. The port still answers `subject_mismatch`, because it qualifies only exact subject keys. A strengthened evaluator needs a new protocol or verifier identity and a new attachment.

`review_qualification` lets only current evidence request separate review or admission. It always reports `activates_scaffold: false` and `changes_runtime_selection_policy: false`.

## Lattice adoption

1. Resync the vendored Animus snapshot to the commit that lands this change. In `scripts/sync_animus.rs`, update `EXPECTED_REVISION` and `SOURCE_ARCHIVE_BLAKE3`. The crate list and the dependency declaration (`animus-runtime`, `default-features = false, features = ["steel"]`) stay unchanged, and no new crate or Cargo dependency is needed.
2. Implement the qualification port over `animus_runtime::qualification::qualify` or `validate_attachment`. Pass the definition identity as published and the profile revision as bare hex, with `required_class` taken from the promotion policy. Map `Qualified` to `{ support: "qualified", qualification_ref }` and `NotQualified(reason)` to `reason.as_str()`. Keep `Unavailable` as the no-producer answer owned by `animus#18`.
3. Promotion stays a separate explicit Lattice operation. A qualified answer is evidence for review only, and current revocation still applies.

## Verification

```sh
cargo test -p animus-runtime --lib qualification
cargo test -p animus-campaign-manager-eval --test qualification --test attachment
cargo check -p animus-runtime --no-default-features --features steel
```

## Non-claims

The checked fixtures come from a deterministic scripted lane over three synthetic tasks. Their Lattice subject keys are fixture keys supplied by the test binding. They claim no live model, deployment, benchmark, or production qualification. The NVIDIA skill-evidence follow-up in #18 (skill-set manifests, scanner coverage, activation, paired arms) is not implemented.
