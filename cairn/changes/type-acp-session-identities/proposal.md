# Proposal: Type ACP session identities

## Why

The ACP stdio host distinguishes ACP session IDs, Animus session IDs, session identities, authority-profile identities, run IDs, entry IDs, connection identities, workspace identities, resource identities, and correlation IDs by field name.

Most of these values remain `String` in transport-neutral core models. A session, run, correlation, or authority identity can therefore enter the wrong internal API without a compiler error.

## What Changes

- Inventory identity-bearing strings in the ACP model, parser, facade commands, state transitions, events, and receipts.
- Add private newtypes for external ACP IDs and Animus-owned identities.
- Separate raw ACP frame DTOs from admitted typed requests.
- Migrate connection state, session binding, correlation tracking, facade commands, observations, updates, and safe receipts.
- Preserve the reviewed ACP protocol cohort, JSON-RPC frame shape, receipt schema, redaction rules, and BLAKE3 identity material.
- Add compile-fail, parser, state-machine, permission, cancellation, receipt, and fixture-client tests.
- Add Animus domain declarations for future Octet nominal enforcement.

## Impact

- **Crate**: `animus-acp-stdio-host` transport-neutral model and shell adapters.
- **Facade**: Typed conversion at the existing one-facade ACP ownership boundary.
- **Compatibility**: ACP protocol strings and receipt projections remain unchanged.
- **Tests**: Positive and negative fixture-client coverage remains the compatibility authority.

## Non-goals

- Do not claim arbitrary ACP client or editor compatibility.
- Do not change ACP protocol methods, frame limits, or permission semantics.
- Do not treat a typed session or authority identity as authorization.
- Do not add provider, filesystem, process, network, credential, or persistence authority to reusable cores.
- Do not wrap protocol versions, display labels, methods, or diagnostic text.
