## Context

The accepted MCP bridge validates trusted server and tool descriptors. It maps them into Animus-owned tool definitions and safe receipt candidates.

The bridge intentionally leaves configuration, process launch, transport, authentication, discovery, clocks, cancellation, and teardown to hosts.

This change adds the first concrete host. It supports one exact standard-input protocol cohort. Streamable HTTP remains a later change.

Autolith provides reference failure behavior. Animus keeps its existing tool, authority, credential, request-context, and evidence owners.

## Decisions

### Decision: use a separate non-default host crate

**Choice:** Create `animus-std-mcp` with no default workspace feature. Its functional core validates configuration and lifecycle observations.

A thin shell owns process and standard-input transport effects. Default `animus-std`, no-std, Wasm, provider, TUI, and unrelated profiles do not acquire MCP dependencies.

**Rationale:** MCP process authority must not leak into reusable or default crates.

### Decision: author configuration in typed Nickel

**Choice:** Configuration names an exact server ID, required or optional posture, standard-input executable descriptor, explicit arguments, environment-key references, working-root capability, protocol cohort, initialization limits, discovery limits, call limits, cancellation limits, and policy identities.

Nickel exports deterministic runtime data. Rust revalidates it before process launch. Runtime code never evaluates Nickel.

**Rationale:** Typed source and strict runtime validation prevent ambient discovery and fail-open defaults.

### Decision: support standard-input transport only in this change

**Choice:** The host starts the exact reviewed executable without a shell. It owns one framed standard-input and standard-output session under named bounds.

Unknown transports, Streamable HTTP, server-sent events, remote catalogs, and dynamic transport fallback return explicit unsupported outcomes.

**Rationale:** A narrow first cohort is easier to bound, cancel, and validate.

### Decision: make lifecycle transitions explicit

**Choice:** The pure lifecycle uses `Configured`, `Starting`, `Initializing`, `Discovering`, `Ready`, `Calling`, `Cancelling`, `Stopping`, `Stopped`, and `Failed` states.

Every observation binds server, process-attempt, transport, protocol, credential-snapshot, configuration, and policy identities.

Required-server failure blocks host readiness. Optional-server failure remains an explicit unavailable catalog entry.

**Rationale:** A Boolean connected flag cannot distinguish startup, stale credentials, cancellation, or teardown uncertainty.

### Decision: bound discovery before tool advertisement

**Choice:** The host admits finite tool count, schema bytes, description bytes, resource metadata, and diagnostic limits before mapping.

Each discovered descriptor passes the existing MCP bridge. Each mapped definition then enters the trusted tool catalog and normal executor admission.

Duplicate names, unstable schema identities, unknown capabilities, malformed descriptors, and over-budget catalogs fail closed.

**Rationale:** Server discovery is untrusted input, not authority.

### Decision: intersect authority at every call

**Choice:** A call binds the exact server generation, mapped tool definition, schema identity, model invocation plan, runtime authority seal, current approval, capability grant, mutation plan, and output limits.

Child or delegated agents receive the intersection of their own grants and the parent MCP grant. They cannot inherit the whole server catalog.

**Rationale:** Registration and discovery cannot authorize later effects.

### Decision: treat server instructions as untrusted request context

**Choice:** Server instructions can enter only through the request-context contribution reducer.

They use an advisory, untrusted, current-request lifetime. They cannot supersede mandatory policy or survive into transcript, compaction, memory, or receipts.

**Rationale:** MCP instructions can influence a model and need the same prompt authority controls as other external context.

### Decision: bind credential snapshots and restart on drift

**Choice:** Configuration carries credential references only. SecretSpec or another caller-owned resolver supplies material for the current operation.

The lifecycle binds a safe credential-snapshot identity. A changed identity makes the current server generation stale and requires owned teardown before restart.

No credential material enters arguments, diagnostics, receipts, or persistent transport traces.

**Rationale:** A long-lived server must not silently use stale or mixed credentials.

### Decision: require owned cancellation and teardown evidence

**Choice:** Cancellation first marks the call generation as cancelling. The shell closes request flow and terminates the owned server process tree under named limits.

A timeout or uncertain teardown blocks restart for that server identity until the host supplies terminal process evidence.

**Rationale:** Starting another server while the old process survives can duplicate effects and credentials.

### Decision: keep receipts metadata-only

**Choice:** Receipts bind server, configuration, protocol, process-attempt, tool, schema, capability, approval, call, cancellation, output-bound, redaction, and BLAKE3 identities.

They reject raw JSON-RPC frames, arguments, results, instructions, credentials, environment values, executable paths, and unrestricted errors.

**Rationale:** MCP evidence must not become a transport log.

## Initial Evidence Ladder

1. Pure configuration and lifecycle tests.
2. In-process fake transport tests for framing and state transitions.
3. Fixture child process for standard-input initialization, discovery, call, cancellation, and teardown.
4. Packaged non-default smoke with no network and no live credentials.
5. Optional live server evidence under a later, exact server profile.

## Alternatives Considered

### Add MCP behavior to `animus-std`

Rejected because default std users must not acquire process, credential, or protocol dependencies.

### Support standard-input and HTTP together

Rejected because their lifecycle, cancellation, credentials, and network claims differ.

### Trust all discovered tools from a configured server

Rejected because server trust does not replace per-tool schema, capability, approval, and output admission.

### Persist raw frames for debugging

Rejected because frames can contain credentials, private paths, prompts, arguments, and results.

## Risks and Trade-offs

- Standard-input only limits interoperability. It keeps the first operational host bounded.
- Required servers can block startup. This behavior is explicit configuration, not fallback.
- Process teardown can become uncertain. The host must block restart rather than assume termination.

## Non-Claims

This design does not prove server trust, server correctness, tool safety, sandboxing, network isolation, credential freshness outside supplied evidence, exactly-once effects, MCP parity, production readiness, or release eligibility.
