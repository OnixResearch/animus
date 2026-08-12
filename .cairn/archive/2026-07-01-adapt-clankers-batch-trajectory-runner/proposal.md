# Proposal: Adapt Clankers batch trajectory runner for Animus

## Why

Clankers' batch trajectory runner is a useful bridge between headless agent execution, deterministic replay, evaluation fixtures, and dataset export. Animus already plans headless, session-ledger, and Hugging Face dataset surfaces; a batch trajectory contract will keep those workflows local, resumable, ordered, and safe before export formats are added.

## What Changes

- Define a local-only batch run contract with validated input jobs, deterministic job IDs, bounded concurrency, selected execution mode, output format, and sidecar resume manifest.
- Add pure planning and resume logic that orders jobs deterministically, skips completed jobs only when manifest evidence matches, and fails closed on remote paths or malformed inputs.
- Route execution through injected headless/session services so reusable runtime code does not spawn providers, daemon sessions, or filesystem side effects directly.
- Record run/job metadata with prompt sizes, model/session handles, result status, objective receipt status, output labels, and redaction status without raw prompts, provider payloads, credentials, or remote destinations.
- Add focused fixtures for JSONL input validation, local path policy, resume behavior, stable output ordering, objective metadata, malformed manifests, and receipt redaction.

## Impact

- **Files**: future batch DTOs, planning/resume logic, output renderer interfaces, tests, docs, and Cairn artifacts.
- **Testing**: positive local batch/resume fixtures and negative remote-path/malformed-input/mismatched-manifest/leak fixtures.
- **Non-claims**: this does not add cloud batch execution, remote HTTP/S3 inputs or outputs, provider-specific eval APIs, or a full dataset exporter by itself.
