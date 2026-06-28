# Proposal: Adapt Hermes retrieval budgets for Animus

## Why

Hermes skills encode a practical retrieval budget pattern: make one targeted retrieval first, retrieve again only for missing required facts or contradictions, stop when evidence is sufficient, and name missing evidence when blocked. Animus should adapt this into typed resource/policy metadata so retrieval-capable tools, skills, and model prompts can avoid unbounded browsing while preserving evidence quality.

## What Changes

- Define retrieval budget descriptors for first action, repeat conditions, stop conditions, missing-evidence behavior, citation/evidence expectations, and progress guidance.
- Attach retrieval budgets to retrieval-capable skills, tools, and policy profiles.
- Add deterministic prompt-hygiene checks that reject retrieval resources without a budget when policy requires one.
- Record bounded retrieval decision metadata in reports/receipts without storing raw fetched content by default.
- Add fixtures based on Hermes web-fetch, browser, Kagi, Context7, multi-lens, and property-discovery skill patterns.

## Impact

- **Files**: future retrieval budget DTOs, prompt assembly integration, receipt/report metadata, checker fixtures, and docs.
- **Testing**: valid budget acceptance plus missing first action, missing stop rule, unbounded repeat rule, missing evidence behavior, and unsafe raw-content receipt rejection.
- **Non-claims**: this change does not implement web search, browser automation, or a specific retrieval provider.
