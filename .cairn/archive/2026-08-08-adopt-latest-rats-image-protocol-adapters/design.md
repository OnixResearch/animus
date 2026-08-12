## Context

Animus keeps terminal effects inside `animus-tui`. Its current `ImageDisplay` retains only a bounded description and media type, while model image bytes remain host-owned. The pinned Rats revision does not include `rat-image`. The latest observed upstream default branch does include it, but the current API performs environment discovery and terminal opening inside the effectful call. Animus must use the upstream implementation without importing ambient terminal behavior into its pure planning boundary or overstating protocol coverage.

A bounded architecture review considered three mechanisms: copy protocol code into Animus, call `rat-image` directly from render models, or place `rat-image` behind an Animus host shell with a pure plan. Copying creates a fork. Direct calls mix terminal authority with deterministic state. The host-shell mechanism preserves the existing crate boundary and permits explicit evidence.

## Decisions

### Decision: resolve latest upstream, then pin an immutable cohort

**Choice:** At implementation baseline time, fetch the canonical Rats remote default branch, record the observed head, review changes from the current pin, and pin the selected commit in every Cargo and Nix reference. The implementation observation is `3b498c3d7c248c335d3e7faf12f133a18c87d632`; it replaced the earlier planning observation after review.

**Rationale:** “Latest” is a selection policy, not a reproducible dependency identity. The receipt needs the exact selected commit and matching generated lock state.

### Decision: keep protocol effects in a thin `animus-tui` shell

**Choice:** Pure code produces a `TerminalImagePlan` from explicit capability observations, layout, image metadata, policy, and lifecycle state. A shell resolves an approved handle to bounded bytes and invokes upstream `rat-image` through explicit protocol and terminal-writer inputs. If upstream lacks the required injection seam, the prerequisite change lands in Rats first. Animus then pins the resulting upstream revision instead of carrying a local patch.

**Rationale:** This split keeps protocol selection and validation testable without a terminal. It also keeps TTY handles, environment reads, writes, and cleanup outside reusable crates.

### Decision: define protocol coverage as a conformance matrix

**Choice:** The focused profile records separate capability rows for Kitty query, transmit, placement, replacement, deletion, and static cleanup; iTerm2 inline image transmission; Sixel rendering; and text fallback. Unsupported rows fail or fall back according to explicit policy. Animation rows remain owned by the later video-playback change.

**Rationale:** A protocol name alone does not establish complete support. A matrix prevents a basic transmit call from becoming a “full Kitty protocol” claim.

### Decision: use bounded handles and metadata-only evidence

**Choice:** Render models carry safe handles, media type, dimensions, byte count, BLAKE3 identity, and alt text. The host resolves bytes only for an admitted plan. Receipts record the selected cohort, protocol row, bounds, status, cleanup result, and safe identities. They never store image bytes, base64, escape payloads, private paths, environment values, or terminal responses.

**Rationale:** Terminal graphics require bytes at the shell, but durable state and evidence do not.

### Decision: keep fallback visible and fail closed on unsafe paths

**Choice:** A missing protocol produces bounded alt text. Malformed capabilities, oversized images, invalid geometry, unknown protocol rows, writer failures, partial transmissions, stale image IDs, and failed cleanup return typed outcomes. The adapter never silently writes escape sequences to stdout as a fallback.

**Rationale:** A visible text fallback is safer than hidden terminal output or a false successful-render claim.

## Risks / Trade-offs

- The latest Rats revision can add unrelated dependency changes; cohort review and exact pinning limit this risk.
- Upstream `rat-image` can require an injected-writer refactor before Animus can adopt it safely.
- Terminal multiplexers and remote sessions can report capabilities incorrectly; explicit observations and text fallback limit the result.
- Live terminal behavior varies by emulator and cannot be established by byte fixtures alone.

## Non-Claims

This change does not establish video playback, audio, provider image availability, terminal-emulator correctness, remote-session transparency, every Kitty extension, production TUI readiness, or release eligibility. Static protocol fixtures establish only the selected encoded operations and Animus boundary behavior.
