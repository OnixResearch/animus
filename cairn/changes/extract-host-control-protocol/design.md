## Context

Animus already expresses the desired dependency direction: reusable host-control logic is UI-neutral and transport-neutral, while hosts own sockets, named pipes, CLI wrappers, PTYs, terminal rendering, process input, and durable storage. Tile independently demonstrates that this contract is useful outside Animus, but its current types are coupled to Tile workspace state and serde wire behavior.

The extraction boundary is therefore the semantic protocol and validation core. It is not a shared daemon, terminal multiplexer, workspace implementation, or transport stack.

## Decisions

### 1. A standalone repository owns the protocol vocabulary

**Choice:** Create `OnixResearch/host-control` with a canonical `host-control-core` package, optional serialization packages or features, a mapping-profile catalog, and consumer conformance fixtures.

**Rationale:** Animus and Tile both need stable review of selectors, topology, bounded observation, semantic status, layout, streams, waits, errors, and feature negotiation without introducing a dependency on the other product.

### 2. Core logic remains `no_std`, pure, and transport-neutral

**Choice:** `host-control-core` uses `#![no_std]` plus `alloc` and accepts explicit values only. It owns DTO validation, selector freshness decisions, topology command admission, bounded-output shaping, layout validation, event filtering, wait resolution over supplied events, stream-order validation, feature negotiation, and deterministic error classification.

It does not read files or environment state, open sockets, inspect processes, access clocks, render terminals, mutate daemon state, persist snapshots, print, or spawn work. Timeout and TTL values are validated as data; consumer shells measure time.

**Rationale:** The pure core can be tested without standing up Animus, Tile, a daemon, a terminal, or a transport.

### 3. The canonical vocabulary is a superset with explicit profiles

**Choice:** The base resource vocabulary covers host, workspace, tab, pane, and agent selectors; live revision-bound and host-declared durable identity; topology snapshots; bounded visible/recent/detector-safe output; truncation, redaction, and BLAKE3 identity metadata; declarative layouts; semantic events and waits; typed errors; protocol versions; and feature labels.

Optional profiles cover layout application, snapshot-first streams, and consumer-specific wire projections. A consumer must negotiate a feature before using it.

**Rationale:** Forcing every host to implement every operation would turn neutral DTOs into an inaccurate lowest-common-denominator server contract.

### 4. Status mapping is explicit and loss-aware

**Choice:** The canonical semantic status set includes unknown, idle, running, waiting, blocked, done, and failed. Animus maps its idle/running/waiting/done/failed states directly and treats unknown/blocked according to a declared profile. Tile maps unknown/idle/working/blocked/done and declares how failed or waiting are represented. A lossy mapping must carry a diagnostic or be rejected; it cannot silently promote one state to another.

**Rationale:** Animus and Tile currently use overlapping but non-identical vocabularies. Hidden coercion would make waits and attention state unsound.

### 5. Native identifiers stay consumer-owned

**Choice:** The canonical core uses opaque validated host/resource identifiers and explicit revision/lifetime metadata. Animus and Tile adapters preserve their native IDs and record deterministic mapping. Tile's numeric `SessionId`, `TabId`, and `PaneId` remain Tile-owned; Animus's string selectors remain Animus-owned. Positional indices require the scope and revision that produced them and are never accepted as durable identity.

**Rationale:** Shared semantic identity does not require one product to adopt another product's storage or wire representation.

### 6. Streams are snapshot-first when that feature is negotiated

**Choice:** A consumer advertising the snapshot-first stream feature must emit one accepted initial snapshot before incremental updates for that stream identity. Incremental-before-snapshot, cross-stream revision, unsupported version, and unbounded/raw stream fixtures fail closed. Hosts unable to provide a redaction-safe snapshot do not advertise the feature.

**Rationale:** Tile already relies on this reconnect contract, and it is a useful neutral rule only when stated as an explicit capability.

### 7. Serialization is an adapter, not core authority

**Choice:** serde and Preserves support are optional adapters over validated core DTOs. Decoding must re-run core validation. The standalone repository does not require JSON, Preserves, a socket framing protocol, or a daemon. Animus's active Preserves bridge split remains responsible for payload-family serialization, not semantic ownership.

**Rationale:** A protocol core should not confuse one encoding with its semantic contract.

### 8. Cutover uses exact pins and compatibility facades

**Choice:** Animus first pins an immutable standalone revision, delegates semantic operations, and keeps its public host-control exports as a facade. The facade contains type aliases, conversions, and re-exports only. Tile adopts through a separate Tile-owned Cairn package and adapter after the standalone conformance profile is released.

No workspace-relative sibling path is accepted in release evidence, and materially different implementations may not share the canonical package name/version identity.

**Rationale:** Incremental cutover preserves product ownership and lets each consumer validate rollback independently.

## Rejected Alternatives

- **Keep the protocol in Animus:** Tile and future hosts would inherit an agent-harness dependency or duplicate semantics.
- **Use `tile-protocol` as the common crate:** it includes Tile-specific commands, snapshots, async traits, and daemon/frontend wire behavior.
- **Extract a shared daemon:** transport and process authority are intentionally outside the common boundary.
- **Merge active Animus crate splits into this package:** authority-scoped std adapters and Preserves payload bridges solve different ownership problems.

## Risks / Trade-offs

- A canonical superset can become too broad; optional feature profiles and a small mandatory base limit that risk.
- Compatibility adapters temporarily increase code before product-local semantic copies can be removed.
- Status mappings need careful negative tests because equal labels do not always imply equal authority or lifecycle meaning.
- Selected conformance fixtures establish bounded compatibility, not arbitrary behavioral equivalence between Animus and Tile.
