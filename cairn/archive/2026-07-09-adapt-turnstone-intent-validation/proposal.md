# Proposal: Adapt Turnstone intent validation contracts

## Why

Turnstone's intent-validation flow separates fast deterministic risk classification from optional LLM review of pending tool calls. Animus already has permission, confirmation, policy, and tool-admission boundaries, but it needs a first-class intent verdict shape so hosts can show evidence-backed risk before approval without letting a model judge create authority.

The model proposes and the gate disposes: verdicts are evidence for admission and operator review, not executor grants. Deterministic deny/critical findings and host policy remain binding.

## What Changes

- Add `IntentVerdict`-style DTOs for tool-call identity, summary, risk level, confidence, recommendation, tier, evidence references, latency bucket, and safe model labels.
- Add deterministic heuristic rule descriptors for common tool classes, command patterns, mutation classes, network access, secret-adjacent reads, and persistent control-plane changes.
- Define optional reviewer semantics where model/classifier verdicts may veto, require review, or add annotations, but cannot auto-grant missing authority or lower deterministic deny findings.
- Persist verdict metadata through safe receipts and runtime events using hashes, labels, counts, and redaction markers only.
- Tie verdicts to tool batch admission so cross-call interactions can require review before any side effect.

## Impact

- **Files**: adapter DTOs, runtime admission integration, heuristic fixtures, optional reviewer envelope, docs, and focused tests.
- **Testing**: positive low-risk read-only and review-required mutation cases plus negative critical command, secret-adjacent access, stale verdict, classifier downgrade, missing evidence, malformed confidence, and receipt-leak cases.
- **Non-claims**: this does not add smart approvals, live LLM judge execution, product approval UI, policy bypass, or raw command/tool bodies in durable evidence.