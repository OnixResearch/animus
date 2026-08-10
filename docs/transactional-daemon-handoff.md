# Transactional daemon handoff

Animus can replace one session worker without moving live process state. The handoff starts only from exact idle state. It finishes only after the replacement proves exact readiness.

The pure transaction core is `animus_runtime::daemon_handoff`. Versioned host DTOs and service ports are in `animus_adapter_daemon`. The injected shell is `animus_std::daemon_handoff_host`. The shell is available only with the non-default `daemon-handoff` feature.

## Ownership boundary

The runtime core owns deterministic validation, state reduction, BLAKE3 identities, Host Control status projection, and metadata-only receipts. It does not read files, open endpoints, spawn processes, manage leases, inspect clocks, load credentials, or persist state.

The adapter crate owns versioned requests, observations, and implementation-free service traits. It does not supply a process manager, endpoint server, lease store, or session store.

The standard host calls only services supplied by its caller. These services own ingress control, state commits, lease mutation, process trees, authenticated local endpoints, readiness checks, and old-worker exit.

## Strict idle admission

Handoff admission requires one exact snapshot. The snapshot binds the worker, session, branch, current lease generation, queue generations, active-turn generation, and authority profile.

Every work class must be idle:

- provider and tool work;
- repair and confirmation work;
- cancellation and durable commit work;
- terminal transitions;
- the primary queue;
- child queues;
- steering and follow-up work.

An active or ambiguous class blocks preparation. Empty display state or one empty queue is not sufficient.

## Successful order

The host keeps the transaction order explicit:

1. Validate exact idle state and prepare one handoff identity.
2. Pause primary, child, steering, follow-up, command, attachment, and remote ingress.
3. Commit the durable handoff state.
4. Release the old worker lease.
5. Start one owned replacement process attempt and endpoint.
6. Admit one authenticated claimant for the exact released generation.
7. Validate the exact session head, composer-draft identity, vault generation, active-turn generation, authority profile, policy, and service state.
8. Commit replacement readiness.
9. Ask the old worker to exit.
10. Record the complete transition.

The old worker cannot exit before readiness commits. A claimant with a stale generation, wrong process, wrong endpoint, wrong protocol, stale authentication, or duplicate ownership fails closed.

## Cancellation and rollback

Cancellation is a forward transaction. It never removes prior evidence.

The host commits cancellation before it requests process termination. It then observes the complete owned process tree. A live leader or descendant blocks lease restoration.

Before the lease effect, the pure core validates all restoration preconditions. The authority profile and durable store head must still match. The restored lease must use the exact next generation.

Ingress resumes only after the old worker owns that restored lease. Spawn failure can restore through an explicit zero-process terminal observation. It cannot invent process ownership or issue teardown for a process that never started.

## Host Control projection

The pure core maps handoff states to the existing Host Control semantic status vocabulary. Prepared, paused, released, started, claimed, and ready states map to `waiting`. Complete maps to `done`. A restored old-worker lease maps to `idle`.

This mapping does not give Host Control lease, process, endpoint, session-store, or handoff authority. Tile can later consume these semantic observations through a separate Tile-owned change. That future work must own transport, PTYs, topology, user policy, and product integration.

## Session and pending-input composition

The committed handoff state binds the durable coding-session head and composer-draft identity. It also binds pending-input and active-turn generations. The replacement must prove those exact values before readiness.

Handoff does not load pending-input bodies or submit restored work. The [pending-input vault](pending-input-vault.md) still requires explicit review and current admission. The [active-turn recovery](active-turn-recovery.md) contract still owns accepted-work recovery and provider gates.

The coding-session host still owns transcript commits, queues, provider dispatch, tools, and continuation. Handoff grants no new authority to those owners.

## Evidence

Receipts contain bounded worker, session, lease generation, authority profile, transition, endpoint, process-attempt, readiness, cancellation, vault generation, redaction, non-claim, and BLAKE3 identity metadata.

Receipts reject draft text, pending input, prompts, transcripts, credentials, socket paths, process arguments, provider data, terminal data, and raw errors.

## Non-claims

This boundary does not transfer PTYs, terminal state, process memory, provider sessions, credentials, physical paths, or raw process arguments.

It does not prove sandboxing, remote security, exactly-once effects, power-loss durability, production readiness, or release eligibility.

## Autolith reference boundary

The reviewed behavior reference is Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871`, under the ISC license. Animus adapted restart-order and stale-claim scenarios only.

Animus did not adopt Autolith storage formats, supervisor scripts, saved images, global mutable state, or runtime dependencies.

## Validation

Run from the repository root:

```sh
scripts/cairn/check-transactional-daemon-handoff --root .
cargo test -p animus-runtime handover::tests --locked
cargo test -p animus-adapter-daemon transaction::tests --locked
cargo test -p animus-std --features daemon-handoff handover_shell::tests --locked
cargo run -p animus-std --features daemon-handoff --bin animus-daemon-handoff-smoke --locked
nix build .#checks.x86_64-linux.transactional-daemon-handoff --no-link -L
```

The focused receipt is `target/cairn/focused-rails/transactional-daemon-handoff.md`. It is bounded evidence for this transaction seam. It is not a production or release certificate.
