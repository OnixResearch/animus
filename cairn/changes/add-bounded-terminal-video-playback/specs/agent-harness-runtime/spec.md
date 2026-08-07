# Agent Harness Runtime

## ADDED Requirements

### Requirement: Terminal video planning is deterministic

r[agent_harness_runtime.terminal_video_playback.planning] Animus MUST model terminal video playback as deterministic state reduction over an admitted descriptor, bounded frame metadata, explicit monotonic ticks, terminal capabilities, policy, lifecycle commands, and prior state.

#### Scenario: Bounded playback advances

r[agent_harness_runtime.terminal_video_playback.planning.advance]
- GIVEN a valid playback descriptor, admitted terminal mode, bounded ordered frames, current tick, and running state
- WHEN pure playback reduction runs
- THEN it MUST return ordered fetch, display, wait, drop, finish, or cleanup actions according to policy
- AND it MUST read no clock, terminal, environment, file, URL, process, network, credential, or persistence state.

#### Scenario: Playback input is stale or invalid

r[agent_harness_runtime.terminal_video_playback.planning.invalid]
- GIVEN a stale tick, duplicate or out-of-order frame, invalid rate, exceeded bound, unknown state transition, or mismatched playback identity
- WHEN pure playback reduction runs
- THEN it MUST return a typed rejection or failure
- AND it MUST NOT fabricate time, silently reorder frames, or produce an unbounded action list.

### Requirement: Decoded frame delivery is injected

r[agent_harness_runtime.terminal_video_playback.frame_source] Animus MUST obtain decoded frames through an injected host frame-source service over approved opaque handles, bounded requests, bounded responses, and typed failures.

#### Scenario: Host supplies a decoded frame

r[agent_harness_runtime.terminal_video_playback.frame_source.frame]
- GIVEN the pure plan requests an admitted frame and the host frame source has that frame
- WHEN the service responds
- THEN it MUST return bounded image bytes or an approved frame handle plus safe metadata to the terminal shell
- AND reusable runtime contracts and durable TUI state MUST retain no frame body.

#### Scenario: Caller expects ambient decoding

r[agent_harness_runtime.terminal_video_playback.frame_source.ambient]
- GIVEN a caller supplies a path, URL, codec request, decoder command, credential, or provider payload without an injected authorized frame source
- WHEN playback preparation runs
- THEN Animus MUST return a typed missing-service or unsupported-authority outcome
- AND it MUST NOT open, fetch, decode, spawn, or resolve the media.

### Requirement: Playback backends are capability-specific

r[agent_harness_runtime.terminal_video_playback.backends] The optional TUI playback shell MUST use only admitted terminal-image conformance rows for Kitty animation, bounded image replacement, thumbnail fallback, or text fallback and MUST record the exact selected mode.

#### Scenario: Kitty animation is admitted

r[agent_harness_runtime.terminal_video_playback.backends.kitty]
- GIVEN the selected Rats cohort and terminal capability evidence admit the required Kitty animation operations
- WHEN an admitted playback plan executes
- THEN the host shell MUST use bounded frame transfer, animation control, placement, and deletion operations in planned order
- AND unsupported Kitty extensions MUST remain disabled and explicit.

#### Scenario: Animation is unsupported

r[agent_harness_runtime.terminal_video_playback.backends.fallback]
- GIVEN the terminal lacks an admitted animation row
- WHEN playback mode selection runs
- THEN it MUST select policy-bounded image replacement, thumbnail, text, or unsupported outcome
- AND it MUST NOT infer animation support from static image support.

### Requirement: Playback lifecycle is bounded and cancellable

r[agent_harness_runtime.terminal_video_playback.lifecycle] Playback MUST enforce named frame, queue, byte, geometry, rate, duration, drop, operation, deadline, and cleanup bounds before retaining or displaying frames, and every active playback MUST support cancellation.

#### Scenario: Producer exceeds consumer capacity

r[agent_harness_runtime.terminal_video_playback.lifecycle.backpressure]
- GIVEN the frame source produces more admitted frames than the selected terminal mode can consume
- WHEN pure lifecycle reduction runs
- THEN it MUST pause requests, drop frames, or fail according to explicit policy before queue bounds are exceeded
- AND it MUST record bounded dropped-frame and backpressure metadata.

#### Scenario: Cancellation occurs

r[agent_harness_runtime.terminal_video_playback.lifecycle.cancel]
- GIVEN playback owns protocol image or animation identities
- WHEN a matching cancellation, terminal loss, layout eviction, source failure, finish, or shutdown event occurs
- THEN reduction MUST stop new frame requests and create exact cleanup obligations
- AND completion MUST remain cleanup-incomplete until the host reports each cleanup outcome.

### Requirement: Playback receipts are metadata-only

r[agent_harness_runtime.terminal_video_playback.evidence] Playback receipts MUST bind policy, safe source class, selected backend mode, terminal-image cohort, bounded frame and drop counts, status, cleanup result, and BLAKE3 identities without retaining media content or authority-bearing values.

#### Scenario: Playback receipt is safe

r[agent_harness_runtime.terminal_video_playback.evidence.safe]
- GIVEN a playback reaches finish, cancel, failure, unsupported, or cleanup-incomplete state
- WHEN receipt projection runs
- THEN it MUST emit only admitted metadata, bounded counts, safe labels, status, non-claims, and BLAKE3 identities
- AND receipt identity MUST be deterministic over those typed fields.

#### Scenario: Playback receipt leaks content

r[agent_harness_runtime.terminal_video_playback.evidence.leak]
- GIVEN a receipt candidate contains media or frame bytes, base64, escape sequences, decoder output, paths, URLs, credentials, environment values, terminal responses, raw timing traces, or unbounded text
- WHEN receipt validation runs
- THEN it MUST reject the candidate with a typed leak diagnostic
- AND the candidate MUST NOT become durable evidence.

### Requirement: Playback verification includes adverse lifecycle paths

r[agent_harness_runtime.terminal_video_playback.verification] Terminal video playback MUST include positive and negative deterministic scheduler, frame-source, backend, bounds, backpressure, cancellation, terminal-loss, cleanup, fallback, receipt, dependency-isolation, and lifecycle checks before archive.

#### Scenario: Change reaches closeout

r[agent_harness_runtime.terminal_video_playback.verification.closeout]
- GIVEN implementation and fake-backed fixtures are complete
- WHEN closeout runs
- THEN focused tests, dependency guards, Nickel checks, Cairn validation, and all Cairn gates MUST pass
- AND skipped decoder, live-terminal, smoothness, color, audio, synchronization, production, and release checks MUST remain explicit non-claims.
