# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: The frozen-policy leaf is pure and authority-free

r[agent_harness_runtime.frozen_policy.boundary] Animus MUST provide frozen local selection policies through `animus-frozen-policy`, a `no_std` + `alloc` crate that depends on no other Animus crate and reads no files, environment, clocks, randomness or network. It MUST NOT train or update weights. Hosts MUST supply artifact bytes and enforce wall-clock deadlines, cancellation and persistence.

#### Scenario: The same request decides identically

- GIVEN an admitted artifact and one decision request
- WHEN the request is decided twice
- THEN both decisions and their receipt identities are identical.

### Requirement: Frozen-policy artifacts are versioned and identity-bound

r[agent_harness_runtime.frozen_policy.artifact] The leaf MUST admit only an `animus.frozen-policy.v1` artifact whose BLAKE3 identity over its canonical encoding equals the caller's selected identity. The artifact MUST bind the policy kind and weights, a feature schema with raw bounds and `min-max-per-mille-v1` normalization, an action schema, score bounds, an optional abstention threshold, an operation budget, runtime compatibility and provenance. Admission MUST fail with a typed error before any decision for a missing, oversized, malformed, wrong-schema, wrong-runtime, out-of-contract, out-of-bounds or unselected artifact. A file name or model name MUST NOT serve as identity. A separate model identity MUST digest the model section alone.

#### Scenario: A changed weight under the selected identity is refused

- GIVEN artifact bytes with one changed leaf score
- WHEN they are admitted under the original selected identity
- THEN admission fails with `Identity` and no decision is made.

### Requirement: Frozen models evaluate in checked fixed point

r[agent_harness_runtime.frozen_policy.model] The leaf MUST evaluate `decision-tree-v1` trees and `mlp-per-mille-v1` multilayer perceptrons over per-mille-normalized features in checked integer arithmetic, producing one score per declared action. Trees MUST be acyclic, connected and bounded in nodes and depth. MLPs MUST be bounded in layers and width, and their output layer MUST be linear. Admission MUST compute each artifact's worst-case operations. Overflow MUST be a refusal, never a wrap or a clamp.

#### Scenario: A tree with a shared child is refused

- GIVEN a tree in which one node has two parents
- WHEN the artifact is admitted
- THEN admission fails with `Model`.

### Requirement: Selection stays within the declared eligible actions

r[agent_harness_runtime.frozen_policy.selection] A decision MUST select only an action that the request declared eligible and that the artifact's action schema declares. Equal scores MUST select the earliest declared action. A best eligible score below the declared abstention threshold MUST abstain. A selection MUST NOT be treated as task success or authorization.

#### Scenario: The best action is not eligible

- GIVEN scores whose maximum belongs to an action the request did not declare eligible
- WHEN the request is decided
- THEN the best eligible action is selected.

### Requirement: Policy failures are typed and fall back only as declared

r[agent_harness_runtime.frozen_policy.failure] Every decision that does not select MUST carry one typed failure: `unavailable`, `incompatible-artifact`, `stale-binding`, `malformed-input`, `input-out-of-range`, `operation-budget`, `score-out-of-range` or `abstained`. The leaf MUST check the request's binding of artifact, feature-schema and action-schema identities against the admitted artifact before inference. The worst-case operation count MUST be checked against the request's budget before inference. A request whose own shape is valid and that declares an eligible fallback MUST answer with that fallback and record the failure. A request with a malformed shape MUST be refused without fallback.

#### Scenario: A stale binding falls back before inference

- GIVEN an admitted artifact and a request pinned to another artifact identity with a declared fallback
- WHEN the request is decided
- THEN the outcome is the fallback with `stale-binding`
- AND no inference operation is charged.

### Requirement: Decisions issue bounded receipts that replay without inference

r[agent_harness_runtime.frozen_policy.receipt] Every decision MUST issue an `animus.frozen-policy-receipt.v1` receipt. The receipt MUST bind:
- the requested binding;
- the admitted artifact and model identities and the policy kind;
- the runtime crate, version and artifact API;
- the input and eligible-set identities;
- the raw features and eligible labels;
- the per-action scores;
- the outcome, action, score and failure;
- deterministic cost units.

The receipt identity MUST be the BLAKE3 identity of its canonical body. Re-admitting a retained receipt MUST check its size, schema, bounds, consistency and identity, and MUST return the recorded outcome without re-running inference.

#### Scenario: A tampered receipt is refused

- GIVEN a retained receipt whose action was changed
- WHEN it is re-admitted
- THEN it is refused with `Identity`.

### Requirement: The campaign manager evaluates a tree route through the leaf

r[agent_harness_runtime.frozen_policy.campaign_route] The evaluation lane MUST provide a route that decides each listed candidate's class through the leaf and maps the classes `follow-signal`, `explore` and `exploit` to ranks. The route MUST add no offers, MUST retain each candidate's leaf receipt identity, and MUST answer any refused brief with `explore-only`'s reply under a recorded disposition, including `stale` and `abstained`.

#### Scenario: The leading family is exploited

- GIVEN a brief whose leading family shows a signal above 6 and holds an unexecuted member
- WHEN the tree route answers
- THEN that member receives the exploit rank and executed candidates follow their observed signal
- AND no offer is added.

### Requirement: The tree route is compared under a predeclared protocol

r[agent_harness_runtime.frozen_policy.evaluation] The lane MUST load the tree route's arms, seeds, task set, outcomes and adoption rule from the checked Nickel protocol `animus.campaign-manager-frozen-policy-eval.v1` before running. `--frozen-policy` MUST report per-arm completion, rewards, dispositions, cost units, leaf receipt counts and latency, and MUST state the go/no-go decision with null or regression results unchanged.

#### Scenario: The comparison reports its decision

- GIVEN the embedded protocol and tree fixture
- WHEN `--frozen-policy` runs
- THEN the report states the decision and holds no prompt, reply or label.

### Requirement: The frozen-policy boundary is verified

r[agent_harness_runtime.frozen_policy.verification] The change MUST include positive and negative tests for admission, both model kinds, selection, every failure, receipts, the campaign route and the protocol. It MUST pass formatting, Clippy, the crate tests, a `wasm32-unknown-unknown` check of the leaf, the evaluator checks, Octet, the Nickel export parity and Cairn validation.

#### Scenario: The route completes real manager trials

- GIVEN the embedded task set
- WHEN the tree route runs every task through the manager
- THEN every trial completes and every brief is scored without fallback.
