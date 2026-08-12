# agent-harness-runtime Change Spec: Cairn policy validation wiring

## Requirements

### Requirement: Generated policy artifact

r[agent_harness_runtime.cairn_policy.generated_artifact] Animus MUST provide repo-local Cairn generated policy data at the path expected by default Cairn runtime validation, or an equivalent repo-owned wrapper that resolves the policy without hidden session-specific absolute paths.

#### Scenario: Default validation finds policy

r[agent_harness_runtime.cairn_policy.generated_artifact.default_path]
- GIVEN a developer runs Cairn validation from the Animus repository root
- WHEN no explicit external policy path is supplied
- THEN validation loads the repo-owned policy artifact or wrapper-resolved policy
- AND it does not depend on a private checkout path.

### Requirement: Policy refresh documentation

r[agent_harness_runtime.cairn_policy.refresh_docs] Animus MUST document the policy source, refresh command, check command, and expected default validation command for Cairn lifecycle artifacts.

#### Scenario: Maintainer refreshes policy intentionally

r[agent_harness_runtime.cairn_policy.refresh_docs.reproducible]
- GIVEN the canonical Cairn policy source changes
- WHEN a maintainer refreshes Animus policy wiring
- THEN the documentation names the source identity and commands used
- AND the generated artifact can be reviewed as an intentional lifecycle dependency update.

### Requirement: Policy drift check

r[agent_harness_runtime.cairn_policy.drift_check] Animus SHOULD provide a deterministic policy drift check or documented fixture that compares the checked-in generated policy to the canonical exported policy when the source is available.

#### Scenario: Stale policy is detected

r[agent_harness_runtime.cairn_policy.drift_check.stale]
- GIVEN the checked-in generated policy differs from the canonical exported policy
- WHEN the drift check runs with the source available
- THEN the check fails with a deterministic diagnostic
- AND it reports the source identity without embedding secrets or local-only full paths in durable evidence.

### Requirement: Default Cairn validation

r[agent_harness_runtime.cairn_policy.default_validation] Animus lifecycle validation MUST be runnable through the default Cairn validation command from the repository root without requiring a developer to remember an external policy argument.

#### Scenario: Validation is local and reproducible

r[agent_harness_runtime.cairn_policy.default_validation.local]
- GIVEN the repository contains lifecycle artifacts and policy wiring
- WHEN `cairn validate --root .` or the pinned equivalent runs
- THEN validation succeeds or fails only on lifecycle/policy content
- AND missing external checkout paths are not reported as validation blockers.

### Requirement: Policy evidence

r[agent_harness_runtime.cairn_policy.evidence] Cairn policy validation evidence SHOULD record policy source identity, policy hash when available, validation command, and validation result.

#### Scenario: Reviewers can audit policy used for validation

r[agent_harness_runtime.cairn_policy.evidence.reviewable]
- GIVEN lifecycle validation evidence is attached to a change
- WHEN reviewers inspect it
- THEN the evidence identifies which policy artifact was used
- AND it includes enough stable identity to reproduce or challenge the validation result.

### Requirement: Policy wiring closeout

r[agent_harness_runtime.cairn_policy.closeout] Policy wiring work MUST run default validation, explicit-policy validation when available, and Cairn proposal/design/tasks gates before archive.

#### Scenario: Policy wiring closeout is reviewable

r[agent_harness_runtime.cairn_policy.closeout.evidence]
- GIVEN the policy wiring change is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN default validation, explicit-policy validation, and relevant Cairn gates are named with their status
- AND unavailable checks include a blocker and next best validation path.
