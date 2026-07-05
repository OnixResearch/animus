# agent-harness-runtime Change Spec: TUI conversation render model

## Requirements

### Requirement: Conversation render model is pure and downstream-only

r[agent_harness_runtime.tui_conversation.boundary] Animus MUST keep the conversation render reducer in `animus-tui` as a pure function over `RuntimeEvent` and adapter DTO snapshots, with no terminal I/O, polling, redraw, provider calls, or tool execution, and MUST NOT add new types or fields to `animus-core`, `animus-adapters`, `animus-steel`, `animus-runtime`, or `animus-std` to support it.

#### Scenario: Reducer is testable without a terminal

r[agent_harness_runtime.tui_conversation.boundary.pure]
- GIVEN a recorded sequence of `RuntimeEvent` and adapter DTO values
- WHEN the conversation render reducer is advanced with them
- THEN it produces a visible transcript model deterministically
- AND no terminal handle, raw-mode call, provider request, or tool dispatch occurs.

#### Scenario: Reusable crates gain no TUI contract

r[agent_harness_runtime.tui_conversation.boundary.no_leak]
- GIVEN maintainers inspect reusable crate manifests and public exports
- WHEN the conversation render model is added
- THEN no new transcript or conversation types appear in `animus-core`, `animus-adapters`, `animus-steel`, `animus-runtime`, or `animus-std`
- AND the TUI consumes only already-accepted downstream surfaces.

### Requirement: Typed bounded transcript entries

r[agent_harness_runtime.tui_conversation.transcript_reducer] The conversation render model MUST produce typed transcript entries for user messages, assistant text, thinking, tool-use, tool-result, image, and error, each carrying bounded model-visible content and safe metadata, and MUST NOT store raw provider bodies, credentials, full tool payloads, or unbounded output verbatim.

#### Scenario: Assistant content blocks assemble in order

r[agent_harness_runtime.tui_conversation.transcript_reducer.ordering]
- GIVEN a model response contains text, thinking, and tool-use blocks followed by a tool result
- WHEN the reducer applies the corresponding events
- THEN the transcript entries appear in event order with their typed kinds
- AND each entry exposes bounded content and safe size/truncation metadata.

#### Scenario: Raw provider body is not stored verbatim

r[agent_harness_runtime.tui_conversation.transcript_reducer.no_raw_body]
- GIVEN an adapter DTO carries a large or sensitive provider body
- WHEN the reducer records the corresponding transcript entry
- THEN it stores only bounded model-visible content and safe metadata
- AND the raw provider body is not embedded in the render model.

### Requirement: Deterministic text snapshot for tests

r[agent_harness_runtime.tui_conversation.test_backend] The conversation render model MUST expose a deterministic text-snapshot projection of the visible transcript that focused tests can assert against without a real terminal.

#### Scenario: Snapshot reproduces visible transcript

r[agent_harness_runtime.tui_conversation.test_backend.snapshot]
- GIVEN a conversation render model advanced with a recorded event sequence
- WHEN `render_text_snapshot` is invoked
- THEN it returns deterministic text representing the ordered transcript entries, truncation markers, and failure entries
- AND the test requires no terminal backend.

### Requirement: Bounded output and leak rejection

r[agent_harness_runtime.tui_conversation.bounded_output] The conversation render model MUST bound visible transcript growth, record truncation state and optional BLAKE3 content hashes for identity, and reject fixtures that embed raw credentials, provider payloads, or unbounded output.

#### Scenario: Oversized entry is truncated with metadata

r[agent_harness_runtime.tui_conversation.bounded_output.truncated]
- GIVEN a model or tool result exceeds the configured visible budget
- WHEN the reducer records the entry
- THEN model-visible content is truncated according to policy
- AND the entry metadata records bounded sizes, truncation status, and an optional BLAKE3 hash without the full raw body.

#### Scenario: Unknown block kind degrades safely

r[agent_harness_runtime.tui_conversation.bounded_output.unknown_kind]
- GIVEN the reducer receives an event variant it does not model
- WHEN it applies the event
- THEN it records a typed unknown-entry marker without panicking
- AND no unbounded or unsafe content is introduced into the transcript.

### Requirement: Conversation render model verification

r[agent_harness_runtime.tui_conversation.verification] The change MUST include focused positive and negative tests for transcript assembly, ordering, bounded output, truncation metadata, unknown kinds, leak rejection, dependency isolation, and text snapshots before closeout.

#### Scenario: Closeout evidence covers conversation fixtures

r[agent_harness_runtime.tui_conversation.verification.closeout]
- GIVEN maintainers intend to archive the conversation render model change
- WHEN closeout validation runs
- THEN evidence names focused reducer tests, snapshot tests, dependency-isolation checks, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
