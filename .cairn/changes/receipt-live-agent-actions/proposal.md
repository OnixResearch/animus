# Proposal: Receipt every live agent action

## Why

Animus runs a live Steel loop where operators and agents act against live session state. Squeak workspaces shared this power and failed to version the actions. The stack rule is the opposite: every decision leaves evidence.

Animus already receipts selected steps: model-request snapshots, tool-execution stages, session events, and the durable journal. The remaining gap is universality. Not every admitted live action produces a published, externally verifiable receipt under the authority gates.

## What Changes

- Add a pure no-std live-action receipt family: action classification, canonical receipt projection, BLAKE3 receipt identity, and verification rules. r[animus.live_action_receipt.identity]
- Publish one receipt for every admitted live-loop action when the session records. r[animus.live_action_receipt.universality]
- Require each receipt to carry only metadata claims. r[animus.live_action_receipt.boundary]
- Make every receipt verifiable with no store access. r[animus.live_action_receipt.verifiable]
- Record the exact authority-gate identities for each action. r[animus.live_action_receipt.authority]

## Lifecycle Prerequisite

Animus runs the native `.cairn/` lifecycle. The checked-in generated policy was stale and is now synced to the cairn repo's current `cairn-default` policy.

## Impact

- **Core**: a new no-std `animus-adapter-action-receipt` family under `crates/` owns DTOs, classification, canonical projection, and verification.
- **Shell**: `animus-std-action-receipt` publishes through an injected store port when a session records.
- **Testing**: positive and negative no-std, codec, host, and Cairn verification.
- **Boundary**: no product effect moves into the family. The loop keeps existing semantics.

## Out of Scope

- Proving model honesty, tool success, or system correctness.
- Changing loop semantics or authority decisions.
- Owning receipt-store retention.
- Replacing model-request snapshots, tool-execution stages, or session-event replay.

## Affected Specs

- `agent-harness-runtime`: live-action receipt requirements.
