## Context

Animus has two relevant neutral surfaces. Process-job profiles normalize program, arguments, working directory, writable paths, environment allowlists, timeout/resource bounds, backend selection, and safe identity evidence before a host dispatches work. Sandlock-inspired descriptors and receipts represent requested filesystem, network, seccomp, workdir, and output constraints while keeping Linux enforcement in `animus-std`.

Octet's Aeneas/Charon/Lean rail independently executes proof stages with explicit argv, cleared allowlisted environments, isolated work roots, bounded output, timeouts, and denied networking. Nickel Export invokes one explicit evaluator over captured declared inputs with evaluator identity, environment removal, process-time/output/resource bounds, and optional replay observations. Mantle owns build sandboxes and a protected-exec supervisor that binds permitted executables to declared inventories.

The vocabulary overlaps, but the claim mechanisms differ. A Bubblewrap mount namespace, a Landlock or seccomp observation, a private evaluator snapshot, and a Mantle build sandbox cannot safely share one `sandboxed = true` result. The evaluation must preserve these differences and accept `incubate` or `blocked` as successful outcomes when a neutral contract would hide them.

## Decisions

### Decision: evaluate before extracting

**Choice:** This Cairn produces a deterministic feasibility receipt and does not create a repository, move public DTOs, add cross-repository dependencies, or change executor behavior. Repository extraction requires a successor Cairn whose scope is bounded by an `extract` result.

**Rationale:** The current evidence proves recurring concepts, not semantic parity. Separating evaluation from extraction prevents architecture momentum from overriding incompatibility findings.

### Decision: use exact reviewed source identities

**Choice:** The inventory records immutable repository revisions, BLAKE3 identities for the reviewed source/docs/contracts, and the exact consumer surface and owner. Workspace sibling paths may be used to collect evidence but never become durable product coordinates.

**Rationale:** Execution contracts evolve quickly. A decision without source identity cannot be replayed or distinguished from later mechanism changes.

### Decision: author the matrix and limits in typed Nickel

**Choice:** A typed Nickel profile owns consumer IDs, semantic classes, capability-state vocabulary, outcome vocabulary, required fixtures, mapping dispositions, extraction criteria, collection bounds, and required non-claims. A checked generated projection feeds a repository-owned Rust evaluator; runtime Animus code does not evaluate Nickel.

**Rationale:** The matrix is reviewable configuration with cross-field invariants. Nickel catches missing owners, duplicate IDs, unknown states, absent negative fixtures, and unbounded collections before decision evaluation.

### Decision: model requested, enforced, and observed facts separately

**Choice:** Capability rows use closed states for requested, configured, enforced, observed, unavailable, unsupported, and not applicable. Each row records mechanism family, enforcement point, observation source, failure behavior, and non-claims. No shared boolean represents sandbox, isolation, hermeticity, or network denial.

**Rationale:** Requests and observations are not enforcement, and enforcement through different kernels or supervisors has different failure modes. Keeping the stages separate prevents claim laundering.

### Decision: normalize outcomes only where terminal meaning agrees

**Choice:** The candidate common vocabulary covers success, nonzero exit, launch denial, missing executable, policy rejection, timeout, cancellation, signal termination, output overflow, cleanup failure, and unsupported mechanism. Consumer detail remains in an explicit extension record. A mapping is lossless only when round-trip evaluation preserves success/failure, cause, retry posture, output completeness, cleanup status, and claim meaning.

**Rationale:** Shared names are useful for orchestration and evidence only when they do not erase why or how execution ended.

### Decision: freeze positive and negative evidence together

**Choice:** Every consumer profile supplies positive representative observations and negative cases for stale source, invalid bounds, environment leakage, path escape, unavailable enforcement, timeout/cancellation ambiguity, output overflow, cleanup uncertainty, unsafe diagnostics, unsupported mechanism claims, and weakened non-claims. Missing negative coverage blocks `extract`.

**Rationale:** A vocabulary that handles only happy paths will drift at exactly the safety boundaries the extraction is intended to stabilize.

### Decision: use a pure deterministic decision core

**Choice:** A repository-owned Rust checker splits into pure parsing/validation/decision functions over loaded matrix values and a thin shell for file reads, generated-projection freshness, and receipt output. The decision receipt binds profile, source, fixture, mapping, dependency, and non-claim identities with BLAKE3 and returns `extract`, `incubate`, or `blocked`.

**Rationale:** The result should be replayable without executing any process or sandbox. A small functional core also permits positive and negative self-tests without mocks.

### Decision: require concrete multi-consumer and release value

**Choice:** `extract` requires at least two independent consumers committed to the same mandatory surface, lossless mandatory mappings, no ambient authority in the candidate core, no product dependency cycle, independently useful release/rollback cadence, complete positive and negative fixtures, and a bounded consumer migration strategy. Conceptual reuse alone is insufficient.

**Rationale:** A repository is an ownership and release boundary, not a catalog of similar nouns. These criteria prevent single-consumer or coordination-only extractions.

### Decision: keep product-specific guarantees outside the candidate core

**Choice:** Animus retains process-job policy and host adapters; Octet retains proof selection, theorem admission, and proof receipts; Nickel Export retains evaluator capture and freshness semantics; Mantle retains build sandbox, protected execution, inventory, build, and release policy. The candidate contract, if admitted later, can own only normalized requests, observations, mechanism capability labels, bounded outcomes, and safe receipt projection.

**Rationale:** Execution authority and correctness claims depend on each product's shell and policy. Moving them would either create a universal sandbox product or weaken existing boundaries.

### Decision: honest blockers are completion

**Choice:** `incubate` is used when the shared surface is promising but lacks a second committed consumer or stable release value. `blocked` is used for semantic loss, cycles, missing evidence, or incompatible guarantees. Both statuses include the next smallest discriminating action and satisfy this feasibility Cairn when all required evidence and tests pass.

**Rationale:** The requested result may not exist. Treating exact blockers as valid prevents forced schemas and preserves future search direction.

## Alternatives Considered

### Extract Animus process-job profiles immediately

Rejected because they include agent-harness policy and queue/backend choices that Octet, Nickel Export, and Mantle do not necessarily share.

### Treat every consumer as an implementation of one sandbox interface

Rejected because evaluator capture, proof-runner isolation, build sandboxes, Landlock, seccomp, and Bubblewrap expose different enforcement and observation guarantees.

### Share only a tiny success/exit-code DTO

Rejected as a repository justification unless the evaluation proves independent release value. A trivial lowest-common-denominator crate can add coordination cost while leaving all meaningful semantics duplicated.

### Put the contract in `host-control`

Rejected because host-control owns topology, selectors, bounded observations, layout, events, and waits, not process execution or sandbox policy. Combining them would broaden its authority and dependency surface.

### Put the contract in Mantle

Rejected because non-build consumers should not depend on a build system, and Mantle's AGPL/product semantics would constrain otherwise neutral consumers.

## Risks / Trade-offs

- The inventory can become stale while the four repositories evolve. Exact identities and a freshness checker make staleness visible but do not eliminate update work.
- A detailed matrix may show only a very small shared surface. `incubate` or `blocked` is preferable to manufacturing a stronger result.
- Consumer fixtures may expose sensitive command or environment content. Fixtures and receipts must use safe synthetic values, bounded diagnostics, and hashes rather than credentials or host paths.
- Mechanism-specific extensions can fragment the candidate contract. The decision core must treat required extensions as extraction blockers when consumers cannot interoperate through the mandatory surface.
- Evaluation adds work without immediate runtime change. Its value is avoiding a costly repository and migration that would otherwise encode false equivalence.
