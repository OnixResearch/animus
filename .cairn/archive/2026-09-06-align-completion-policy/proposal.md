# Align completion policy with the current lifecycle input

## Why

The current Cairn CLI rejects the repository's positive completion fixture.
The lifecycle upgrade retained old policy identities in the runtime and registry.
Process exit zero does not mean that the completion verdict passes.

## What Changes

Align the reviewed policy identity across the registry, runtime, and completion fixtures.
Use the owner's canonical hash operation and compare complete core and CLI reports.
Keep old-policy, stale-subject, and non-passing status rejection.
Preserve the independently pinned core and CLI where selected-contract parity holds.

## Impact

The change affects completion compatibility and its maintained verification rail.
Old-policy registries and evidence remain invalid after the explicit policy update.
No default feature, process capability, library algorithm, or dependency pin changes.

## Out of Scope

Native training, executable authenticity, production promotion, arbitrary caller authentication,
and policy changes to the already adopted outcome-machine lifecycle are excluded.
