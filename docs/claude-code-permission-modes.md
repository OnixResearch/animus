# Claude Code permission mode adaptation

Animus treats Claude Code-style permission modes as bounded local UX caveats around guarded actions. Permission policies can say `allow`, `ask`, or `deny`, but local `allow` never expands authority beyond Nickel contract checks, verified UCAN grants, sandbox state, and host-owned capability gates.

The pure adapter-boundary core lives in `animus-adapters`:

- `PermissionPolicy::validate` turns host-loaded descriptors into typed allow/ask/deny policies with source identity, trust scope, trust status, precedence, and redacted prompt-summary policy.
- `resolve_permission` combines a guarded action, the prior authority result, selected local policy, and optional prompt token into `Execute`, `PromptRequired`, or `Denied`.
- `PermissionPromptRequest` and `PermissionPromptDecision` scope ask-mode continuation tokens by action, resource label, session label, and policy hash.
- `PermissionReceipt` and `validate_permission_receipt_text` keep evidence to action classes, redacted resource hashes, authority/local/prompt status, policy hash, reasons, and redaction markers.

If authority is denied, the permission resolver denies before dispatch even when local policy says `allow`. If local policy says `ask`, the resolver returns `PromptRequired` without model, tool, shell, filesystem, network, or Steel dispatch until a matching host prompt token is supplied. Stale or mismatched prompt tokens fail closed.

Permission receipts must not include raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs.
