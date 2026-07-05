# agent-harness-runtime Change Spec: Ornith offline evaluation feedback adaptation

## Requirements

### Requirement: Offline scaffold evaluation plans

r[agent_harness_runtime.ornith_eval_feedback.plan] Animus MUST represent offline scaffold evaluation as deterministic plan data with task-set identity, scaffold revision ID and hash, model route, verifier identity and hash, trust-boundary policy hash, tool-surface hash, sampling profile, resource labels, run count, and output format before execution.

#### Scenario: Invalid evaluation plan fails closed

r[agent_harness_runtime.ornith_eval_feedback.plan.invalid]
- GIVEN an evaluation plan contains a remote task-set location, missing scaffold hash, missing verifier hash, unsupported output format, non-positive run count, or provider-specific public benchmark claim
- WHEN plan validation runs
- THEN validation returns a typed evaluation-plan error
- AND no batch, model, verifier, tool, filesystem, process, or network execution is started.

### Requirement: Redacted evaluation receipts

r[agent_harness_runtime.ornith_eval_feedback.receipts] Scaffold evaluation receipts MUST expose safe comparison metadata such as model ID, scaffold hash, trust-boundary hash, tool-surface hash, verifier hash, context-window class, sampling labels, timeout/resource labels, run count, aggregate status, and redaction state, and MUST NOT include raw prompts, hidden tests, provider payloads, credentials, full paths, verifier bodies, or unbounded tool outputs.

#### Scenario: Evaluation receipt rejects transcript leaks

r[agent_harness_runtime.ornith_eval_feedback.receipts.redacted]
- GIVEN an evaluation receipt fixture contains raw prompt text, hidden test content, provider payloads, credential-looking values, absolute path dumps, verifier bodies, or unbounded tool outputs
- WHEN receipt validation runs
- THEN validation fails with a typed redaction diagnostic
- AND the receipt is not accepted as scaffold comparison evidence.

### Requirement: Reviewed feedback candidates

r[agent_harness_runtime.ornith_eval_feedback.review] Animus MAY produce feedback candidates from offline evaluation, including promote, demote, revise, or investigate recommendations, but feedback MUST require host review before scaffold admission, activation, or runtime selection policy changes.

#### Scenario: Feedback cannot auto-accept scaffold changes

r[agent_harness_runtime.ornith_eval_feedback.review.no_auto_accept]
- GIVEN an offline evaluation produces a promote or revise feedback candidate
- WHEN the host review outcome is missing, denied, unavailable, or stale
- THEN the feedback is recorded as non-authoritative evidence
- AND no scaffold revision is admitted, activated, or selected for future runtime influence.

### Requirement: Evidence freshness policy

r[agent_harness_runtime.ornith_eval_feedback.freshness] Animus MUST classify scaffold evaluation evidence freshness by comparing model route, scaffold hash, verifier hash, trust-boundary hash, tool-surface hash, runtime version label, and sampling profile against current evaluation policy.

#### Scenario: Stale evidence is marked or reduced weight

r[agent_harness_runtime.ornith_eval_feedback.freshness.stale]
- GIVEN an evaluation receipt was produced under an older scaffold hash, verifier hash, trust-boundary hash, tool-surface hash, runtime version label, model route, or sampling profile
- WHEN evidence freshness is evaluated
- THEN the evidence is marked stale or reduced-weight according to policy
- AND stale evidence cannot by itself justify scaffold promotion.

### Requirement: Offline feedback scope

r[agent_harness_runtime.ornith_eval_feedback.scope] Animus offline evaluation feedback MUST remain a local, reviewed evidence workflow and MUST NOT implement RL training, live runtime self-mutation, cloud benchmark runners, hidden-test access, provider-specific eval APIs, or public benchmark claims without a separate change.

#### Scenario: Scope rejects live self-improvement claims

r[agent_harness_runtime.ornith_eval_feedback.scope.live_mutation_rejected]
- GIVEN a feedback workflow requests automatic scaffold activation, hidden-test access, cloud benchmark execution, provider-specific eval submission, or public benchmark publication
- WHEN scope validation evaluates the workflow
- THEN validation rejects the workflow with a typed unsupported-scope diagnostic
- AND no runtime policy, provider, verifier, network, or filesystem effect is invoked.

### Requirement: Evaluation feedback verification

r[agent_harness_runtime.ornith_eval_feedback.verification] The offline evaluation feedback adaptation MUST include focused positive and negative tests for plan validation, receipt redaction, feedback review outcomes, evidence freshness, scope enforcement, workspace checks, roadmap validation, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers evaluation feedback behavior

r[agent_harness_runtime.ornith_eval_feedback.verification.closeout]
- GIVEN maintainers intend to archive the evaluation feedback adaptation
- WHEN closeout validation runs
- THEN evidence names focused plan, receipt, feedback, freshness, scope, workspace, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
