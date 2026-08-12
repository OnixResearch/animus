# Evidence: Evaluate bounded-execution contract

## Verdict

PASS for the bounded feasibility evaluation.

The terminal architecture decision is `blocked`. Animus, Octet, Nickel Export, and Mantle do not yet share a lossless mandatory execution contract. This result is an accepted terminal outcome. It does not approve repository extraction, dependency changes, DTO moves, or consumer cutover.

## Reviewed cohort

The evaluation binds these exact immutable source identities:

- Animus revision `0c1f6dce0ed11b4b848df0b4f56cac49bd8c19ab`, source BLAKE3 `196c9a78574536f5012995a386f6fbef87fd95ce3ea9b5a73dfdaa90384345b3`.
- Octet revision `d87153a1bbfe4c3469b2dee6fb5512eafa812d88`, source BLAKE3 `388f36d5f7aebeef6d46a7f897b83d7a8b9393981a8712df366b8426ee770e59`.
- Nickel Export revision `0de12cd4400cf0df6f4f66b235d5dba9ac0b2870`, source BLAKE3 `51f81fcb325517b62f644ce81577bf76d4f6cda72c75fe1bedc80612f6f38a17`.
- Mantle revision `3854efbd7ed144ecafabd694a3913c5470cb7180`, source BLAKE3 `46d228d83f855fe9e3f8d3ea8bf434c8a42dabd0bc52fbcaf1f65a7edf9314e8`.
- Frozen Lattice consumer fixture revision `5c9902f777412296c13e2347e45b3bede335b828`.

The profile records each consumer's license, dependencies, mechanisms, semantics, capabilities, outcomes, retained authority, and non-claims.

## Decision evidence

`scripts/cairn/check-bounded-execution-evaluation --root .` passed.

- Terminal status: `blocked`.
- Profile BLAKE3: `bea3653761c45b70e875ab882da83fed19da45297c00eb45e5d556f5c2b41ea1`.
- Decision BLAKE3: `cd26505717c90cefffda7eaf5315e68ed28dcd57ff4a2fce67230ba8810b8958`.
- Consumers: 4.
- Semantic classes: 12.
- Capability mappings: 26.
- Outcome mappings: 44.
- Blockers: 6.
- Next action: dual-run an outcome-only mapping in Octet and Nickel Export without sandbox capability claims.
- Focused receipt: `target/cairn/focused-rails/bounded-execution-evaluation.md`.

The blockers cover signal, cancellation, network, filesystem, timeout, output, and cleanup differences. The profile preserves requested, configured, enforced, observed, unavailable, unsupported, and not-applicable states. It does not collapse these facts into a sandbox boolean.

## Conformance and failure evidence

The checked fixture corpus covers success, nonzero exit, launch denial, missing executable, environment rejection, path or root escape, timeout, cancellation, signal termination, output overflow, network-policy differences, cleanup failure, unsupported mechanisms, stale source identity, unsafe diagnostics, and overclaim wording.

The pure Rust self-tests cover all three terminal statuses. Negative cases cover duplicate and missing consumers, missing semantics, invalid source identities, unknown capability states, unknown outcomes, semantic-loss laundering, dependency cycles, single-consumer extraction, unsafe fixture content, weakened non-claims, and universal-equivalence wording.

The frozen Lattice fixture maps only reviewed exit and signal facts. It does not grant Lattice authority or claim current integration.

## Authority evidence

The Nickel projection and Rust decision core consume already-loaded bounded records. They do not launch a consumer executor or sandbox. The shell only reads bounded repository files, evaluates the checked Nickel projection, compiles and runs the repository-owned checker, and writes an ignored metadata-only receipt.

No standalone repository, runtime dependency, public DTO move, or consumer cutover was created. Existing products retain process launch, filesystem, environment, network, clock, signal, child-reaping, timeout, cancellation, output, cleanup, sandbox, proof, build, and release authority.

## Verification

The following checks passed:

- `scripts/cairn/check-bounded-execution-evaluation --root .`
- `scripts/cairn/check-nickel-policy-contracts --root .`
- `cargo test -p animus-runtime --locked processjobprofiles`
- `scripts/cairn/check-sandlock-host --root .`
- `scripts/cairn/check-agent-guardrail --root .`
- `scripts/cairn/check-component-catalog --root .`
- `scripts/cairn/check-adapter-conformance-index --root .`
- `cargo fmt --all -- --check`
- `cargo test --workspace --locked`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `nix build .#checks.x86_64-linux.bounded-execution-evaluation --no-link -L`
- `scripts/cairn/check-evidence-safety --root .`
- `nix run .#cairn -- validate --root .`
- proposal, design, and tasks Cairn gates

The configured remote builder was unavailable. Nix continued with the local builder.

## Non-claims

This evaluation does not prove sandbox equivalence, hermeticity, network denial, filesystem isolation, executor correctness, proof soundness, build correctness, future determinism, or release eligibility. It proves only the reviewed DTO and classification facts for the pinned source cohort.
