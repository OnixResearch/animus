## Context

SecretSpec separates a checked-in declaration from provider-owned values. Its Rust SDK can select a manifest profile and provider, validate required keys, resolve values, materialize path secrets, and emit metadata-only local audit records. SOPS instead decrypts YAML, JSON, ENV, INI, or binary files using host-owned AWS KMS, GCP KMS, Azure Key Vault, age, or PGP authority.

The reviewed SecretSpec material explicitly describes these as different layers. On 2026-07-23 its documentation called native SOPS support an open proposal. Animus therefore needs two explicit host mechanisms behind one authority-free request contract, not a fabricated claim that the released SecretSpec cohort already implements SOPS.

## Success Contract

Completion requires one pure contract plus two optional, bounded host adapters with deterministic positive and negative conformance evidence. Passing compilation, resolving only environment-backed examples, invoking `sops` through an unrestricted shell string, or logging a provider success without proving plaintext omission is insufficient.

False completion includes:

- storing a `String` secret inside a serializable/debuggable request, result, event, receipt, or runtime state;
- hashing secret bytes into cache or evidence identity;
- inheriting the complete host environment into a SOPS process;
- writing a decrypted document to a temporary path;
- treating SecretSpec's non-blocking local audit as durable authoritative evidence;
- enabling a `secretspec+sops` mode from unreleased documentation or an unpinned branch;
- describing successful decryption as proof of authorization, encryption correctness, confidentiality, or provider truth.

## Architecture

```text
Nickel resolver policy -> checked generated projection
                         |
                         v
SecretResolutionCore (pure)
  descriptor + request + explicit capability facts
  -> admitted plan | stable rejection
                         |
                         v
Injected SecretResolutionService (host-owned)
  +----------------------+--------------------+
  |                                           |
SecretSpecHost                              SopsHost
pinned SDK/provider config                 capability file + bounded child
  |                                           |
  +-------------> EphemeralSecret ------------+
                    |
                    v
             caller-owned effect boundary
                    |
                    v
             safe metadata receipt core
```

`animus-adapter-secrets` owns DTOs, admission, failure classes, provider capabilities, and safe receipt construction. It is `no_std + alloc`, performs no I/O, and depends only on authority-free crates. `animus-std-secrets` owns concrete SecretSpec SDK use, encrypted file access, bounded SOPS process invocation, output parsing, ephemeral memory, and shell diagnostics.

## Decisions

### Decision: model declaration and storage separately

**Choice:** Requests name an Animus resolver id, SecretSpec project/profile/key identities or a SOPS document/key-path identity, a safe purpose/reason code, a destination binding, and explicit bounds. Resolver kind is a closed enum. SOPS is not represented as a SecretSpec provider until a pinned released SDK proves that capability.

**Rationale:** SecretSpec declares and resolves application requirements; SOPS encrypts files. Preserving that distinction prevents storage encryption from being promoted into application-interface, authorization, or audit claims.

### Decision: keep material outside serializable DTOs

**Choice:** Planning DTOs and receipts contain safe labels, reference identities, policy identities, counts, statuses, and BLAKE3 hashes of non-secret configuration. Plaintext is returned only in an `EphemeralSecret` host value that omits `Serialize`, `Deserialize`, `Clone`, `Debug`, equality, display, and content hashing and exposes a borrow only to the immediate caller-owned delivery closure.

**Rationale:** Type shape should make accidental persistence and logging harder. Animus does not hash values because low-entropy secrets can be attacked through value-derived digests.

### Decision: use a pure admission core and thin host shells

**Choice:** Admission consumes a request, loaded policy, provider capability facts, and explicit availability facts. It checks resolver identity, project/profile/key shape, purpose, destination, declared capability, format, lexical path/key segments, and named bounds before returning a plan. Shells perform only the admitted I/O and return a typed observation for pure receipt construction.

**Rationale:** All policy branches remain testable without standing up SecretSpec, SOPS, KMS, files, or processes.

### Decision: pin SecretSpec and isolate it behind an optional host crate

**Choice:** Implementation selects an exact released SecretSpec SDK cohort after license/API review and enables it only in `animus-std-secrets`. Manifest and machine provider configuration remain caller-selected. The adapter requests one admitted key and does not call `set_as_env_vars`, import/migration, generation, provider login, or bulk profile injection.

**Rationale:** Direct SDK use avoids parsing secret-bearing CLI output while keeping provider network/filesystem behavior in an opt-in std shell. Narrow per-key delivery minimizes secret spread.

### Decision: support SOPS through bounded explicit execution

**Choice:** Initial SOPS support invokes an exact configured `sops` executable with explicit argv through the shared bounded-process mechanism. It capability-opens one encrypted file beneath an admitted root, allows a documented structured-format subset, requests decrypted output over a bounded pipe, parses into a duplicate-rejecting in-memory representation, extracts one lexical key path, and drops the document immediately. Stdout truncation, stderr output, timeout, cancellation, launch failure, nonzero exit, unsupported format, malformed structure, duplicate keys, missing key, and oversized selected values fail before delivery.

**Rationale:** SOPS is a Go application with an established file format and key-service ecosystem. A bounded process adapter preserves that implementation boundary and avoids vendoring cryptographic/key-service behavior into Animus.

### Decision: make provider credentials explicit but opaque

**Choice:** Policy may allow named environment keys or other host credential channels required by a selected SecretSpec/SOPS provider, but contains no values. The shell constructs the smallest explicit environment from caller-supplied opaque material and never records those values or their hashes. Missing credentials are a typed pre-effect failure where the provider can declare them; provider-native denials remain fixed safe classes.

**Rationale:** KMS, cloud, age, PGP, keyring, and password-manager authentication differ. Animus can bound delivery without owning their authority or pretending all failures are distinguishable in advance.

### Decision: separate Animus receipts from SecretSpec audit

**Choice:** Animus receipts record resolver kind/id, declaration/project/profile/key reference hashes, reason code, destination class, policy/config identities, bounds, outcome class, and redaction evidence. SecretSpec audit metadata may be observed as advisory correlation only. Audit write failure never becomes a false success or false denial, and Animus does not claim complete access history.

**Rationale:** SecretSpec documents auditing as local, capped, truncating, and non-blocking. It is useful operational metadata, not authoritative durable evidence.

### Decision: gate future native SecretSpec SOPS support

**Choice:** A fixture-backed compatibility gate verifies an exact released SecretSpec version, provider name/URI, read behavior, address mapping, error classes, and plaintext/redaction boundaries before enabling `secretspec+sops`. Until then, configuration for that combined mode fails with `unsupported_provider_cohort`; direct SOPS remains available.

**Rationale:** Documentation from a branch or open pull request is not release evidence.

## Adversarial Audit

| Family | Mechanism | Result |
|---|---|---|
| Environment injection | Resolve a whole profile and mutate process-global environment | Rejected: ambient, difficult to isolate, and unnecessarily broad. |
| SecretSpec CLI JSON export | Parse `secretspec export --format json` | Rejected for the initial Rust host: it broadens plaintext process/output exposure compared with the SDK. |
| Embed SOPS internals | Reimplement or bind SOPS cryptography/key services | Rejected: expands authority and review surface and diverges from the established implementation. |
| Unbounded SOPS child | Run a shell command and parse arbitrary stdout | Rejected: shell injection, environment inheritance, hangs, truncation ambiguity, and diagnostic leaks. |
| Distinct bounded hosts | One pure request contract with pinned SecretSpec SDK and explicit bounded SOPS adapters | Selected: preserves layer, authority, and release boundaries. |

The selected design must survive malformed manifests, unknown profiles/keys, traversal and symlink escape, executable substitution, hostile encrypted filenames, duplicate JSON/YAML keys, decompression/output growth, invalid UTF-8 where text is required, binary confusion, inherited credential leakage, signal/timeout/cleanup uncertainty, provider errors containing sensitive text, value-derived hash attempts, audit-write failure, and an unavailable native SecretSpec SOPS cohort.

## Compatibility and Non-claims

The feature is opt-in. Existing core/runtime/std users acquire no credential provider, SOPS executable, manifest search, filesystem read, environment read, or network behavior. Compatibility re-exports remain metadata/service contracts unless the new feature is selected.

Successful resolution proves only that the selected adapter observed one bounded value for one admitted request. It does not prove the value is correct or fresh, the caller is globally authorized, SOPS or its key services are correct, SecretSpec provider policy is sufficient, audits are complete, memory cannot be inspected, host paths are safe outside the capability, external effects are unique, or the system is production ready.
