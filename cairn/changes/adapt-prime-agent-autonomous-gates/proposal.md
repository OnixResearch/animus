## Why

Prime Agent's autonomous mode injects follow-up continuations until configured quality gates pass or a continuation, turn, token, or wall-clock limit is reached. Two mechanisms carry the safety load. First, a failed gate returns its bounded output to the agent for another attempt. Second, the same failed gate does not rerun when the workspace has not changed. Animus has `controller-continuation-policy` and `ralph-loop`, but no gate concept and no change detection; a loop can burn budget rerunning a gate that cannot pass.

Animus needs gates as host-injected executors with bounded output, a workspace fingerprint that makes rerun decisions deterministic, and the explicit rule that passing a gate or reaching a limit never implies task success.

## What Changes

- Add bounded autonomous continuation with explicit continuation, turn, token, and wall-clock limits as an extension of the existing continuation policy. r[agent_harness_runtime.autonomous_gates.continuation]
- Add quality gates as host-injected executors; a failed gate returns bounded output to the agent for another attempt. r[agent_harness_runtime.autonomous_gates.gates]
- Add workspace change detection: a domain-separated BLAKE3 fingerprint over the admitted file set decides whether a failed gate may rerun. r[agent_harness_runtime.autonomous_gates.change_detection]
- Add the success-boundary rule: passing a gate verifies only what that gate checks, and reaching a limit is never task success. r[agent_harness_runtime.autonomous_gates.no_success_claim]
- Add a Nickel-authored autonomous policy for limits, gate descriptors, fingerprint inputs, and non-claims. r[agent_harness_runtime.autonomous_gates.profile]
- Add metadata-only gate and continuation receipts. r[agent_harness_runtime.autonomous_gates.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.autonomous_gates.verification]

## Impact

- **Files**: new focused autonomous-gate contract module, Nickel policy, fixtures, focused check script, docs; composition seams in `controller-continuation-policy` and `ralph-loop`.
- **Reusable boundary**: gate execution, fingerprinting over real files, and clock access stay host-owned; the contract consumes typed results and fingerprints.
- **Runtime composition**: `controller-continuation-policy` gains gate outcomes and fingerprints as typed inputs; ralph-loop profiles may bind gates without changing their completion-promise semantics.
- **Testing**: positive fixtures for gate pass, bounded retry, and fingerprint skip; negative fixtures for limit-as-success confusion, unbounded output, and stale fingerprints; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Executing gate commands inside reusable crates, or granting the agent new shell authority.
- Treating gate passage, limit exhaustion, or continuation count as task completion or quality proof.
- Defining repo-specific gate suites; gates remain host-supplied descriptors.
- Proving change detection completeness outside the admitted file set, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: autonomous-gate boundary, policy, continuation, gates, change detection, success boundary, evidence, and verification.
