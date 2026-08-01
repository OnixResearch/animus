# Host control protocol boundary

Animus models host control as UI-neutral DTOs and pure validation, not as a terminal multiplexer or socket protocol. Semantic ownership lives in the standalone [`OnixResearch/host-control`](https://github.com/OnixResearch/host-control) repository, pinned at `e359ccb80a52efb860307278510fa5297ac970ed`. `animus-adapters` retains a compatibility re-export only; hosts own local sockets, named pipes, CLI wrappers, PTYs, terminal rendering, process input, policy, and durable workspace storage.

## Contracts

- `HostSelector` identifies a workspace, tab, pane, or agent for one host.
- `SelectorLifetime::Live` binds compact selectors to a topology revision; callers must re-resolve after topology changes.
- `SelectorLifetime::Durable` is available only when a host explicitly declares a selector durable.
- `HostTopologySnapshot`, `HostTopologyResource`, and `TopologyCommand` validate focus, rename, close, read, send, wait, subscribe, layout export, and layout apply commands over in-memory topology data with labels, cwd/provenance summaries, focus state, revision metadata, and semantic status.
- `HostOutputView` bounds visible, recent, recent-unwrapped, and detector-safe output reads with truncation, redaction, counts, optional safe references, and BLAKE3 hashes when truncation needs identity evidence.
- `LayoutNode`, `LayoutPaneDraft`, `LayoutCommandDescriptor`, `LayoutApplyRequest`, and `LayoutApplyResponse` describe declarative pane/split trees, labels, cwd summaries, optional command descriptors, focus, zoom, created topology, and focus state without claiming live PTY, scrollback, or process preservation.
- `HostEvent`, `HostEventSubscription`, `HostWaitRequest`, `filter_host_events`, and `resolve_host_wait` share semantic status matching for subscriptions and one-shot waits across agent status, output match, pane lifecycle, cancellation, timeout, and host-error events.
- `HostControlEnvelope`, `HostTypedError`, and `HostUnknownFields` carry correlation IDs, protocol version, typed errors, accepted features, unknown feature labels, and unknown fields without requiring any IPC server.

## Source identity and consumer mapping

`config/host-control-source.ncl` is the typed source of truth. Its checked JSON projection, `Cargo.lock`, and `flake.lock` must agree on the exact standalone revision. Workspace-relative, release-local, branch-only, tag-only, vendored, and direct transitive-fork sources fail the focused rail.

Animus maps its existing opaque host/resource IDs, live or durable selector lifetimes, revision-scoped topology operations, status vocabulary, bounded observations, layout DTOs, waits, and envelopes directly to the standalone core. Unsupported or lossy consumer states fail closed rather than being coerced. Tile adoption is an independent consumer lifecycle and does not make either product the semantic owner.

## Agent-control relationship

The separate [agent-control feature](agent-control-pubsub.md) owns pause, resume, cancellation, safe-checkpoint waiting, and bounded run-control events. Host control can resolve a UI selector and publish to that explicit run handle.

Host control does not own the agent-control state machine. Agent control does not own topology, selectors, sockets, layouts, PTYs, or transport authorization.

## ACP relationship

The optional [ACP stdio host](acp-stdio-host.md) is a separate bounded local protocol adapter, not a replacement host-control owner. It delegates lifecycle actions to the coding-session facade and does not gain process, daemon, remote-control, endpoint-discovery, credential, or host-shutdown authority.

## Non-claims

This contract does not copy Herdr's JSON protocol, start a host daemon, manage PTYs, preserve live processes during layout apply, read unbounded transcripts, or make terminal rendering available to reusable runtime code.

## Focused checks

```sh
cargo test -p animus-adapters --test host_control_conformance --locked
scripts/cairn/check-host-control-extraction --root .
nix build .#checks.x86_64-linux.host-control-extraction --no-link -L
```
