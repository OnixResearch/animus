# Proposal: Adapt agent status authority for Animus

## Why

Herdr separates two ideas that Animus will need as soon as multiple hosts, panes, headless clients, and background agents share one runtime: semantic lifecycle state and display-only presentation. Its useful pattern is not terminal screen scraping itself, but the authority model around `idle`, `working`, `blocked`, `done`, and `unknown`, plus explicit rules for when hook reports override heuristic fallback.

Animus needs the same status discipline behind typed runtime events so TUI shells, headless waits, notifications, receipts, and orchestration can agree on whether an agent needs attention without allowing two sources of truth to race.

## What Changes

- Define an Animus-owned semantic agent status DTO with authority source identity, agent label, optional native session reference, message, display metadata, sequence, and TTL fields.
- Define authority precedence so trusted lifecycle reports can suppress host heuristic fallback, while untrusted or stale reports fail closed to explicit unknown or fallback states.
- Keep custom labels, titles, and compact activity text presentation-only; they must not change waits, notifications, policy, or workspace rollups.
- Add explainable status-decision reports that identify the selected authority, fallback reason, stale-source handling, and redacted evidence without embedding raw prompts, terminal snapshots, or provider payloads.
- Keep process detection, screen matching, terminal reads, and host UI rendering in host adapters. Reusable runtime code consumes typed observations only.

## Impact

- **Files**: future runtime status DTOs, event payloads, host-observation adapter contracts, receipt/explain records, tests, and docs.
- **Testing**: positive lifecycle-report and fallback tests; negative stale sequence, expired TTL, conflicting authority, malformed source, visual-label escalation, and raw-evidence leak tests.
- **Non-claims**: this does not adopt Herdr's terminal screen detection manifests, implement a sidebar, add notification UI, or make screen scraping authoritative in reusable runtime crates.
