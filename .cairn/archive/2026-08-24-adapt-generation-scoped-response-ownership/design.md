## Context

Animus owns provider-neutral streaming DTOs and optional provider parsers. Host adapters own sockets, HTTP clients, Iroh sessions, cancellation, and teardown. Reusable transports need an explicit owner boundary so an old stream cannot contaminate a new model response.

`digitallyinduced/haskell-agent` assigns each WebSocket request generation-scoped ownership. It poisons a connection after interruption, malformed input, or an exchange that returns before its terminal frame.

## Goals

- Bind one active exchange generation to each reusable response connection.
- Fail closed on concurrent ownership attempts.
- Poison uncertain connections before any later exchange can claim them.
- Release ownership only after a valid terminal frame.

## Non-goals

- Transport authentication, encryption, or provider correctness.
- Provider-independent equivalence of terminal frames.
- Automatic retry or replay after poisoning.
- A global connection manager in reusable core crates.

## Decisions

### Decision: One generation owns a connection

**Choice:** The pure ownership reducer admits exactly one generation for a reusable response connection. A second claim while the first generation is active fails closed.

**Rationale:** Ownership must be explicit before frame parsing begins. Connection identity alone cannot distinguish old and new exchanges.

### Decision: Uncertainty poisons the connection

**Choice:** Interruption, malformed framing, ownership mismatch, or return before the admitted terminal frame transitions the connection into a poisoned state. The host must close or discard it.

**Rationale:** Reuse after an uncertain exchange risks late frames entering a later response. Discarding capacity is safer than mixing state.

### Decision: Terminal completion releases ownership

**Choice:** Only the admitted terminal-frame observation can release generation ownership. End-of-read, timeout, cancellation, and parser error do not imply terminal completion.

**Rationale:** A transport ending is not evidence that the provider completed the response protocol.

## Dependency Direction

```text
provider and transport hosts
            ^
            |
stream parser adapters -- pure generation-ownership reducer
```

The reducer consumes supplied connection, generation, frame, and terminal facts. It does not open sockets, read clocks, retry, or close resources.

## Adversarial Audit

The audit must try to show that Animus:

- admits two live generations for one connection;
- reuses a connection after interruption or malformed framing;
- treats end-of-read as a valid terminal frame;
- accepts a late frame from an old generation;
- reports a poisoned connection as reusable;
- claims transport correctness from ownership metadata.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Poisoning reduces connection reuse after uncertain failures. That cost prevents cross-response corruption.
- Provider terminal semantics differ. Each adapter must bind its exact terminal cohort.
- Ownership metadata cannot stop unrelated host code from misusing the same socket. The live host rail must demonstrate the composed boundary.

## Non-Claims

This change does not prove frame authenticity, exactly-once delivery, network isolation, provider correctness, response semantic completeness, or safe automatic replay.
