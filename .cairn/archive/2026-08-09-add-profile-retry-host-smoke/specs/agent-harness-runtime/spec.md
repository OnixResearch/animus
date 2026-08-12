# agent-harness-runtime Change Spec: Profile-bound provider-retry host smoke

## ADDED Requirements

### Requirement: Packaged profile-retry smoke composes accepted contracts

r[agent_harness_runtime.profile_retry_host_smoke.contract] Animus MUST provide a non-default packaged smoke that loads accepted model-profile and provider-retry policy projections and uses their existing runtime and host contracts.

#### Scenario: Explicit inputs build the bounded smoke

r[agent_harness_runtime.profile_retry_host_smoke.contract.explicit]
- GIVEN an operator supplies explicit profile policy, retry policy, provider, model, prompt, and failure-class inputs
- WHEN the packaged smoke validates its configuration
- THEN it MUST load both policy projections without ambient discovery
- AND it MUST use injected deterministic timer and model services without network or credential access.

#### Scenario: Invalid configuration fails before effects

r[agent_harness_runtime.profile_retry_host_smoke.contract.failure]
- GIVEN a required argument or policy projection is missing, malformed, stale, unsupported, or unsafe
- WHEN smoke admission runs
- THEN it MUST return a stable bounded failure before timer or model dispatch
- AND it MUST NOT print raw policy content, private paths, prompts, or responses.

### Requirement: Packaged retry binds one exact loaded profile

r[agent_harness_runtime.profile_retry_host_smoke.binding] The packaged smoke MUST bind the exact resolved profile identity and immutable request identities into one same-target retry invocation before dispatch.

#### Scenario: Retryable failure dispatches the exact request once

r[agent_harness_runtime.profile_retry_host_smoke.binding.pass]
- GIVEN the loaded profile resolves the requested target and policy classifies a pre-commit failure as retryable
- WHEN the correlated fake timer completes
- THEN the smoke MUST dispatch the exact retained model request once through the injected adapter
- AND the invocation MUST bind the resolved profile identity, prompt, tool catalog, response format, reasoning intent, stream intent, credential class, and retry policy.

#### Scenario: Drift and terminal paths fail closed

r[agent_harness_runtime.profile_retry_host_smoke.binding.failure]
- GIVEN the profile is missing, the request changes, the failure is terminal, the timer is cancelled, or the model service is absent
- WHEN retry admission or dispatch runs
- THEN the smoke MUST not claim a completed retry dispatch
- AND the affected timer or model call count MUST remain within the admitted bound.

### Requirement: Packaged profile-retry evidence is metadata-only

r[agent_harness_runtime.profile_retry_host_smoke.receipt] The packaged smoke MUST emit bounded metadata-only profile, retry, dispatch, and receipt records with BLAKE3 identities, explicit non-claims, and current-head metadata when Git is available.

#### Scenario: Successful smoke output omits model content

r[agent_harness_runtime.profile_retry_host_smoke.receipt.safe]
- GIVEN one exact fake retry dispatch succeeds
- WHEN the smoke renders its result
- THEN it MUST include profile, resolution, policy, invocation, decision, request, and receipt identities plus timer and dispatch counts
- AND it MUST omit the raw prompt, model response, policy documents, private paths, credentials, and provider diagnostics.

#### Scenario: Focused receipt binds source and current head

r[agent_harness_runtime.profile_retry_host_smoke.receipt.current_head]
- GIVEN the focused rail passes in a Git worktree
- WHEN the rail writes its receipt
- THEN it MUST record the tested source BLAKE3 identity, current commit, and dirty status
- AND a receipt from a dirty worktree MUST NOT be described as clean current-head evidence.

#### Scenario: Evidence keeps stronger behavior as non-claims

r[agent_harness_runtime.profile_retry_host_smoke.receipt.non_claims]
- GIVEN the deterministic packaged smoke passes
- WHEN dogfood readiness documentation records the evidence
- THEN it MAY claim packaged profile loading and fake-backed retry execution
- AND it MUST retain live-provider reliability, fallback, streaming, persistence, TUI, daemon, and production behavior as non-claims.

### Requirement: Packaged profile-retry smoke has focused verification

r[agent_harness_runtime.profile_retry_host_smoke.verification] The change MUST include positive and negative tests for argument parsing, explicit policy loading, exact profile binding, retry classification, timer gating, request drift, service absence, receipt projection, and content omission.

#### Scenario: Focused rail covers successful and rejected paths

r[agent_harness_runtime.profile_retry_host_smoke.verification.rail]
- GIVEN the repository contains the packaged smoke and its fixtures
- WHEN the focused script and Nix check run
- THEN they MUST exercise one successful retry and the required negative paths
- AND they MUST run without provider credentials or network access.

### Requirement: Profile-retry smoke closeout is evidence-bound

r[agent_harness_runtime.profile_retry_host_smoke.closeout] Maintainers MUST run the focused profile, retry, packaged-smoke, Nix, and Cairn checks before they archive the change.

#### Scenario: Closeout records bounded evidence

r[agent_harness_runtime.profile_retry_host_smoke.closeout.pass]
- GIVEN implementation, tests, docs, and lifecycle artifacts are complete
- WHEN closeout validation runs
- THEN the selected focused checks and Cairn gates MUST pass
- AND the evidence MUST state that no live provider or production retry behavior was exercised.
