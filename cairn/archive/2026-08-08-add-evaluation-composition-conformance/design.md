## Context

Animus has many local evaluation-related checks, but no single versioned suite verifies the proposed native evaluation architecture from task profile through safe output. The core, adapter, and host changes intentionally split task and episode planning, effects, protocol capture, verifier execution, persistence, and evidence. Their composition introduces failure modes that isolated crate tests cannot detect.

The conformance suite is evaluation-only. It introduces no runtime behavior. It uses deterministic fake services and public production APIs to check exact identities, state transitions, failure classes, redaction, and receipts. The reviewed Verifiers cohort supplies scenario inspiration for best-of-N, user simulation, isolated judging, model-call interception, streaming commit order, and content-addressed resume. Animus fixtures remain repository-owned and enforce stronger authority and privacy rules.

Completion requires a declared matrix, public-entrypoint fixture harness, positive and negative cases, exact expected diagnostics, dependency guards, focused and Nix rails, and an aggregate evidence summary. Calling private helpers, recreating reductions in test code, checking only trait invocation counts, or skipping negative paths is false completion.

## Decisions

### Decision: define one versioned conformance matrix

**Choice:** A typed Nickel profile names the exact native evaluation schema, selected crate and feature cohort, fixture families, required public entrypoints, service catalog, bounds, expected outcomes, redaction, evidence, and non-claims. Generated data is revalidated by Rust.

**Rationale:** A versioned matrix makes coverage and drift reviewable without embedding fixture policy in ad hoc test code.

### Decision: test public production paths only

**Choice:** Fixtures create checked native plans, invoke public adapter admission, execute the public fake-backed host, reduce through public core APIs, and validate public receipts and downstream projections. Test helpers can construct data but cannot implement alternate planning, resume, reward, gateway, or receipt semantics.

**Rationale:** The suite must fail when production wiring breaks, not only when fixture code changes.

### Decision: organize scenarios by mechanism

**Choice:** The matrix has families for:

- task and taskset identity;
- single-agent, best-of-N, user-sim, and isolated-judge recipes;
- ledger and episode correlation;
- service and capability binding;
- gateway request, retry, stream, and commit ordering;
- verifier and artifact isolation;
- reward and metric reduction;
- persistence and resume;
- Harbor and ATIF compatibility;
- evaluation-feedback and dataset projections;
- receipts, redaction, and claims.

**Rationale:** Mechanism families prevent duplicate scenario wording from hiding missing coverage.

### Decision: pair every positive family with adversarial negatives

**Choice:** Each mechanism family has at least one success fixture and one failure fixture. Negative fixtures assert the exact diagnostic class, no prohibited service call, unchanged reusable state, and no success receipt.

**Rationale:** Evaluation failures are often authority or stale-evidence failures. Happy-path coverage alone is insufficient.

### Decision: use deterministic fake services, not behavioral mocks

**Choice:** Fakes implement public adapter traits and return scripted typed observations. They record bounded request identities and state transitions. Assertions check outputs and unchanged-state properties rather than only call counts.

**Rationale:** Fakes exercise contracts without network, process, provider, or storage dependencies. Call counts alone do not establish correct binding or state.

### Decision: keep live checks separate and non-authoritative

**Choice:** The conformance rail uses no live provider, tool, process, container, cloud, network, credential, or benchmark. A later live smoke can cite this suite as contract evidence but cannot upgrade it into production evidence.

**Rationale:** Deterministic CI and live integration answer different questions and have different authority.

### Decision: emit a bounded evidence summary

**Choice:** The rail reports profile and fixture identities, family counts, pass/fail/block status, expected diagnostic classes, public entrypoint coverage, dependency state, redaction state, and non-claims. It excludes task bodies, transcripts, hidden tests, provider/tool bodies, credentials, paths, and logs.

**Rationale:** Lifecycle review needs compact evidence, not copied test data.

## Required Scenario Matrix

| Family | Positive | Negative |
|---|---|---|
| Taskset | exact and bounded lazy selection | unstable generator, missing bound, unsafe shuffle |
| Recipes | single, best-of-N, user-sim, isolated judge | role widening, fan-out overflow, shared promotion |
| Episode | exact ordered ledger references | copied transcript, stale branch, wrong subject |
| Ports | matching typed observations | absent service, wrong plan, wrong attempt |
| Gateway | exact retry and terminal-after-commit | early terminal, changed retry, late write |
| Verifier | separate service and safe artifacts | shared mutation, symlink, collision, hidden test |
| Rewards | finite attributed reduction | NaN, duplicate, mismatch, silent clamp |
| Persistence | atomic reopen and exact resume | torn row, stale policy, failed cleanup |
| Compatibility | native-to-Harbor and ATIF safe subset | lossy identity, unsupported version, leak |
| Downstream | advisory feedback and admitted dataset draft | automatic promotion, unsafe content export |
| Evidence | metadata-only receipt | raw content and overclaim rejection |

## Alternatives Considered

### Keep checks inside each change only

Rejected because no check would own cross-crate lifecycle and identity coherence.

### Add a second test evaluator

Rejected because duplicate logic can agree with fixtures while production behavior drifts.

### Require live model evaluation

Rejected because live providers add credentials, network, cost, nondeterminism, and claims that this contract suite cannot support.

### Use snapshots of full traces

Rejected because full traces can contain prompts, hidden tests, reasoning, tool bodies, provider payloads, credentials, and paths.

## Risks / Trade-offs

- The matrix adds maintenance whenever public evaluation schemas change.
- Strict expected diagnostics can require deliberate fixture updates after safe error-taxonomy changes.
- Fake services cannot prove external implementation behavior.
- Broad composition checks can run longer than focused crate tests, so the focused rail must report family-level progress.

## Non-Claims

This design does not prove live host behavior, provider or tool correctness, sandboxing, network enforcement, storage durability, exactly-once effects, verifier soundness, benchmark validity, model quality, training fitness, production readiness, authoritative completion, or release eligibility.
