# Design: Skill-source lifecycle contracts

## Context

`animus-adapters` already accepts host-supplied asset manifests and observations. It validates BLAKE3 identities, trust state, freshness, drift, dry-run actions, and mutation requests without performing host effects.

That contract identifies content but does not identify source lifecycle. A branch can move, an observed tree can differ from a declared revision, and a candidate can be confused with an active asset unless mode and promotion facts are explicit.

## Success Contract

Completion requires pure frozen and tracked-candidate models, exact content binding, explicit promotion, typed Nickel policy, host-authority separation, safe evidence, and paired failure fixtures. A source URL, branch name, successful clone, or matching asset label is not completion.

## Architecture

```text
host-owned source observation
  -> pure source descriptor validation
  -> pure manifest and installed-state reconciliation
  -> inactive candidate or exact frozen identity
  -> explicit host review and promotion evidence
  -> pure promotion admission
  -> host-owned mutation request under existing capability policy
```

Reusable Animus crates receive loaded metadata only. They do not discover sources, resolve revisions, fetch objects, traverse trees, execute VCS tools, inspect archives, open paths, read credentials, or mutate stores.

## Source Modes

### Frozen

A frozen descriptor binds:

- stable skill asset identity;
- safe source-kind and origin labels;
- exact source revision;
- lowercase BLAKE3 tree identity;
- exact manifest content identity;
- bounds-profile and policy references;
- provenance, trust, and review references.

An active skill manifest must use a frozen identity. The observed content, declared tree, and manifest identity must match under the reviewed canonicalization profile.

A tag or named reference is not immutable by itself. The descriptor still requires an exact resolved revision and tree identity.

### Tracked candidate

A tracked-candidate descriptor binds:

- stable skill asset identity;
- safe source-kind and origin labels;
- tracking-label identity;
- exact observed revision and BLAKE3 tree identity;
- prior frozen identity when one exists;
- observation identity and freshness epoch;
- bounds-profile, policy, and provenance references;
- candidate outcome and safe diagnostics.

The tracked form is discovery input only. It cannot be active, trusted for use, enabled, installed, or selected by runtime composition.

When the observed revision changes, reconciliation emits a new candidate or a no-op. It does not mutate the active frozen manifest.

## Promotion

Promotion is an explicit exact transition from one current candidate to one frozen manifest identity. The request binds the candidate, source revision, tree identity, manifest identity, review, policy, host observation, requested action, and capability label.

The pure core denies promotion when:

- candidate or host observation is stale;
- content or revision differs;
- the asset identifier collides;
- the bounds profile differs or is missing;
- trust or review evidence is absent;
- another candidate or active identity conflicts;
- the request attempts automatic enablement or source replacement;
- source metadata contains paths, credentials, URLs with user information, tokens, or arbitrary tool output.

A passing promotion admission authorizes no mutation by itself. The existing host asset mutation boundary still requires an injected service, a fresh dry-run plan, exact action identity, and capability label.

## Typed Nickel Policy

Nickel owns the reviewable source-lifecycle policy. Its contracts define:

- allowed source kinds and safe label grammar;
- mode-specific required fields;
- canonical tree and manifest identity profiles;
- positive file-count, byte, path-depth, entry-type, process, and observation bounds;
- allowed symlink, archive, and submodule posture;
- source-mode transition rules;
- review and promotion requirements;
- safe diagnostic classes and mandatory non-claims.

The checked runtime projection is deterministic. Rust consumes the generated projection and does not evaluate Nickel at runtime.

No policy field contains a raw credential, private URL, full host path, fetched content, provider response, or VCS output.

## Functional Core and Imperative Shell

### Functional core

A focused `no_std` adapter module beside `asset.rs` owns:

- source descriptor validation;
- frozen identity matching;
- tracked candidate classification;
- source and asset collision detection;
- freshness and drift decisions;
- promotion admission;
- metadata-only receipt validation.

The core takes immutable inputs and returns typed outcomes. Tests need no filesystem, process, network, clock, VCS, store, or credential service.

### Imperative shell

A consuming host owns:

- remote and local source configuration;
- credential references and resolution;
- VCS or transport execution;
- deadlines, cancellation, stream bounds, and teardown;
- archive and tree inspection;
- capability-relative extraction and no-follow policy;
- candidate storage and retention;
- review workflow and active-store mutation.

A host can use Mantle for frozen materialization. A host that observes a moving source can use Bounded Exec for an authorized VCS client and Cap Root for lexical extraction admission. These mechanisms do not grant source trust or activation authority.

## Canonical Identity

BLAKE3 identifies stack-owned canonical public metadata and skill trees. The policy names the canonicalization profile so file order, path form, metadata inclusion, and entry types are deterministic.

A BLAKE3 match proves byte identity under that profile only. It does not prove authorship, review quality, safety, authority, or source authenticity.

## Collision and Path Posture

One manifest cannot contain duplicate skill asset identifiers. A source cannot replace another source's active asset through a matching display name.

Unsafe relative paths, absolute paths, parent traversal, unsupported entry types, escaping links, duplicate normalized paths, and policy-denied submodules or symlinks must become host-supplied denial facts. The pure contract validates those facts and refuses candidate or promotion admission.

Animus does not claim that metadata validation prevents filesystem substitution. The host retains capability handles and no-follow I/O policy.

## Evidence

Safe receipts contain only source mode, safe source kind and origin labels, opaque revision identity, BLAKE3 tree and manifest identities, asset identity, bounds and policy references, observation freshness, drift class, promotion outcome, action identity, capability label, and fixed diagnostics.

Receipts exclude raw URLs, full paths, credentials, tokens, VCS arguments, environment values, source bytes, file bodies, prompts, scripts, tool output, stdout, stderr, and unrestricted errors.

## Consumer Rollout

- Existing host asset manifests remain compatible and have no source-lifecycle claim.
- Consumers opt into source descriptors explicitly.
- Frozen-only consumers can ignore tracked candidates.
- A tracked consumer must retain the active frozen identity until explicit promotion and mutation complete.
- Cross-repository consumers must pin an immutable Animus revision. Workspace-relative paths are test inputs only.

## Failure Behavior

Malformed references, invalid BLAKE3 values, empty revisions, unsupported modes, missing bounds, stale observations, mismatched trees, duplicate assets, source collisions, unsafe metadata, absent review, automatic activation, conflicting promotion, missing host service, and plan drift fail closed.

Fetch, VCS, transport, credential, extraction, and store errors remain host outcomes. The pure core does not retry, fall back, or infer success from partial data.

## Validation

Positive fixtures cover an exact frozen source, unchanged tracked observation, changed inactive candidate, explicit promotion, compatible legacy manifest, and safe receipt.

Negative fixtures cover moving tags without exact revisions, tree mismatch, stale observation, duplicate asset, source collision, unsafe metadata, missing bounds, denied symlink or submodule facts, automatic activation, conflicting promotion, missing service, plan drift, receipt leakage, and feature contamination.

Maintained checks cover adapter tests, Nickel export freshness, no-std and Wasm compilation, component and contract indexes, safe receipts, Cairn validation and gates, and the smallest relevant Nix checks.

## Reference and Claim Boundary

QM commit `7f2c916360f1797a8ff2a77ce2ce40c5fabab087` informs only the tracked-versus-frozen lifecycle scenarios. QM tests were not run, and its implementation is not conformance or security evidence for Animus.

Passing this change proves selected descriptor, reconciliation, promotion-admission, policy-projection, and receipt facts. It does not prove source authenticity, network safety, VCS correctness, extraction safety, review quality, script safety, host confinement, runtime authority, or release eligibility.
