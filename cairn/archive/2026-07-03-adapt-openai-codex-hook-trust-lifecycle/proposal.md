# Proposal: Adapt OpenAI Codex hook trust lifecycle for Animus

## Why

OpenAI Codex exposes lifecycle hooks around prompt submission, tool use, permission requests, compaction, subagent starts/stops, and turn stops. The useful pattern is not the exact command-hook implementation, but the explicit lifecycle surface: hooks receive typed event inputs, may return narrowly scoped outputs, and must be reviewed or managed before they can affect an agent run.

Animus needs a similar lifecycle contract so embedders can add policy checks, validation, audit context, and continuation decisions without hard-coding those behaviors into the runtime core.

## What Changes

- Define an Animus-owned lifecycle hook registry for session, prompt, tool, permission, compaction, subagent, and stop events.
- Add hook identity and trust records using content-addressed definitions and explicit managed/user/project trust scopes.
- Define typed hook inputs and outputs with narrowly scoped effects such as additional context, deny decisions, continuation requests, and system-visible warnings.
- Bind hooks to runtime events and policy decisions without allowing hooks to bypass adapter contracts or sandbox policy.
- Record hook outcomes in runtime reports with redacted inputs, hook identity, trust status, and failure class.

## Impact

- **Files**: hook DTOs, trust records, event wiring, Nickel hook config contracts, validation tests, runtime event/report docs, and shell-owned hook executor interfaces.
- **Testing**: positive tests for trusted hooks adding context or denying supported operations; negative tests for untrusted hooks, malformed output, unsupported effect fields, failing hooks, and attempted policy bypass.
- **Non-claims**: this change does not add a concrete subprocess hook runner, Codex-compatible `hooks.json`, plugin installation, or UI hook browser.
