## Phase 1: Prerequisites and playback inventory

- [ ] [serial] Complete or bind passing evidence from `adopt-latest-rats-image-protocol-adapters`, including exact Rats cohort and static lifecycle conformance. r[agent_harness_runtime.terminal_video_playback.backends]
- [ ] [serial] Run and record current positive and negative TUI render, image display, control, queue, cancellation, safe-receipt, dependency-isolation, Nickel, and Cairn baselines. r[agent_harness_runtime.terminal_video_playback.verification]
- [ ] [serial] Inventory selected Kitty animation operations, bounded replacement behavior for alternate protocols, terminal cleanup rules, and explicit unsupported rows. r[agent_harness_runtime.terminal_video_playback.backends]
- [ ] [serial] Define a typed Nickel playback profile with named frame, queue, byte, geometry, rate, duration, drop, operation, deadline, and cleanup bounds; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.terminal_video_playback.planning] r[agent_harness_runtime.terminal_video_playback.lifecycle]

## Phase 2: Pure playback core

- [ ] [serial] Add typed playback descriptor, capability, state, command, tick, frame metadata, action, outcome, cleanup, and error contracts under the TUI boundary. r[agent_harness_runtime.terminal_video_playback.planning]
- [ ] [serial] Implement deterministic reduction for start, fetch, display, wait, pause, resume, drop, cancel, source failure, terminal loss, finish, and cleanup over explicit inputs. r[agent_harness_runtime.terminal_video_playback.planning] r[agent_harness_runtime.terminal_video_playback.lifecycle]
- [ ] [parallel] Add positive scheduler tests for ordered frames, pause and resume, bounded buffering, admitted frame dropping, finish, and cleanup. r[agent_harness_runtime.terminal_video_playback.planning] r[agent_harness_runtime.terminal_video_playback.verification]
- [ ] [parallel] Add negative tests for stale ticks, duplicate or out-of-order frames, invalid rates, oversized frames, queue overflow, missing capabilities, source failure, cancellation races, terminal loss, and cleanup failure. r[agent_harness_runtime.terminal_video_playback.lifecycle] r[agent_harness_runtime.terminal_video_playback.verification]

## Phase 3: Injected frame source and terminal shell

- [ ] [serial] Add an injected frame-source service over approved opaque handles with bounded decoded-frame responses and typed source failures; keep file, URL, codec, process, network, credential, clock, and persistence effects outside Animus. r[agent_harness_runtime.terminal_video_playback.frame_source]
- [ ] [serial] Add a non-default host shell for admitted Kitty animation operations and policy-bounded static image replacement, plus text or thumbnail fallback. r[agent_harness_runtime.terminal_video_playback.backends]
- [ ] [serial] Wire cancellation, terminal loss, layout eviction, finish, and shutdown to exact image and animation cleanup obligations without direct writes from the pure core. r[agent_harness_runtime.terminal_video_playback.lifecycle]
- [ ] [parallel] Add fake frame-source and writer fixtures for success, backpressure, partial writes, source timeout, terminal disconnect, unsupported animation, fallback, cancellation, and cleanup failure. r[agent_harness_runtime.terminal_video_playback.frame_source] r[agent_harness_runtime.terminal_video_playback.backends] r[agent_harness_runtime.terminal_video_playback.verification]

## Phase 4: Evidence and closeout

- [ ] [parallel] Add deterministic metadata-only BLAKE3 receipts and leak negatives for media or frame bytes, base64, escape sequences, decoder output, paths, URLs, environment values, terminal responses, timing traces, and unsupported smooth-playback claims. r[agent_harness_runtime.terminal_video_playback.evidence] r[agent_harness_runtime.terminal_video_playback.verification]
- [ ] [serial] Update the component catalog, facade inventory, TUI docs, README, feature guidance, authority map, non-claims, and focused Cairn/Nix rail. r[agent_harness_runtime.terminal_video_playback.evidence]
- [ ] [serial] Run focused tests, dependency and source-shape guards, Nickel checks, relevant workspace checks, Cairn validation, and proposal/design/tasks gates; record skipped decoder, live-terminal, audio, production, and release checks. r[agent_harness_runtime.terminal_video_playback.verification]
