# Canonical conversation view projection

Animus derives each visible conversation from one admitted ledger branch. The ledger remains the only model-context and tool-correlation owner.

`animus_runtime::conversation_projection` validates safe source facts and creates snapshot-first projections. Exact BLAKE3 identities bind the policy, source lineage, stream, cursor, items, actions, snapshots, plans, and receipts.

Assistant entries with one exact turn identity fold into one stable item. Steering entries remain ordered markers. Equal text and timestamps never define identity or order.

A cursor binds the session, branch, leaf, generation, anchor, sequence, policy, and active turn. Drift causes a bounded rebuild. A matching cursor produces an exact top-up or no-op.

Caches are disposable. `ConversationProjectionCache` is an injected port. The non-default std feature supplies policy loading and an in-memory cache fixture. Hosts own durable caches, transport, clocks, UI rendering, and ledger reads.

Legacy healing requires one exact source identity. It can add metadata only. Ambiguous joins, content changes, and active-turn races fail closed.

Redaction occurs before projection input. Views and receipts contain safe identities and counts. They reject transcript, reasoning, tool, provider, credential, environment, path, UI-state, truth, and delivery data.

The interaction reference is camelAI commit `70dd57a6219d9a19b25840aebbf1105c63c3a5f2` under MIT. Animus does not add a runtime dependency or adopt its storage model.

This contract does not prove transcript truth, UI delivery, cache durability, renderer correctness, accessibility, task completion, production readiness, or release eligibility.

```sh
scripts/cairn/check-conversation-projection --root .
nix build .#checks.x86_64-linux.conversation-projection --no-link -L
```
