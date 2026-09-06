# Selected-check capture

`animus-std-check-exec` retains bounded private output through an opt-in API.
The original `AgentCheckExecutionService` still returns only metadata.
The packaged completion smoke still omits raw output.

## API

`BoundedCheckExecutor::execute_captured` returns a `CapturedCheck` from one actual run.
The capture has private fields and no public constructor or deserializer.
`observation()` borrows the original metadata observation.
`output()` borrows the pinned Bounded Exec result, without another copy or UTF-8 conversion.
`into_observation()` discards the private bytes without another execution.
`Debug` includes status, counts, and truncation flags, not private bytes.

The result includes both retained stream prefixes, observed counts, truncation flags,
completion, disposition, exit code, and signal.
A successful Rust return can describe a failed, timed-out, cancelled, or truncated child.
The disposition remains authoritative for the bounded process outcome.
A truncated prefix is not a complete record, even when the child exits zero.
An infrastructure or teardown error returns no fabricated capture.

The host preserves the original exact argv allowlist and basename resolution.
It retains null stdin, explicit cwd, and a cleared environment.
Only configured PATH, HOME, TMPDIR, and the fixed NIX_CONFIG reach the child.
The fixed NIX_CONFIG clears builders and secret-key-files.
Stream limits, process-group teardown, and observation hash framing remain unchanged.
The host owns no file-output redirection or record-persistence mechanism.

## Cancellation

`execute_captured_with_cancellation` accepts a caller-owned atomic flag.
The host does not modify the flag.
An already raised flag returns `CancelledBeforeSpawn` at the host checkpoint.
Later cancellation uses the pinned runner and retains its actual result.
No retry or broader fallback occurs.

Cancellation is cooperative, not atomic authorization or effect rollback.
A flag can change after the checkpoint and before the kernel starts the child.
The embedding application still owns current authorization and revocation coordination.
Runner cleanup concerns the direct child and owned pipes.
It does not prove termination of every process that escapes the process group.

## Security and ownership

The bytes remain untrusted and can contain secrets.
The embedding application owns redaction, storage, record validation, and output meaning.
An allowlisted path does not authenticate its executable or prevent path mutation.
The caller must supply an admitted immutable executable namespace and current authority.
The capture does not authenticate its caller or attest a native build.
It grants no sandbox, hermeticity, input-integrity, artifact-correctness, or promotion claim.

This change adds no dependency, default feature, authority core, or lock update.
The crate remains AGPL-3.0-or-later.
The OnixResearch owner grant covers owner-controlled consumer reuse without changing third-party notices.

## Checks

```sh
cargo test -p animus-std-check-exec --features test-fixtures --locked
cargo clippy -p animus-std-check-exec --all-targets --all-features --locked -- -D warnings
scripts/cairn/check-real-check-completion-host-smoke --root .
```

The real-child controls cover binary bytes, nonzero exit, signal, timeout,
each stream overflow, exact bounds, command drift, missing executables,
pre-spawn cancellation, running-child cancellation, and explicit process context.
A compile-fail control rejects public capture construction.
The original completion controls retain their metadata-only output checks.
These controls use fixture children, not candidate training or production services.
