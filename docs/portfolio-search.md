# Bounded portfolio search

The bounded portfolio-search facade exported by `animus-runtime` is a pure orchestration core for hard tasks where several materially different mechanisms should be explored before one result is validated. It is suitable for proof routes, architecture alternatives, root-cause hypotheses, security reviews, and optimization strategies.

The core never calls a model, tool, validator, clock, filesystem, process, network, scheduler, or store. A host supplies typed values and applies returned immutable state.

## Contract and phases

A `PortfolioContract` fixes:

- BLAKE3 objective, source-policy, and validator-policy identities;
- evidence classes that count for one bounded result;
- explicit false-completion cases;
- required adversarial risk classes; and
- finite rounds, approaches, tool calls, and token limits.

The state machine moves through `Framing`, `IndependentExploration`, `Synthesis`, optional `TargetedRedirection`, `AdversarialAudit`, and `Validation`. It terminates honestly as `Validated`, `Exhausted`, or `Blocked`.

`Validated` means only that one audited route has a matching supplied validation receipt. It does not mean that an agent task, repository, proof corpus, release, or deployment is complete. Authoritative completion remains a separate Cairn/runtime concern.

## Approach registry

Each `ApproachRecord` keeps a safe ID and family label, BLAKE3 mechanism and claim identities, concrete artifact and evidence references, remaining-gap strength, state, exact blocker, and next discriminating check.

Mechanism fingerprints deduplicate paraphrases. They should be derived by the host from a canonical mechanism description, not raw prompt bytes. Distinct fingerprints are not a semantic proof that two routes differ.

`Equivalent`, `Stronger`, and `Unknown` gaps must remain blocked under matching blocker classes. A blocked route reopens only when it receives a genuinely new mechanism fingerprint or evidence reference and is reclassified with a simpler remaining gap.

## Independent exploration and redirection

During `IndependentExploration`, dispatch metadata must not disclose prior approach IDs. This validates declared context isolation; it cannot prove that a host omitted hidden context.

After synthesis, `underexplored_live_families` returns every family with the minimum live-route count in lexical order. There is no fixed required number of workers or approaches. The host decides whether evidence and remaining budget justify another dispatch.

## Audit and validation

A candidate can enter adversarial audit only when it is active, has concrete evidence, and leaves a simpler gap. One finding is required for every contract risk class. Missing, failed, or unresolved required findings prevent validation.

A `PortfolioValidationReceipt` must match the contract fingerprint, selected approach, mechanism fingerprint, candidate/audit evidence fingerprint, the contract's admitted validator-policy fingerprint, and explicit pass status. The pure core checks identity and state linkage; it does not execute or prove the soundness of the validator.

## Receipt boundary

Terminal `PortfolioReceipt` values retain BLAKE3 identities, bounded counters, phase, selected route identity, safe evidence references, and an exact terminal blocker when applicable. Validators reject raw prompt markers, full home paths, raw tool output, provider payload markers, secret markers, malformed hashes, and post-projection field tampering.

Portfolio receipts do not contain transcripts and do not claim proof semantics, validator soundness, release readiness, repository readiness, or ambient authority.

## Focused checks

```sh
cargo test -p animus-runtime portfolio_search --locked
cargo check -p animus-runtime --lib --locked
cargo clippy -p animus-runtime --lib --locked -- -D warnings
```
