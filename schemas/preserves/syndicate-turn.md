# Capability-free Syndicate turn profile

Profile label: `syndicate.protocol.v1.animus-safe`

The profile uses the Syndicate Protocol v1 first-order turn shapes while restricting assertion and message bodies to validated `preserves.dataspace.v1` records:

```preserves-schema
version 1 .

Turn      = [TurnEvent ...].
TurnEvent = [@oid Oid @event Event].
Event     = Assert / Retract / Message.

Assert  = <A @assertion AnimusDataspaceRecord @handle Handle>.
Retract = <R @handle Handle>.
Message = <M @body AnimusDataspaceRecord>.

Handle = int.
Oid = int.
```

`Oid` and `Handle` values accepted by this profile are non-negative integers representable as Rust `u64`. Turns contain at most `MAX_SYNDICATE_TURN_EVENTS` ordered entries. Text and binary fixtures are generated and decoded by `animus_preserves::{to,from}_syndicate_turn_{text,bytes}`.

The profile does not include `Sync = <S @peer #:#t>`, WireRefs, caveats, extensions, protocol error packets, or arbitrary Preserves assertion bodies. It is therefore a bounded profile, not a full Syndicate relay conformance claim.
