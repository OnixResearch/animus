## Context

The accepted ACP/MCP adaptation describes data-only external-agent descriptors and capability intersections. This change has the opposite transport direction: it exposes an admitted Animus coding-session host as an ACP agent process to a caller that explicitly launches it. The coding-session facade owns session state and events, the confirmation broker owns approval correlation, the tool executor owns final tool admission, and the durable authority profile owns resume drift checks.

## Decisions

### 1. Use a focused optional host crate

**Choice:** Add a non-default ACP host crate or focused std authority block that pins one reviewed ACP protocol/library cohort. `animus-core`, `animus-adapters`, `animus-runtime`, default `animus-std`, and Wasm/no-std profiles do not acquire the protocol transport dependency.

**Rationale:** ACP is a product integration, not a requirement for every Animus embedder.

### 2. Split protocol reduction from transport

**Choice:** A deterministic reducer consumes validated decoded ACP requests, explicit connection/session state, coding-facade observations, and typed host outcomes, and returns next protocol state, facade commands, confirmation requests, ordered ACP updates, and transport actions. It does not read stdin, write stdout, spawn work, access clocks, inspect environment, or contact a provider/tool directly.

**Rationale:** Correlation, ordering, capability negotiation, and lifecycle behavior should be testable without an IDE process or live stdio timing.

### 3. Keep stdio as the only initial transport

**Choice:** The imperative shell owns bounded stdin reads, ACP framing/decoding, stdout writes, flushing, backpressure, cancellation/deadline observation, EOF and broken-pipe classification, safe stderr diagnostics, and cleanup. Launch configuration is explicit; no ambient ACP client/server discovery, socket listener, WebSocket, HTTP, TLS, endpoint lookup, or network fallback is provided.

**Rationale:** Local stdio gives a useful editor integration with the smallest transport and trust surface.

### 4. Make the coding-session facade authoritative

**Choice:** ACP initialize negotiates only implemented capabilities. Session creation, loading, prompting, continuation, and cancellation map to facade commands and correlated outcomes. The ACP adapter keeps transport correlation and projection state only; it does not own transcript entries, active leaf, queue policy, tool state, run status, or durable commits.

**Rationale:** A protocol adapter must not create a second lifecycle state machine that can diverge from headless or TUI behavior.

### 5. Route permissions through existing authority

**Choice:** ACP permission requests and client responses bridge to the existing permission kernel, confirmation broker, Animus/Basalt authorization, sandbox requirements, and tool executor. Effective tools remain the intersection of session profile, host admission, active permission mode, external/client declarations, and executor readiness. An ACP allow response cannot turn a denied or unavailable action into an admitted effect.

**Rationale:** User-interface approval is not ambient capability or enforcement proof.

### 6. Project bounded events with explicit visibility

**Choice:** Existing facade `RuntimeEvent` values are mapped into ordered ACP session updates under named buffer/count/byte policies. Assistant-visible content, tool status, confirmation posture, queue status, and terminal state use safe protocol projections. Hidden reasoning, provider-native payloads, raw tool arguments/output, credentials, physical paths, and unbounded logs are omitted or represented by approved redaction metadata.

**Rationale:** Protocol streaming must preserve the same visibility and receipt policy as other frontends.

### 7. Fail closed on lifecycle and transport faults

**Choice:** Unknown required capability, unsupported message, malformed frame, duplicate correlation, out-of-order outcome, stale session/run ID, buffer exhaustion, client EOF during required confirmation, write failure, cancellation ambiguity, or facade/store/profile failure produces a typed terminal or blocked outcome. No reconnect, retry, new session, or unsandboxed fallback is inferred.

**Rationale:** Silent protocol recovery can duplicate prompts, approvals, tools, or transcript mutations.

### 8. Bind resume to durable profile admission

**Choice:** ACP session load/resume resolves an explicit safe session reference through the host, validates the durable store, repair state, active leaf, and runtime authority profile, and only then exposes continuation. Transport correlation IDs and external session labels are never durable authority and are hashed or redacted in receipts.

**Rationale:** An editor reconnect must not bypass the same recovery and authority checks as headless/TUI resume.

### 9. Use fixture-client conformance before live compatibility claims

**Choice:** A checked-in deterministic ACP fixture client drives initialize, capability negotiation, new/load session, prompt streaming, tool/permission flows, cancellation, resume, malformed input, and disconnect scenarios through the production codec/reducer/shell seam. Optional live editor smokes are separate evidence.

**Rationale:** Protocol behavior needs executable evidence without making a particular IDE or credential a test prerequisite.

## Risks / Trade-offs

- ACP protocol evolution can break compatibility; an exact cohort and explicit advertised feature set make upgrades reviewed.
- Stdio backpressure can stall the host; bounded queues and fail-closed termination favor honesty over unbounded buffering.
- Mapping rich Animus events into a smaller protocol can be lossy; projection diagnostics must not promote omitted state.
- Clients may interpret approval as authority; the bridge keeps authorization and enforcement outcomes distinct.
- Fixture-client conformance does not establish compatibility with every ACP client or IDE version.

## Non-Goals

- WebSocket, HTTP, TLS, remote listeners, remote trust, endpoint discovery, or network fallback.
- Implementing an MCP server, plugin host, terminal multiplexer, editor, or language server.
- Letting ACP own transcript, active-leaf, tool, queue, sandbox, authorization, or durable-session state.
- Ambient binary/client discovery, inherited credentials, provider login, model catalog UX, or global session discovery.
- Claiming arbitrary ACP-client parity, live-provider quality, production support, or whole-system correctness.
