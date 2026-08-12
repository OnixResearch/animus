# Evidence: Split Preserves boundary bridges by payload family

## Outcome

- Added `animus-preserves-schema` as the authority-free owner of schema identities and complete family catalog metadata.
- Added `animus-preserves-adapter` as the minimal adapter DTO text/binary bridge. Its manifest has no `animus-runtime`, `animus-preserves`, std-shell, provider, filesystem, process, network, credential, terminal, or persistence dependency.
- Added explicit `animus_preserves::{adapter,runtime,provenance,policy,review}` family paths while preserving existing root facade exports.
- Added session/provenance and policy-visible compatibility records that revalidate safe fields and owning DTOs after decode.
- Kept dataset/export declarations review-only with no encoder, decoder, migration, or stable-wire claim.
- Updated the component catalog, runtime facade inventory, README/root feature guidance, Preserves boundary docs, and Nix checks.

## Baseline

Before implementation, `nix develop -c cargo test -p animus-preserves --locked` passed. Existing runtime and adapter boundary tests were green, so failures introduced after the split would be regressions.

## Focused family and fixture evidence

The following commands passed:

```console
nix develop -c cargo test -p animus-preserves-schema --locked
nix develop -c cargo test -p animus-preserves-adapter --locked
nix develop -c cargo test -p animus-preserves --locked
nix develop -c cargo clippy -p animus-preserves-schema -p animus-preserves-adapter -p animus-preserves --all-targets -- -D warnings
nix develop -c cargo check -p animus-preserves-adapter --locked
nix develop -c scripts/cairn/check-preserves-boundary-schemas --root .
```

Positive fixtures cover adapter model/tool request and response text/binary round trips, safe compatibility metadata, facade parity, session/provenance text/binary round trips, policy-visible text/binary round trips, and reconstruction through owning adapter/runtime DTOs.

Negative fixtures cover malformed text/binary, missing payloads, unsupported schema versions, wrong schema families, wrong payload kinds, invalid owning DTO fields, unknown policy reasons, too many metadata fields, raw-prompt metadata names, provider JSON, private paths, unsafe raw-material markers, and leak-safe diagnostics. Diagnostics assert that rejected private values are not echoed.

`cargo tree -p animus-preserves-adapter --edges normal --prefix none` showed adapter contracts, shared schema primitives, Preserves, Serde, and their normal transitive dependencies; it contained no `animus-runtime` or broad `animus-preserves` package.

## Catalog and documentation evidence

The following rails passed with their positive and negative fixtures:

```console
nix develop -c scripts/cairn/check-component-catalog --root .
nix develop -c scripts/cairn/check-runtime-facade-inventory --root .
nix develop -c scripts/cairn/check-evidence-safety --root .
```

The Preserves checker self-test includes a valid minimal adapter manifest and a negative manifest that introduces `animus-runtime`; the latter is rejected with `PRESERVES_ADAPTER_AMBIENT_RUNTIME`. The current schema catalog records owner, identity, coverage tier, dependency set, safe-metadata policy, wire status, and fixture-set identity for all eight families.

## Workspace evidence

The post-change workspace rail passed:

```console
nix develop -c cargo fmt --all -- --check
nix develop -c cargo check --workspace --all-targets --locked
nix develop -c cargo test --workspace --locked
nix develop -c cargo clippy --workspace --all-targets --locked -- -D warnings
```

## Nix and checked-style evidence

`nix flake check -L` passed on the supported host system. Its Octet check reported `Status: clean`, zero findings, zero warnings, and zero errors. The full flake also ran the new adapter/schema package checks and `preserves-boundary-schemas` rail.

## Cairn evidence

The following lifecycle checks passed:

```console
nix run .#cairn -- validate --root . --change split-preserves-boundary-bridges
nix run .#cairn -- gate proposal split-preserves-boundary-bridges --root .
nix run .#cairn -- gate design split-preserves-boundary-bridges --root .
nix run .#cairn -- gate tasks split-preserves-boundary-bridges --root .
```

No focused rail was skipped. This evidence demonstrates bounded schema/fixture conformance and dependency isolation; it does not claim provider behavior, runtime correctness, release eligibility, persistence safety, or arbitrary cross-language semantic equivalence.
