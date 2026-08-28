# Proposal: Add trust-labeled context admission

## Why

Animus already reduces request-local context through typed contributions. Document ingest also keeps extracted content untrusted.

The runtime does not yet require every indexed or external contribution to carry origin, trust tier, screening, quarantine, freshness, and policy-admission facts. This gap can let a host flatten untrusted content into ordinary prompt text.

## What Changes

- Extend request-context contribution metadata with bounded trust facts.
- Add a pure fail-closed context-admission decision before prompt assembly and provider dispatch.
- Treat classifier and screening results as advisory observations.
- Require an injected Basalt-compatible policy decision for protected external context.
- Preserve trust and quarantine facts through contribution reduction and model-request snapshots.
- Add metadata-only receipts and leak rejection.

## Impact

Hosts can route indexed files, email, messages, notes, and external records into Animus without erasing their origin or trust posture.

Animus retains agent-context admission. The resource catalog retains content and quarantine state. Basalt retains policy meaning. Hosts retain classifiers, retrieval, storage, and provider effects.

## Dependencies

The change can use fakes before a resource-catalog producer exists. Live adoption requires a published producer contract and the Basalt content-context policy revision.

## Non-goals

- Do not implement prompt-injection detection.
- Do not claim that a classifier proves content safe or malicious.
- Do not store indexed content or quarantine records in Animus.
- Do not let policy admission grant tool, filesystem, process, or network authority.
- Do not expose raw content in receipts.
- Do not add a Spacedrive source dependency.
