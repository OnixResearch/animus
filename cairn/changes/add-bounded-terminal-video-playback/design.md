## Context

Terminal video is a sequence of image operations under timing and resource policy. A decoder owns untrusted container and codec processing. A frame source owns decoded frame delivery. The TUI owns display state and terminal cleanup. Combining these roles creates broad process, filesystem, codec, clock, and terminal authority. The existing Animus architecture instead favors pure state reduction and injected host services.

## Decisions

### Decision: model playback as a pure state machine

**Choice:** A deterministic playback core receives an admitted playback descriptor, bounded frame metadata, explicit monotonic ticks, terminal capability facts, policy, and lifecycle commands. It returns fetch, display, drop, wait, pause, finish, fail, cancel, and cleanup actions without reading a clock or terminal.

**Rationale:** Tests can reproduce scheduling and failure paths without real media, time, or terminal state.

### Decision: inject decoded frames through an opaque source

**Choice:** A host-owned frame-source service resolves an approved source handle and returns bounded decoded image frames plus safe metadata. Reusable code and the TUI planner do not open files, fetch URLs, spawn decoders, inspect codecs, or retain an unbounded queue.

**Rationale:** Codec and transport authority is larger than display authority and needs a separate security boundary.

### Decision: use protocol capabilities instead of a universal playback claim

**Choice:** Kitty animation operations are used only when their conformance rows pass. Other terminals use policy-bounded image replacement or a text or thumbnail fallback. The result records the exact backend mode and unsupported operations.

**Rationale:** iTerm2, Sixel, multiplexers, and Kitty implementations have different update and cleanup behavior.

### Decision: apply backpressure before frame retention

**Choice:** Typed Nickel policy owns named queue, frame-byte, aggregate-byte, geometry, rate, duration, skipped-frame, operation, and cleanup bounds. The pure core requests at most the admitted window, drops or pauses according to policy, and never accumulates an unbounded frame list.

**Rationale:** Video can exhaust memory and terminal bandwidth even when each frame is individually valid.

### Decision: cancellation always creates cleanup work

**Choice:** Cancellation, source failure, terminal loss, layout eviction, replacement, finish, and shutdown create exact cleanup obligations for allocated protocol identities. Completion remains cleanup-incomplete until the host reports the cleanup outcome.

**Rationale:** Terminal image state can outlive the visible TUI unless delete operations are explicit.

### Decision: keep evidence metadata-only

**Choice:** Receipts bind policy, source-handle class, frame counts, dropped counts, backend mode, terminal-image cohort, BLAKE3 identities, terminal outcome, and cleanup status. They exclude frames, media bytes, decoder output, escape sequences, terminal responses, paths, URLs, and timing traces.

**Rationale:** Playback evidence needs outcome and bound facts, not content retention.

## Risks / Trade-offs

- Host frame sources can lie about decoded metadata; Animus receipts remain scoped to supplied observations.
- Terminal throughput varies; deterministic policy cannot guarantee smooth playback.
- Kitty animation support can require more upstream Rats operations than static images.
- Frame dropping improves boundedness but changes visible playback.

## Non-Claims

This design does not establish decoder safety, media authenticity, exact timing, smooth playback, frame completeness, color fidelity, audio support, audio-video synchronization, terminal correctness, sandboxing, production readiness, or release eligibility.
