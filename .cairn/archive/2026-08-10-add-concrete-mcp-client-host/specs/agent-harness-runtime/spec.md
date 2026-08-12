# Agent Harness Runtime

## ADDED Requirements

### Requirement: The MCP client host is separate, non-default, and authority-scoped

r[agent_harness_runtime.mcp_client_host.boundary] Animus MUST provide the first concrete MCP client host as a separate non-default std crate with a pure configuration and lifecycle core plus a thin process and transport shell. Reusable and default crates MUST NOT acquire MCP process, network, credential, clock, or persistence authority.

#### Scenario: MCP host is explicitly selected

r[agent_harness_runtime.mcp_client_host.boundary.selected]
- GIVEN an embedder enables the MCP host and supplies explicit configuration, process, credential, cancellation, and policy services
- WHEN host construction runs
- THEN Animus MAY construct the selected host for the reviewed cohort
- AND unrelated default, no-std, Wasm, provider, TUI, and std profiles MUST remain free of the MCP host dependency.

#### Scenario: Ambient MCP runtime is expected

r[agent_harness_runtime.mcp_client_host.boundary.ambient]
- GIVEN a caller expects reusable runtime or default std code to discover, launch, or trust MCP servers
- WHEN MCP admission runs
- THEN Animus MUST reject before process, network, credential, or filesystem effects
- AND it MUST NOT substitute a default server or transport.

### Requirement: MCP host configuration is typed, exact, and bounded

r[agent_harness_runtime.mcp_client_host.configuration] Human-authored MCP host configuration MUST use typed Nickel contracts for exact server identity, required or optional posture, standard-input executable descriptor, arguments, environment-key references, capability root, protocol cohort, finite limits, credential references, policy, diagnostics, and non-claims. Rust MUST revalidate the deterministic generated projection before launch.

#### Scenario: Complete server configuration is admitted

r[agent_harness_runtime.mcp_client_host.configuration.valid]
- GIVEN generated configuration names one reviewed standard-input cohort with complete exact fields and positive named limits
- WHEN Rust validation runs
- THEN Animus MUST return one canonical configuration and domain-separated BLAKE3 identity
- AND process launch and later lifecycle plans MUST bind that exact identity.

#### Scenario: Configuration is unsafe or incomplete

r[agent_harness_runtime.mcp_client_host.configuration.invalid]
- GIVEN configuration enables auto-discovery, unknown transport, shell fallback, unrestricted environment inheritance, raw credential value, unbounded schema, missing executable identity, or weakened non-claims
- WHEN Nickel or Rust validation runs
- THEN validation MUST fail before process launch
- AND no partial required or optional server state may be advertised as ready.

### Requirement: Standard-input server lifecycle is explicit and generation-bound

r[agent_harness_runtime.mcp_client_host.lifecycle] The first MCP host MUST support only one exact standard-input protocol cohort with explicit configured, starting, initializing, discovering, ready, calling, cancelling, stopping, stopped, and failed states bound to server, process-attempt, transport, protocol, credential-snapshot, configuration, and policy identities.

#### Scenario: Required server becomes ready

r[agent_harness_runtime.mcp_client_host.lifecycle.required]
- GIVEN a required server starts, completes bounded initialization, and completes bounded discovery under exact current identities
- WHEN lifecycle reduction runs
- THEN the server MAY enter ready state for that generation
- AND host readiness MUST remain blocked until every required server reaches an admitted terminal readiness outcome.

#### Scenario: Optional server fails

r[agent_harness_runtime.mcp_client_host.lifecycle.optional]
- GIVEN an optional server fails startup, initialization, discovery, or teardown
- WHEN lifecycle reduction runs
- THEN the host MAY continue with an explicit unavailable optional-server entry
- AND it MUST NOT substitute another server, tool, transport, or stale generation.

#### Scenario: Credential snapshot changes

r[agent_harness_runtime.mcp_client_host.lifecycle.credential_drift]
- GIVEN the safe credential-snapshot identity changes while a server generation is live
- WHEN lifecycle admission runs
- THEN the current generation MUST become stale and stop accepting new calls
- AND restart MUST remain blocked until terminal evidence covers the owned old process tree.

#### Scenario: Call is cancelled

r[agent_harness_runtime.mcp_client_host.lifecycle.cancelled]
- GIVEN a current call receives valid cancellation or exceeds its named deadline
- WHEN cancellation runs
- THEN the lifecycle MUST enter cancelling and request owned teardown under policy
- AND uncertain or incomplete process termination MUST block same-server restart.

### Requirement: Discovery is bounded and maps through existing MCP contracts

r[agent_harness_runtime.mcp_client_host.discovery] Tool discovery MUST enforce finite tool, name, description, schema, resource-metadata, and diagnostic limits before each descriptor passes existing MCP mapping and trusted tool-catalog validation.

#### Scenario: Bounded tool catalog is discovered

r[agent_harness_runtime.mcp_client_host.discovery.valid]
- GIVEN a ready server returns a finite set of well-formed tool descriptors under all configured limits
- WHEN discovery validation and existing MCP mapping run
- THEN only complete mapped Animus tool definitions MAY enter the trusted catalog
- AND each definition MUST bind the exact server generation and schema identity.

#### Scenario: Discovery is malformed or over budget

r[agent_harness_runtime.mcp_client_host.discovery.invalid]
- GIVEN discovery returns duplicate names, schema drift, unknown capability, malformed descriptor, excessive count, excessive bytes, or unrestricted diagnostics
- WHEN validation runs
- THEN discovery MUST fail closed for that generation
- AND no partial or earlier catalog may be advertised as current.

### Requirement: Every MCP call passes current Animus authority

r[agent_harness_runtime.mcp_client_host.authority] An MCP call MUST bind the current server generation, mapped tool definition, schema identity, model invocation plan, runtime authority seal, approval, capability grant, mutation plan, cancellation state, and output bounds before transport dispatch. Delegated callers MUST receive only the intersection of parent and caller grants.

#### Scenario: Current call is admitted

r[agent_harness_runtime.mcp_client_host.authority.allowed]
- GIVEN all exact bindings, current authority observations, approvals, capabilities, mutation rules, executor readiness, and output bounds pass
- WHEN call admission runs
- THEN the host MAY dispatch one bounded call to the exact server generation
- AND the returned result MUST pass existing malformed, size, leak, and output validation before model visibility.

#### Scenario: Call authority is stale or wider than grants

r[agent_harness_runtime.mcp_client_host.authority.denied]
- GIVEN a call has missing approval, denied capability, stale model plan, stale server generation, schema mismatch, mutation conflict, child-grant expansion, cancellation, or over-budget output
- WHEN admission runs
- THEN Animus MUST deny before transport dispatch or model visibility
- AND server registration or discovery MUST NOT substitute for current authority.

### Requirement: MCP instructions are untrusted request-local context

r[agent_harness_runtime.mcp_client_host.context] Server-supplied instructions MAY enter model context only through the request-context contribution contract as untrusted advisory current-request input. They MUST NOT supersede mandatory policy or enter conversation, compaction, memory, quality, or receipt storage.

#### Scenario: Server instructions are admitted for one request

r[agent_harness_runtime.mcp_client_host.context.request_local]
- GIVEN current server instructions pass size, encoding, source, and payload-identity validation
- WHEN request-context reduction runs
- THEN the instructions MAY compete only as an untrusted advisory current-request contribution
- AND they MUST expire at the request terminal observation.

#### Scenario: Server instructions request durable or mandatory status

r[agent_harness_runtime.mcp_client_host.context.denied]
- GIVEN server instructions request mandatory priority, policy supersession, durable retention, memory generation, or cross-turn lifetime
- WHEN context admission runs
- THEN Animus MUST deny those properties before prompt composition
- AND it MUST NOT promote the instructions because the server or tool is trusted.

### Requirement: MCP host evidence is metadata-only

r[agent_harness_runtime.mcp_client_host.evidence] MCP host receipts MUST contain bounded server, configuration, protocol, generation, process-attempt, tool, schema, capability, approval, call, cancellation, output-bound, lifecycle, redaction, non-claim, and BLAKE3 identity metadata only.

#### Scenario: Safe MCP host receipt is projected

r[agent_harness_runtime.mcp_client_host.evidence.safe]
- GIVEN a lifecycle transition, discovery, call, cancellation, or teardown outcome passes exact validation
- WHEN receipt projection runs
- THEN the receipt MUST bind only the safe supplied facts and mandatory non-claims
- AND it MUST not claim server trust, sandboxing, tool safety, exactly-once effects, or production readiness.

#### Scenario: Receipt contains transport or secret material

r[agent_harness_runtime.mcp_client_host.evidence.rejects]
- GIVEN a receipt candidate contains raw frames, arguments, results, instructions, credentials, environment values, executable or socket paths, provider data, or unrestricted errors
- WHEN receipt validation runs
- THEN Animus MUST reject it without echoing the value
- AND the candidate MUST NOT enter session, completion, lifecycle, or release evidence.

### Requirement: MCP host conformance includes successful and failed servers

r[agent_harness_runtime.mcp_client_host.verification] The change MUST include deterministic positive and negative production-path tests for configuration, lifecycle, framing, discovery, mapping, authority, credentials, cancellation, teardown, context, receipts, feature isolation, and a packaged standard-input smoke.

#### Scenario: Complete MCP host corpus passes

r[agent_harness_runtime.mcp_client_host.verification.complete]
- GIVEN fixture servers cover successful initialization, discovery, call, cancellation, teardown, malformed frames, drift, limits, denial, and process-tree survival
- WHEN focused core, host, child-process, composition, and packaged-smoke checks run
- THEN allowed cases MUST produce exact deterministic outcomes and denied cases MUST fail before prohibited effects or claims
- AND the smoke MUST use no network and no live credential.

#### Scenario: Closeout evidence is incomplete

r[agent_harness_runtime.mcp_client_host.verification.incomplete]
- GIVEN maintainers intend to archive without cohort pinning, license review, malformed-frame, process-tree, credential-drift, authority, leak, feature-isolation, Cairn, or relevant Nix evidence
- WHEN closeout validation runs
- THEN archive readiness MUST remain blocked with the missing evidence or exact unavailable-rail blocker
- AND Streamable HTTP, remote catalogs, server sandboxing, MCP parity, and production readiness MUST remain explicit non-claims.
