## Context

`BoundedCheckExecutor` already receives complete bounded captures from Bounded Exec.
Its metadata-only service discards them after computing the established hashes.
Neural Stream needs the actual native report, not a hash of unavailable bytes.
Animus owns this host adaptation; Bounded Exec retains process mechanics.

## Decisions

### Decision: Retain one actual execution

**Choice:** Add `CapturedCheck` with private observation and output fields, borrowed accessors, and metadata-only Debug. Construct it only after the existing owner runner returns and the existing observation encoder succeeds. Do not add Clone, deserialization, or a public constructor. Expose the pinned Bounded Exec output contract through a shared reference rather than translating away disposition or truncation.

**Rationale:** The exact bytes and observation remain associated with one run. Failed and truncated runs stay observable but never become passing results. Infrastructure/teardown errors return no invented capture. Bytes remain untrusted and private; callers own redaction, interpretation, persistence, authority, and immutable executable resolution.

### Decision: Preserve the old service

**Choice:** The old `execute` method consumes the captured result and returns only its original observation. Keep the exact allowlist, basename resolution, explicit paths, null stdin, environment clearing, accepted exit code, output bounds, process-group teardown, and hash framing unchanged.

**Rationale:** Existing completion and smoke receipts remain metadata-only. Raw bytes do not gain completion authority. This is not a new universal execution abstraction.

### Decision: Observe explicit cancellation

**Choice:** Add a capture method accepting a caller-owned atomic cancellation flag. A flag already raised at the host checkpoint returns a pre-spawn cancellation error. Later cancellation uses the pinned runner. No global cancellation source, retry, or broader fallback is introduced.

**Rationale:** The embedding authority service can stop owned work. Cancellation remains cooperative and does not serialize arbitrary authority revocation with the kernel spawn. The caller still owns current authorization. Cleanup covers the runner's direct child and pipes, not every process that can escape its process group.

## Risks / Trade-offs

- Retained output can contain secrets. No raw-output Debug or serialization is provided.
- A configured path is not executable authenticity; hosts must prevent path mutation and spoofed executables.
- Observed byte counts can exceed retained prefixes. Consumers must inspect truncation and disposition before record decoding.
- A successful capture does not prove the command's reported meaning or complete side-effect cleanup.
- No dependency, default feature, authority core, policy, or lock changes are required.
