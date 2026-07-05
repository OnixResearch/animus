# Proposal: Adapt Claude Code lifecycle hooks for Animus

## Why

Claude Code hooks show a useful customization point: hosts often need to run checks or emit notifications before and after actions such as tool calls, file edits, validation, or session lifecycle events. Animus should adapt hook points as typed event contracts, not hidden shell snippets, so embedders can extend runtime behavior without weakening adapter boundaries.

This change defines safe lifecycle hook semantics with explicit capabilities, bounded inputs, deterministic ordering, and fail-closed behavior.

## What Changes

- Define hook point identifiers for session, prompt assembly, model request, tool dispatch, mutation, validation, receipt, and workflow completion events.
- Define safe hook event DTOs that expose bounded metadata rather than raw prompts, credentials, provider payloads, full paths, or unbounded outputs.
- Require hook implementations to be host-owned adapters with declared capabilities and policy-gated execution.
- Specify hook outcomes such as observe, annotate, deny, or request revalidation without letting hooks bypass core checks.
- Add positive and negative tests for valid hook observation, denied pre-action hooks, malformed hook outputs, unavailable hook services, timeout/cancellation classes, and leak attempts.

## Impact

- **Files**: future hook DTOs, hook registry core, policy contracts, host adapter boundaries, receipt metadata, tests, and docs.
- **Testing**: valid pre/post hook flows plus negative tests for hidden shell execution, unsafe event payloads, malformed hook output, denied hook capabilities, unavailable services, and action-bypass attempts.
- **Non-claims**: this change does not adopt Claude Code's hook file format, does not execute arbitrary shell commands from reusable runtime crates, and does not allow hooks to mutate state without revalidation.
