# Add skill-source lifecycle contracts

## Why

Animus already validates trusted skill manifests, BLAKE3 content identities, deferred disclosure, and host asset drift. It does not distinguish immutable skill sources from moving upstream sources or define how a moving source becomes a reviewable candidate without automatic activation.

A narrow adapter contract can add frozen and tracked-candidate lifecycle semantics. Animus remains authority free, while Mantle or an application host owns acquisition, VCS execution, extraction, storage, credentials, review, and activation.

## What Changes

- Add `Frozen` and `TrackedCandidate` source modes for skill assets.
- Bind safe source labels, exact observed revisions, BLAKE3 tree identities, bounds-profile references, trust state, candidate state, and promotion evidence.
- Require every active skill manifest to resolve to an exact frozen content identity.
- Treat a tracked upstream change as an inactive candidate until explicit current promotion converts it to a frozen manifest identity.
- Extend host asset reconciliation with pure source-currentness, collision, drift, and promotion admission.
- Define a typed Nickel policy for allowed source kinds, bounds profiles, source-mode transitions, promotion requirements, and required non-claims.
- Keep raw URLs, paths, credentials, VCS output, fetched bytes, symlink resolution, extraction, network access, and filesystem mutation outside reusable Animus crates.
- Add metadata-only receipts and positive and negative conformance fixtures.

## Success Criteria

- A moving branch or tag cannot change an active skill silently.
- Frozen sources require an exact revision and matching BLAKE3 tree identity.
- Tracked observations create candidates only and never enable, install, or replace a skill automatically.
- Promotion binds the exact candidate, reviewed policy, source revision, tree identity, manifest identity, and current host observation.
- Duplicate asset identifiers, source collisions, stale observations, unsafe metadata, and identity drift fail closed.
- Default, `no_std`, Wasm, core, runtime, Steel, TUI, and unrelated host builds gain no VCS, network, filesystem, credential, or process authority.

## Non-Goals

- A Git, Radicle, HTTP, package-registry, archive, or filesystem fetcher in Animus reusable crates.
- Automatic updates, automatic promotion, ambient source discovery, or branch-head activation.
- A new trust store, code-signing system, package manager, secret store, or plugin installer.
- Executing skill scripts or granting a skill tool, process, filesystem, network, model, or credential authority.
- Claiming source authenticity, upstream trust, review quality, sandboxing, or release eligibility from a content hash.

## Ownership

- **Animus:** pure source descriptors, validation, reconciliation inputs, promotion admission, and safe receipts.
- **Mantle:** immutable frozen acquisition and build materialization when selected by a product.
- **Application host, such as Lattice:** bounded tracked-source observation and candidate storage.
- **Bounded Exec and Cap Root:** process and lexical extraction mechanisms under host policy.
- **Cairn and product policy:** review and promotion evidence, without becoming runtime authority.

## Reference Boundary

The public `yc-software/qm` skill-pack lifecycle at commit `7f2c916360f1797a8ff2a77ce2ce40c5fabab087` is a requirements reference only. Animus does not copy its Node fetcher, npm workflow, credential model, automatic-update behavior, or deployment layout.

## Impact

- **Affected spec:** `agent-harness-runtime`.
- **Affected code:** a focused `no_std` adapter module beside host asset reconciliation, safe receipt projections, and optional host conformance fixtures.
- **Policy:** typed Nickel source-lifecycle and bounds contracts with a deterministic checked projection.
- **Documentation:** source modes, ownership, promotion flow, consumer obligations, and non-claims.
- **Testing:** positive and negative unit, policy, no-std, Wasm, conformance, redaction, drift, collision, and promotion fixtures.
