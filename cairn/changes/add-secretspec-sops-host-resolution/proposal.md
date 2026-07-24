## Why

Animus keeps credentials outside reusable runtime cores, but it does not yet provide a typed secret-resolution service or concrete std-shell integrations for a provider-independent declaration layer and encrypted files. Consumers therefore either inject ambient environment variables or build product-local adapters with inconsistent admission, redaction, evidence, and process bounds.

SecretSpec and SOPS solve different parts of this problem. SecretSpec declares the secret names and profiles an application expects and resolves them through configured providers. SOPS encrypts structured files at rest. The current released SecretSpec documentation describes a native SOPS provider as proposed rather than released, so Animus must not claim that SOPS is already available through a pinned SecretSpec SDK.

## What Changes

- Add an authority-free `animus-adapter-secrets` contract for typed resolution requests, declaration/profile/key identities, reason metadata, provider capabilities, stable failure classes, and redaction-safe receipts.
- Keep plaintext secret material in a non-serializable, non-debuggable, short-lived delivery value that can cross only an injected host service boundary; reusable cores receive plans and safe outcomes, not provider clients or ambient credentials.
- Add typed Nickel policy for admitted resolver kinds, provider/profile selection, project manifests, SOPS document formats, lexical key paths, count/byte/deadline bounds, allowed environment-key names, audit requirements, and required non-claims.
- Add an optional `animus-std-secrets` shell with a pinned SecretSpec SDK adapter and an independently explicit SOPS adapter.
- Resolve SOPS files through capability-confined encrypted-file reads and bounded explicit-argv process execution, extract only the requested admitted key, retain no decrypted document on disk, and reject malformed, duplicate, missing, oversized, timed-out, truncated, or nonzero outcomes before delivery.
- Add a compatibility gate for a future released SecretSpec native SOPS provider. Until a pinned release passes that gate, direct SOPS mode remains distinct and `secretspec+sops` configuration fails as unsupported.
- Emit safe metadata-only resolution receipts and document that SecretSpec's local audit log is best-effort and is not authoritative Animus completion evidence.
- Keep key discovery, SOPS KMS/age/PGP credentials, SecretSpec provider credentials, filesystem authority, process authority, provider availability, access control, rotation, revocation, and durable audit retention host-owned.

## Success Criteria

- Pure request admission and receipt construction are testable without files, processes, environments, clocks, provider SDKs, or secret values.
- A configured SecretSpec profile resolves one declared key through the injected host service and delivers its value only to the caller-owned execution boundary.
- A configured SOPS file resolves one admitted key through bounded decryption without writing plaintext to a workflow, config, diagnostic, receipt, cache key, run record, or temporary decrypted file.
- Missing declarations, unsupported profiles/providers/formats, unsafe paths, duplicate or unknown key segments, absent provider credentials, timeout, truncation, malformed output, and unavailable executables fail with stable redaction-safe classes.
- Positive and negative tests prove no ambient fallback, no plaintext serialization/debugging, no value-derived hashing, no unbounded process/file behavior, and no false audit or encryption claims.

## Out of Scope

- Replacing SecretSpec provider policy, SOPS encryption, KMS/age/PGP identity management, provider authorization, rotation, revocation, or availability.
- Moving filesystem, process, environment, network, clock, persistence, or credential authority into `animus-core`, `animus-adapters`, `animus-runtime`, or `animus-steel`.
- Injecting a resolved profile wholesale into ambient process environment, accepting plaintext values in Nickel, or persisting decrypted SOPS documents.
- Claiming SecretSpec audit completeness, SOPS cryptographic correctness, arbitrary secret confidentiality, host compromise resistance, provider truth, release eligibility, or production readiness.

## Impact

- **Crates:** new authority-free adapter-family crate, optional std host crate, compatibility re-exports, and feature gates.
- **Configuration:** typed Nickel resolver policy and deterministic generated projection; runtime Rust does not evaluate Nickel.
- **Dependencies:** exact reviewed SecretSpec and SOPS cohorts remain optional and outside no-std/default reusable surfaces; license and feature isolation are checked.
- **Consumers:** Lattice adoption is a separate successor Cairn and cannot begin until this contract and host adapters have passing conformance evidence and an immutable consumable revision.
