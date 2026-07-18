# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Coding session host exposes a narrow lifecycle facade

r[agent_harness_runtime.coding_session_host.facade] Animus MUST expose a non-default coding-session host facade for prompt, continue, steer, follow-up, cancel, resume, compact, state and message snapshots, and event subscription over explicit injected services, without owning resource discovery, command parsing, credentials, model catalog UX, keybindings, themes, or terminal rendering.

#### Scenario: Embedder creates a bounded session host

- GIVEN an embedder supplies validated session config and explicit provider, tool, persistence, policy, cancellation, timer, and summarizer service postures
- WHEN the facade is constructed
- THEN it MUST expose the supported lifecycle operations and typed unavailable-service failures
- AND it MUST NOT discover or construct omitted services from ambient state.

#### Scenario: Caller supplies an unexpanded workflow command

- GIVEN input requires slash-command, prompt-template, skill, or project-resource interpretation
- WHEN it reaches the session facade without caller-owned expansion and admission
- THEN the facade MUST reject or treat it only as ordinary admitted prompt data according to its typed input
- AND it MUST NOT scan resources or execute command policy itself.

### Requirement: Session lifecycle decisions have a functional core

r[agent_harness_runtime.coding_session_host.functional_core] Run, queue, active-leaf, durable-cursor, pending-effect, cancellation, repair, recovery, and terminal-state transitions MUST be pure deterministic reductions over validated state, commands, and typed effect outcomes.

#### Scenario: Identical lifecycle input repeats

- GIVEN identical validated session state, command, policy, and effect outcome
- WHEN the reducer runs repeatedly
- THEN it MUST return identical next state, ordered events, and effect requests
- AND it MUST perform no filesystem, provider, tool, clock, timer, network, credential, terminal, or persistence effect.

### Requirement: One session has one active run

r[agent_harness_runtime.coding_session_host.single_run] A coding session MUST admit at most one active or cancelling run identity, reject overlapping ordinary prompts before transcript, queue, store, or provider mutation, and require correlated run identities for continuation and cancellation outcomes.

#### Scenario: Second ordinary prompt arrives while busy

- GIVEN one run is active
- WHEN another ordinary prompt is submitted without steering or follow-up behavior
- THEN the reducer MUST return an already-running error
- AND it MUST NOT append the prompt, enqueue it implicitly, contact a provider, or change the active leaf.

#### Scenario: Stale cancellation arrives

- GIVEN a cancellation outcome references an earlier run identity
- WHEN the current session has a different active run
- THEN the outcome MUST be rejected as stale
- AND current run state MUST remain unchanged.

### Requirement: Busy input uses explicit queue semantics

r[agent_harness_runtime.coding_session_host.queues] Prompt-like input during an active run MUST explicitly select steering or follow-up behavior, pending queue entries MUST remain non-transcript state, and a queued entry MUST become model-visible only after selected injection and durable user-message commit.

#### Scenario: Steering message is injected

- GIVEN a steering item is queued while a run is active
- WHEN the current assistant turn and admitted tool batch reach the steering injection boundary
- THEN the reducer MUST select the item according to queue policy and request a user-message commit
- AND no next provider dispatch may include it until that commit succeeds.

#### Scenario: Pending follow-up is edited or removed

- GIVEN a follow-up remains pending and has not been injected
- WHEN the caller edits or removes it through admitted queue operations
- THEN transcript history and the active leaf MUST remain unchanged
- AND queue status events MUST reflect only bounded safe state.

### Requirement: Dependent effects follow durable message boundaries

r[agent_harness_runtime.coding_session_host.durable_boundaries] The session host MUST durably commit admitted user messages before provider dispatch, assistant tool-call messages before tool dispatch, each tool result before provider continuation, and final assistant messages before terminal completion.

#### Scenario: New prompt starts

- GIVEN a valid idle session and admitted user prompt
- WHEN prompt reduction runs
- THEN the first effect request MUST be a user-message and active-leaf commit
- AND provider dispatch MUST not be requested until the commit outcome is successful and current.

#### Scenario: Assistant requests a tool

- GIVEN a provider returns an admitted assistant message containing tool calls
- WHEN the session host handles the response
- THEN it MUST commit the assistant message and leaf before tool execution
- AND each admitted tool result MUST be committed before another provider request.

#### Scenario: Persistence fails before tool execution

- GIVEN an assistant tool-call message cannot be durably committed
- WHEN the store returns failure
- THEN tool dispatch MUST not start
- AND the session MUST enter a typed blocked or recovery-required state.

### Requirement: Frontends consume one ordered runtime event stream

r[agent_harness_runtime.coding_session_host.events] Headless, SDK, and TUI consumers MUST observe the same ordered provider/UI-neutral runtime lifecycle events and MUST derive visible status and transcript boundaries without owning authoritative session mutation.

#### Scenario: Headless and TUI consume one run

- GIVEN one facade run produces prompt, model, tool, commit, queue, and terminal events
- WHEN headless and TUI reducers consume identical events
- THEN they MUST agree on lifecycle status, transcript boundaries, queue counts, tool correlation, and terminal state
- AND neither consumer may append session history independently.

#### Scenario: Retry or cancellation progress is transient

- GIVEN the host emits a progress event that is not a durable transcript message
- WHEN a frontend renders it
- THEN the event MAY affect visible progress state
- AND it MUST NOT be replayed as user, assistant, or tool transcript content.

### Requirement: Resume validates store, replay, and repair before continuation

r[agent_harness_runtime.coding_session_host.recovery] Resume MUST validate the committed store head, active-leaf replay, message/tool bindings, and required interrupted-tool repair before admitting prompt or continuation, and stale, malformed, partial, or unavailable recovery state MUST fail closed.

#### Scenario: Interrupted tool call is recovered

- GIVEN the committed active branch ends with or contains an unmatched assistant tool call and the store is writable
- WHEN resume runs
- THEN the host MUST plan, durably apply, and revalidate an admitted synthetic terminal repair before continuation
- AND it MUST not re-execute the tool.

#### Scenario: Store has ambiguous head

- GIVEN recovery reports committed corruption, conflicting head, partial commit, stale leaf, or unresolved malformed binding
- WHEN resume runs
- THEN the session MUST remain blocked with a typed recovery diagnostic
- AND no provider, tool, compaction, or branch mutation may start.

### Requirement: Session-host behavior has shared conformance evidence

r[agent_harness_runtime.coding_session_host.conformance] The coding-session facade MUST include shared black-box positive and negative scenarios over fake services and the explicit durable store, including event/store ordering, run exclusivity, queues, tools, cancellation, repair, restart, compaction, branches, service failures, and frontend reducer agreement.

#### Scenario: Durable fake-backed product rung is claimed

- GIVEN the packaged non-default session smoke passes with fake provider and tool services plus the explicit durable store
- WHEN readiness evidence is assembled
- THEN it MAY claim that bounded durable fake-backed session path
- AND it MUST list live providers, concrete product tools, TUI dogfood, daemon attach, plugin runtime, OAuth, and production readiness as non-claims.

#### Scenario: Change is ready for archive

- GIVEN maintainers intend to sync and archive the facade change
- WHEN closeout validation runs
- THEN focused reducer, host, store, headless, TUI, composition, receipt, packaged-smoke, dogfood-hygiene, Cairn, and relevant Nix checks MUST pass
- AND unavailable optional checks MUST retain blockers and next-best evidence.
