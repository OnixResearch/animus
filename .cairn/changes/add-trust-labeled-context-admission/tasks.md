# Tasks: Add trust-labeled context admission

## Pure trust model

- [ ] [serial] Add nominal origin, trust-tier, screening, quarantine, freshness, context-use, and fencing-decision types. r[animus.context_fencing.facts]
- [ ] [serial] Extend protected contribution validation without weakening existing source-class, budget, conflict, lifetime, or payload checks. r[animus.context_fencing.admission]
- [ ] [serial] Add pure admitted, omitted, quarantined, review-required, and denied outcomes. r[animus.context_fencing.admission]
- [ ] [serial] Keep classifier results advisory and distinct from authority or safety claims. r[animus.context_fencing.screening_boundary]

## Policy and runtime composition

- [ ] [serial] Add a Basalt-compatible content-context decision adapter with exact binding checks. r[animus.context_fencing.policy]
- [ ] [serial] Block protected context when policy is missing, denied, stale, mismatched, or unavailable. r[animus.context_fencing.policy]
- [ ] [serial] Preserve fencing metadata through reduction, final payload validation, and model-request snapshots. r[animus.context_fencing.projection]
- [ ] [serial] Keep catalog, classifier, quarantine, policy-loading, UCAN, storage, and provider effects behind injected host services. r[animus.context_fencing.host_boundary]

## Receipts and verification

- [ ] [serial] Add metadata-only fencing receipts and raw-content leak rejection. r[animus.context_fencing.receipts]
- [ ] [parallel] Add positive tests for authored, collaborative, policy-admitted external, and review-only context. r[animus.context_fencing.verification]
- [ ] [parallel] Add negative tests for missing facts, unknown tiers, quarantine, flags, stale observations, unavailable classifiers, policy mismatch, denial, and payload drift. r[animus.context_fencing.verification]
- [ ] [parallel] Add receipt leak tests for prompts, documents, messages, email, classifier bodies, credentials, headers, and host paths. r[animus.context_fencing.verification]
- [ ] [serial] Document owner boundaries, policy composition, screening non-claims, and the Spacedrive design-reference provenance. r[animus.context_fencing.evidence]
- [ ] [serial] Run focused Rust, Clippy, Octet, request-context, request-lifecycle, evidence-safety, Cairn validation, and Cairn gates. r[animus.context_fencing.evidence]
