# Proposal: Project router request signals from typed metadata

## Why

`Signals::project` takes an *already-banded* `SignalDraft` — task_shape, risk, contract/context pressure, latency, cost, privacy, and fallback eligibility are all pre-classified labels. Nothing in the workspace computes those bands from a real request. Today every caller of the router would have to hand-classify risk and pressure, which is exactly the ad hoc glue the vLLM adaptation was meant to replace. Animus needs a deterministic default projector plus a pluggable classifier seam so a host can inject any signal source (including the local Supra-Router model) without the router depending on it.

## What Changes

- Add a pure rule-based `project_signals(metadata: &RequestMetadata) -> Result<SignalDraft, SignalProjectionError>` that derives each band from typed, safe request metadata: token/char counts, tool-call presence, code presence, structured-output pressure, context-window pressure, privacy policy, and a fallback-eligibility hint.
- Define a `SignalSource` adapter trait in `animus-adapters` (implementation-free) that hosts implement to supply a `SignalDraft` from richer classifiers; the rule projector is the default `SignalSource`.
- Require the projector to omit raw prompt bodies, provider payloads, credentials, and unbounded text — it consumes only bounded counts and labels.
- Add positive tests covering each band derivation and negative tests for malformed metadata, missing privacy policy, and unsafe-payload rejection.

## Impact

- **Files**: `crates/animus-router` request-projection module, `animus-adapters` `SignalSource` trait, request-metadata DTO if absent, router tests, facade inventory, and Cairn evidence.
- **Testing**: positive projection fixtures per band; negative tests for empty prompt, missing privacy policy, oversized-but-bounded inputs, and leak rejection.
- **Non-claims**: this does not call a classifier model, depend on a remote service, change recipe selection, or wire the runtime; the Supra-Router adapter is a later cairn.
