# Proposal: Reuse context packets across subagent kinds

## Why

Worker teams already have a strong minimized context-packet contract, but recipe-backed subagents and fresh-context tasks still describe their context boundaries separately. Reusing one context-packet model across subagent kinds would make context minimization, leak validation, budget accounting, and UI display consistent.

This change promotes the worker-team context packet pattern into a shared subagent context boundary.

## What Changes

- Add a reusable subagent context packet type with selected parent excerpts, summaries, file observations, memory descriptors, bounded tool-result summaries, and admitted context references.
- Deny full transcripts, hidden memory, credentials, provider-native payloads, unbounded tool output, and unsafe raw material unless explicitly admitted and leak-validated.
- Add deterministic BLAKE3 context-packet identities and budget summaries for receipts and ledger projection.
- Adapt recipe-backed, fresh-context, side-question, and worker-team validators to consume or project the shared packet form.

## Impact

- **Files**: future shared context DTOs, validation helpers, conversion shims, tests, docs, and safe receipt fields.
- **Testing**: positive minimized context packets for every child-run kind; negative full-transcript, hidden-memory, credential, provider-payload, unbounded-output, unsafe-raw, and over-budget tests.
- **Non-claims**: this does not import raw parent transcripts into children or make context admission automatic.
