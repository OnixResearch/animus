# OpenAI Codex diff review mode boundary

Animus adapts dedicated Codex-style review as a read-only, diff-scoped runtime profile. The reusable adapter contract is data-only: hosts provide a validated change-set selector, source labels, model/tool services, and prior review state; reusable logic validates requests, policy decisions, findings, evidence bounds, and rerun state without reading files, invoking version control, mutating workspaces, opening pull requests, or escalating approvals.

A diff-review profile is not itself a capability-bearing semantic reviewer. Hosts that need attributed role evidence must separately activate an `animus_runtime::agent_roles` reviewer assignment, satisfy its exact capability intersection and separation policy, bind the immutable subject, and authorize review output. The role remains unable to import raw diffs into durable output, mutate the subject, approve itself, grant authority, or establish completion.

## Request shape

A review request uses a version-control-neutral selector:

- working-copy changes;
- one commit or revision;
- a base/head branch or revision comparison;
- an externally supplied patch.

The active host adapter declares the selector kinds it supports. `DiffReviewRequest::validate` fails before model or tool dispatch when the selector is unsupported, the required revision or patch label is absent, or no bounded target label is supplied.

## Read-only policy

`DiffReviewPolicyProfile::read_only` admits diff inspection, read-only file access, and model review calls. It denies file edits, write-capable tools, approval escalation, commit creation, and pull-request management. Denials are typed so a host can record why review mode rejected a requested action without performing it.

## Findings and evidence

Review findings are structured with identity, severity, category, affected span, bounded evidence, rationale, confidence, and remediation guidance. Evidence may reference a file span, patch hunk, or external input snippet. Full files, unbounded logs, unrelated context, raw prompts, provider payloads, credentials, and private paths are rejected by validation. Bounded snippets retain a BLAKE3 identity for deterministic deduplication and evidence checks.

## Rerun state

`compare_diff_review_rerun` compares prior and current reports. Prior findings that still appear remain open, prior findings absent from the rerun are fixed unless the host marks them dismissed, and current-only findings are newly introduced.

## Non-goals

Review mode does not apply fixes, create commits, manage pull requests, depend on Git-only semantics, or claim live-provider review quality. Fixes belong in a separate coding run with an explicit write-capable policy.
