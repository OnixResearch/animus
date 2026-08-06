# Runtime authority seal

Animus seals one immutable authority unit before one dependent dispatch. The seal closes a gap between session-level authority admission and effect start.

The seal does not grant authority. Existing policy, capability, provider, tool, credential, sandbox, timeout, cancellation, and cleanup owners remain authoritative.

## Ownership

`animus-runtime::runtime_authority_seal` is the pure core. It validates supplied facts, computes identities, prepares receipts, and applies state transitions. It does not read files, environment variables, clocks, networks, processes, providers, credentials, or mutable global state.

`animus-std::runtime_authority_seal_host` is the optional host shell. Enable it with the `runtime-authority-seal` feature. The shell uses injected services for runtime observation, receipt commit, current enforcement recheck, effect dispatch, and cleanup.

The `coding-session-host` feature includes this shell. The first pilot uses deterministic fake services. It does not contact a provider or execute a real tool.

## Unit lifecycle

A unit has these states:

1. `preparing`
2. `sealed`
3. `running`
4. `completed`, `cancelled`, or `failed`

The pure core prepares a seal plan and a metadata receipt. The host must commit the exact receipt identity before it can mark the unit as sealed. The host then obtains a fresh enforcement recheck before effect start.

A terminal unit cannot run again. A caller must prepare a new unit when valid later policy changes authority.

## Sealed identity

The domain-separated BLAKE3 unit identity binds these safe values:

- session and dispatch labels;
- session-lineage identity;
- durable application-host binding identity;
- session authority-profile identity;
- realized enforcement-observation identity;
- canonical operation identities and operation-set identity;
- adapter-cohort identity;
- resource-profile and resource-budget identities;
- resource identities;
- runtime-services identity;
- every required runtime-view class and identity.

Input arrays use canonical order. Checkout paths, clocks, randomness, locale, secrets, prompts, payloads, and model output do not affect the identity.

Every required runtime-view class must have one current observation. Each view identity must also match its declared unit identity. Missing, unexpected, duplicate, stale, unknown, unsupported, or mismatched observations block seal preparation.

## Monotonic authority

A sealed or running unit cannot gain authority. Animus rejects these mutations:

- capability or operation addition;
- profile rebind;
- adapter replacement;
- resource expansion;
- ambient authority discovery;
- fallback to another authority source;
- repeated sealing of the same unit.

A current recheck can narrow or deny the unit. A narrowed recheck does not authorize the current dispatch because the pilot requires the exact sealed effect scope. A wider recheck fails with a post-seal-widening issue.

## Evidence boundary

The seal receipt contains safe identities, state, required commit ordering, a redaction marker, and explicit non-claims. It excludes raw grants, descriptors, paths, credentials, prompts, provider payloads, tool bodies, model output, and environment values.

A passing receipt does not prove:

- authorization;
- operating-system isolation;
- descriptor closure or hidden-authority absence;
- sandbox or host integrity;
- effect correctness;
- release eligibility.

The host report records this event order:

1. unit prepared;
2. seal receipt committed;
3. current enforcement rechecked;
4. effect started;
5. effect completed;
6. cleanup completed.

These events describe injected-service observations only.

## Stack integration

Kamacite owns the canonical application-host binding artifact. Animus stores its identity as the durable binding input. Animus does not replace Kamacite identity rules.

The session authority profile and realized enforcement observation remain separate inputs. Their existing admission and drift rules still apply.

The later application-host pilot must use one frozen Kamacite artifact and receipt. That pilot must verify the same identity independently before it prepares an Animus unit.

## Bounded design reference

Redox OS provides a design signal for explicit resource handles and narrow process boundaries. Animus adopts no Redox runtime, syscall, namespace, scheme, capability, or isolation mechanism in this change.

The seal remains a metadata contract over supplied identities. Separate host evidence must support operating-system isolation claims.

## Verification

Run the focused tests:

```text
cargo test -p animus-runtime authorityseal --locked
cargo test -p animus-std --features runtime-authority-seal runtime_authority --locked
cargo clippy -p animus-runtime --all-targets --locked -- -D warnings
cargo clippy -p animus-std --features runtime-authority-seal --all-targets --locked -- -D warnings
```

The tests include valid lifecycle, deterministic identity, exact view matching, receipt safety, pre-seal denial, current-recheck denial, post-seal widening, terminal reuse, effect failure, and cleanup cases.
