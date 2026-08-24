# Persist-before-process gateway

`animus_session_core::gateway` defines pure gateway admission, ordering, recovery, and authority contracts. It does not implement a remote transport.

## Durable admission

The shell must commit each inbound update before agent processing starts. It must commit each reply intent before transport delivery starts. Records bind stable work, conversation, content, and sequence identities.

An initial admission plan never authorizes an effect. Exact commit evidence changes the record to `Committed`. Processing and delivery reject every uncommitted record.

## Ordering and capacity

One conversation processes updates in sequence order. Earlier work must complete, park, or cancel before later work starts. Separate conversations can progress under an explicit host capacity.

The core emits plans only. The host owns workers, executors, clocks, storage, and concurrency enforcement.

## Recovery and authority

Recovery reconstructs durable pending updates and reply intents. A processing update returns to pending admission under its stable identity. An in-flight reply remains uncertain until exact transport evidence exists.

Remote gateway sessions admit read-only tools by default. Mutation requires exact current session, remote-user, tool, and request authority. Transport authentication does not grant mutation authority.

## Non-claims

This contract does not prove remote identity, transport authentication, message freshness, exactly-once delivery, external acceptance, tool safety, sandboxing, or service correctness.

Run the focused rail:

```sh
nix develop -c scripts/cairn/check-persist-before-process-gateway --root .
```
