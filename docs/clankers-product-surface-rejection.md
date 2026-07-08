# Clankers product-surface rejection

Animus uses Clankers as a bounded architecture reference, not as a product blueprint. The reusable Animus crates may adopt Clankers-style boundary patterns such as explicit lifecycle ownership, injected effect services, focused verification rails, safe receipts, neutral ledgers, policy seams, and deterministic evidence. They must not make Clankers product surfaces default Animus behavior.

## Rejected default surfaces

Animus rejects default adoption of these Clankers product surfaces:

- Clankers TUI and terminal application workflows;
- daemon/attach protocol and long-lived product daemon ownership;
- Matrix/P2P coordination and peer messaging;
- global `.clankers` state directories or host-global session stores;
- provider router/OAuth ownership and bundled provider-account flows;
- plugin subprocess runtime and product plugin launch behavior;
- Wasm/Extism loading as a default reusable runtime surface;
- Automerge desktop storage and desktop-app persistence assumptions;
- self-evolution apply/rollback workflows;
- live dogfood harnesses and product dogfood automation;
- product tools shipped as Animus reusable defaults; and
- release tagging workflow ownership.

These surfaces may remain useful references for host applications. They are not reusable Animus defaults, and they must not appear as required dependencies, runtime assumptions, global paths, or implicit shell behavior in `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, or the default `animus-std` contracts. The [dogfoodable host readiness roadmap](dogfoodable-host-readiness-roadmap.md) defines how optional host-owned dogfoodability can advance without weakening this default rejection stance.

## Bounded pattern adoption

Accepted Clankers influence is limited to small, host-neutral contracts:

- lifecycle state is explicit and validated at the boundary;
- effects are represented by injected service traits or DTOs before dispatch;
- receipts carry safe summaries, hashes, classifications, and redaction markers;
- ledgers preserve neutral replay state without binding to one product store; and
- verification rails are focused, deterministic, and runnable without standing up a product daemon.

When a pattern needs an effect, Animus names the host-owned service boundary and keeps the reusable core deterministic. When a pattern needs a UI, provider, plugin, storage layer, daemon, or network transport, the host owns that implementation outside the reusable default contracts.

## Future exception rule

Any future change that adopts a rejected Clankers product surface must create a focused Cairn change before implementation. The proposal and design must name:

- host ownership for the product surface;
- injected service boundaries between reusable contracts and effectful shell code;
- safety policy and permission checks;
- redacted receipts and bounded evidence;
- non-default feature gating; and
- focused validation rails proving the surface cannot become ambient default behavior.

The default remains rejection unless that Cairn change explicitly justifies default behavior and updates this note, the accepted specs, and the marker rail together.
