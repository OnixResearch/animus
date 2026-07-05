# OpenAI Codex exec-policy rule adaptation

Animus adapts Codex-style command-prefix policy as an Animus-owned contract. The policy is not a Codex `.rules` parser and does not add process execution. It defines how a host-owned process adapter can ask the reusable runtime whether a command may cross the active sandbox boundary.

## Runtime contract

- Command intent is validated as argv-like typed data before evaluation.
- Sandbox escalation context records the active sandbox, requested capability, and safe escalation reason.
- Rules carry a stable identity, a decision (`allow`, `prompt`, or `forbid`), a safe rationale, a command prefix, positive examples, and negative examples.
- Rule examples are validation fixtures: positive examples must match the prefix, and negative examples must not match it.
- Evaluation is pure and deterministic. If multiple rules match, the most restrictive decision wins: `forbid` over `prompt` over `allow`.
- If no rule matches, evaluation fails closed with `forbid`.

## Shell-wrapper handling

Animus splits only narrow `sh -c` and `bash -lc` wrappers whose source is plain command words separated by `&&`, `||`, or `;` tokens. Each command in a simple chain is evaluated separately so a forbidden command cannot hide behind an allowed prefix.

Unsupported syntax, including pipes, redirects, variables, globs, quoting, subshells, and background execution, is left opaque. Opaque wrappers are evaluated as the wrapper command itself and therefore fail closed unless policy explicitly admits that wrapper.

## Evidence and telemetry

Policy evaluation emits matched rule identities, decision labels, sandbox labels, escalation reasons, and BLAKE3 command fingerprints. Runtime `SandboxEscalation` events intentionally omit raw command bodies and command output. The redaction marker is `command_hash_only`. Preserves envelopes round-trip these events with hash-only command evidence and reject malformed fingerprints or unknown redaction markers.

## Authored policy fixtures

Typed Nickel fixtures live in `policy/exec-policy/policy.ncl`. The generated runtime fixture is `policy/exec-policy/generated/exec-policy.json`.

Run the focused rail:

```sh
scripts/cairn/check-exec-policy --root .
```

The rail checks Nickel export drift, positive and negative policy validation, shell smuggling behavior, runtime sandbox escalation telemetry, and Preserves event-envelope redaction.
