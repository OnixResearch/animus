## Why

Animus models agent, tool, and model orchestration behind typed adapter boundaries and safe receipts. Its tool-result and evidence rails decide how a result is rendered, stored, and reused across a session.

The `mbrock/luv` workshop keeps field notes on four agent harnesses at `wiki/agent-harnesses.org`. The notes converge on six design points that any robust harness must treat as given: typed results with multiple renderings, harness-owned size, store-typed handles, data-first history, explainable silence, and supervised cycles. Luv's `AGENTS.md` adds the five-second silence rule: a command that stays silent is broken, and finding out how is the task.

This change adapts the convergence list into binding requirements for Animus tool-result and evidence rails. It does not add a new subsystem.

## What Changes

- Require typed tool results whose renderings are projections of one datum.
- Make size handling a harness responsibility with honest omission and stable handles.
- Require short, stable, store-typed handles.
- Require data-first history that renders without re-running tools.
- Require explainable silence: detach-with-handle bounds and a dump that names suspended work.
- Require supervised cycles with a prepare/commit split and a park-versus-finish distinction.
- Require bounded telemetry that rides with the result.
- Add the matching positive, negative, and boundary fixture coverage.

## Impact

- **Files**: runtime rails, tool-result and evidence types, shell supervision, fixtures, and this lifecycle package.
- **Testing**: focused runtime and rail tests, fixtures, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `mbrock/luv` `wiki/agent-harnesses.org` and `AGENTS.md` at `main`.

## Non-goals

- A new agent subsystem or a new harness protocol.
- Changes to model or provider semantics.
- Superseding the bounded tool-result cache change. Both changes must coexist.
- Claims that tools are correct, fresh, or safe because the harness recorded them.
