# Process-job profiles

Process-job profiles are project-defined, backend-neutral start profiles for long-running work. They let a host shell name bounded work such as `quick-check` without embedding ad hoc shell strings in prompts or reusable runtime code.

Resolving a profile is pure: Animus accepts already-loaded profile sources and policy, validates them, and returns a backend-neutral `StartProcessJobRequest` plus `ProcessJobProfileIdentity`. The resolver does not read files, spawn a process, contact pueue or systemd, write storage, open the network, or notify users. Hosts own those imperative steps after a request has passed validation.

## Policy

The policy source is `policy/process-jobs/policy.ncl`. Nickel owns the schema for backend choices, command/program shape, args, cwd, writable paths, environment allowlists, timeout/memory/CPU/log limits, notification defaults, and safe identity metadata. Checked JSON fixtures live under `policy/process-jobs/generated/` and are verified by `scripts/cairn/check-process-job-profiles`.

## Resolution rules

- Source precedence is deterministic: explicit sources override workspace sources, which override global sources.
- Duplicate profile names at the same precedence fail closed.
- Profiles must set exactly one of `command` or `program`; `args` are valid only with `program`.
- Secret-like environment keys such as `APP_TOKEN`, `APP_SECRET`, `APP_PASSWORD`, or `APP_KEY` fail before backend dispatch.
- Cwd and writable paths must stay under configured allowed prefixes and must not contain parent traversal.
- Unsupported backends and resource-limit violations fail before backend dispatch.

Safe profile identity metadata includes schema version, source label, policy hash, profile name, selected backend, and BLAKE3 profile/request hashes. Raw environment values, credentials, full path dumps, and backend-specific secret locators are excluded from identity envelopes and receipt metadata.
