# Outcome-machine Lifecycle Delta

## ADDED Requirements

### Requirement: Animus owns outcome-machine lifecycle admission

r[animus.lifecycle.outcome_machine_profile] Animus MUST pin the published `outcome-machine` Cairn revision. Profile-selected changes MUST retain BLAKE3-bound rejection and completion evidence without transferring agent-completion or release authority.

#### Scenario: Planned agent change attempts sync

- GIVEN an Animus change has a complete delivery plan
- AND repository completion evidence remains planned
- WHEN the repository-owned Cairn runner evaluates sync
- THEN sync MUST remain blocked
- AND model output or process exit MUST NOT override that result

#### Scenario: Repository evidence completes admission

- GIVEN the authoritative Animus completion rail passes
- AND the change binds that observation with fresh-state evidence
- WHEN Cairn reevaluates the change
- THEN every BLAKE3 evidence identity MUST match
- AND Cairn MUST NOT claim Animus release eligibility
