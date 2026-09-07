# Process execution authority mapping

## ADDED Requirements

### Requirement: Exact execution mapping
r[agent_harness_runtime.process_authority.mapping] The optional Basalt adapter MUST map only Exec/process-execute/process/execute to ProcessExecute and MUST use the owner's exact binder and operation-profile identity. Existing document mappings SHALL remain separate.

#### Scenario: Exact invocation
- GIVEN an admitted complete context resource and exact execution profile
- WHEN the adapter translates the request
- THEN it selects the owner's execution action without running a process

#### Scenario: Wrong purpose or malformed context
- GIVEN a document grant, wrong action/profile, or malformed execution resource
- WHEN execution admission runs
- THEN it denies without dispatch

### Requirement: Explicit execution policy
r[agent_harness_runtime.process_authority.policy] Exec MUST require the separately reviewed execution-policy identity. Document mappings MUST retain their default policy identity. Neither reference nor receipt SHALL prove policy authenticity or context custody.

#### Scenario: Default policy is not execution authority
- GIVEN an execution request with the document policy identity
- WHEN preflight runs
- THEN it denies before provider verification or effects

### Requirement: Qualified cohort and retained gates
r[agent_harness_runtime.process_authority.cohort] Animus MUST qualify the published pinned Basalt source for its adapter and optional root reference facade. It SHALL preserve raw verification, revocation, replay, live authority through dispatch, local gates and commit-before-effect. Generic contracts and default dependencies MUST remain independent of Basalt.

#### Scenario: Verified execution callback
- GIVEN signed test authority, current verification facts, passing local gates and successful commit
- WHEN dispatch runs
- THEN one counted effect follows commit with no production execution claim

#### Scenario: Failed final gates
- GIVEN replay, revocation, local denial or failed commit
- WHEN dispatch runs
- THEN no effect occurs and failure evidence grants no authority
