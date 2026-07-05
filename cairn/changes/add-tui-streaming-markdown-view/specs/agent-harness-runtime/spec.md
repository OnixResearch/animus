# agent-harness-runtime Change Spec: TUI streaming markdown view

## Requirements

### Requirement: Streaming deltas advance the conversation reducer

r[agent_harness_runtime.tui_streaming.delta_lifecycle] The streaming view MUST apply model-streaming text and thinking deltas and content-block start/stop lifecycle events to the conversation render model incrementally, and MUST tolerate orphaned or out-of-order deltas without panicking or corrupting transcript order.

#### Scenario: Text deltas accumulate into a content block

r[agent_harness_runtime.tui_streaming.delta_lifecycle.accumulate]
- GIVEN a content-block-start event followed by several text deltas and a content-block-stop event
- WHEN the streaming view applies them to the conversation model
- THEN the transcript assembles a single assistant text entry from the accumulated deltas
- AND the entry metadata records the block lifecycle boundaries.

#### Scenario: Orphaned delta degrades safely

r[agent_harness_runtime.tui_streaming.delta_lifecycle.orphan]
- GIVEN a text delta arrives with no preceding content-block-start event
- WHEN the streaming view applies it
- THEN the delta is recorded under a typed orphan marker without panicking
- AND no unbounded or unsafe content is introduced.

### Requirement: Markdown rendering is host-only

r[agent_harness_runtime.tui_streaming.markdown_render] Markdown rendering MUST use host-only `rat-markdown` widgets consuming bounded conversation-model content, and reusable crates MUST NOT import Rats, Ratatui, or markdown widget types.

#### Scenario: Markdown snapshot reproduces rendered blocks

r[agent_harness_runtime.tui_streaming.markdown_render.snapshot]
- GIVEN the conversation model holds accumulated markdown text
- WHEN the host view renders it through `rat-markdown`
- THEN the visible output reflects formatted markdown bounded by the visible window
- AND reusable crates import no markdown widget types.

### Requirement: Bounded multimodal image payloads

r[agent_harness_runtime.tui_streaming.image_payloads] Multimodal image payloads MUST render as bounded display images with safe metadata and MUST NOT retain raw provider bytes in render state beyond the display window.

#### Scenario: Oversized image payload is bounded

r[agent_harness_runtime.tui_streaming.image_payloads.bounded]
- GIVEN an adapter image payload exceeds the configured display budget
- WHEN the view records the image entry
- THEN it stores a bounded display descriptor and safe metadata
- AND the raw provider bytes are not retained in render state.

### Requirement: Bounded scrollback with eviction metadata

r[agent_harness_runtime.tui_streaming.scrollback] The streaming view MUST keep a bounded visible scrollback window and MUST record safe eviction metadata for evicted entries rather than retaining full content.

#### Scenario: Evicted entries leave metadata only

r[agent_harness_runtime.tui_streaming.scrollback.eviction]
- GIVEN the transcript exceeds the configured visible window
- WHEN the scrollback window evicts older entries
- THEN evicted entries leave counts and optional hashes as metadata
- AND full evicted content is not retained in render state.

### Requirement: Streaming view verification

r[agent_harness_runtime.tui_streaming.verification] The change MUST include focused positive and negative tests for delta accumulation, block lifecycle, markdown rendering, image bounding, scrollback eviction, orphaned deltas, and Rats-type isolation before closeout.

#### Scenario: Closeout evidence covers streaming fixtures

r[agent_harness_runtime.tui_streaming.verification.closeout]
- GIVEN maintainers intend to archive the streaming markdown view change
- WHEN closeout validation runs
- THEN evidence names focused streaming, markdown, image, scrollback, dependency-isolation, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
