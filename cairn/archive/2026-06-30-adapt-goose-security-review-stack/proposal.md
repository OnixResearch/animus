# Proposal: Adapt Goose security review stack for Animus

## Why

Goose documents a layered security model around permission modes, tool permissions, prompt-injection detection, and an adversary reviewer that evaluates high-risk tool calls before execution. Animus already has stronger authority-first policy goals, but it needs a concrete security review stack that combines deterministic checks, optional classifier results, and pre-tool review receipts without treating model judgment as authority.

This change adapts Goose's layered review pattern while keeping all allow decisions subordinate to Animus capability, sandbox, confirmation, UCAN-style authority, and redaction policy.

## What Changes

- Define security scan inputs for requested tool calls, recent bounded context summaries, original task metadata, and active policy labels.
- Add deterministic prompt-injection and dangerous-action pattern findings with confidence, finding IDs, and safe explanations.
- Add optional classifier/adversary reviewer outcomes as policy inputs, never as authority grants.
- Define fail behavior per risk class so high-risk review failures can fail closed while low-risk classifier outages can degrade safely.
- Integrate review outcomes with permission resolution, confirmation prompts, sandbox decisions, and safe receipts.
- Add negative tests for exfiltration, destructive commands, privilege escalation, hidden-instruction payloads, model-review override attempts, and fail-open overuse.

## Impact

- **Files**: future security finding DTOs, deterministic scanner core, optional classifier/reviewer contracts, policy-binding helpers, receipt DTOs, docs, tests, and host adapter hooks.
- **Testing**: positive benign tool-call review tests plus negative prompt-injection, exfiltration, destructive-action, reviewer-failure, and authority-bypass tests.
- **Non-claims**: this change does not copy Goose's default autonomous mode, does not make LLM risk labels authoritative, does not send private content to external classifiers by default, and does not let an adversary reviewer weaken active policy.
