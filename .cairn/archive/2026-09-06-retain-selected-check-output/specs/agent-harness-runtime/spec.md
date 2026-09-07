# Selected-check capture

## ADDED Requirements

### Requirement: Retain actual bounded output

r[agent_harness_runtime.check_capture.retention] The selected-check host MUST offer an opt-in private capture of the exact bounded stdout, stderr, completion, disposition, truncation, and metadata observation from one actual execution. Infrastructure failure MUST NOT produce a fabricated capture. Capture MUST NOT claim executable authenticity, caller authority, sandboxing, output correctness, or native materialization.

#### Scenario: Complete binary output is retained

- GIVEN a reviewed child that emits bounded binary data on both streams
- WHEN the capture method executes it
- THEN the returned buffers MUST equal the observed bytes without UTF-8 conversion or replacement

#### Scenario: Failed or excessive output is retained honestly

- GIVEN a child that exits nonzero, receives a signal, times out, or exceeds either output bound
- WHEN the capture method returns
- THEN it MUST preserve the actual failure facts and bounded prefixes without reporting a passing check or complete truncated record

### Requirement: Preserve existing admission and safe receipts

r[agent_harness_runtime.check_capture.compatibility] The capture path MUST reuse the existing exact command admission, cleared environment, finite stream limits, null stdin, process-group teardown, and observation hash framing. The original service and smoke receipts MUST remain metadata-only. Capture Debug MUST NOT reveal output bytes.

#### Scenario: Command drift fails before effects

- GIVEN a request whose command differs from its configured allowlist
- WHEN either execution API runs
- THEN it MUST reject the request before spawning a child

#### Scenario: Raw bytes remain opt-in

- GIVEN a child containing private text or binary output
- WHEN ordinary execution or Debug rendering runs
- THEN its metadata output MUST NOT include the private bytes

### Requirement: Cancellation is explicit and bounded

r[agent_harness_runtime.check_capture.cancellation] A cancellable capture MUST use a caller-owned flag and the pinned bounded runner. A flag already raised at the host's pre-spawn checkpoint MUST reject execution. Cancellation after that checkpoint MUST retain the runner's actual outcome and MUST NOT imply rollback or atomic authority revocation.

#### Scenario: Pre-cancelled request has no child effect

- GIVEN an exact allowed request and an already raised cancellation flag
- WHEN the cancellable host method runs
- THEN it MUST return a pre-spawn cancellation error without a child marker file

#### Scenario: Running child is cancelled

- GIVEN an exact allowed bounded child and a flag raised while it runs
- WHEN the host observes cancellation
- THEN it MUST return a non-passing cancellation disposition after the runner completes its owned teardown
