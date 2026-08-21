## Context

Animus owns agent and tool orchestration behind typed adapter boundaries and safe receipts. Tool results flow from adapters into the session, are rendered for the model, and are recorded as evidence.

The `mbrock/luv` wiki page `agent-harnesses.org` reads four real harnesses (froth, sheaf, nxt, swash) and converges on six design points. The page is a reading of those repositories, not a plan. Animus already implements pieces of several points in its adapter and receipt rails. This change binds the rest and forbids the recurring violations.

## Goals

- One typed result datum with several renderings.
- Harness-owned size with honest omission.
- Store-typed short handles.
- Data-first history with rerun-free rendering.
- Explainable silence with detach bounds.
- Supervised cycles with real parallelism.
- Bounded telemetry beside results.

## Non-goals

- A packaged harness protocol or new transport.
- Provider, model, or role semantics.
- Product correctness claims from recorded history.
- Superseding the bounded tool-result cache change.

## Decisions

### Decision: One datum, many renderings

**Choice:** The runtime treats a tool result as a typed value. Model-facing text, human-facing views, and persisted records are projections of that value. Ad-hoc strings are not the canonical datum.

**Rationale:** The convergence list is explicit: the model-facing text, the human-facing view, and the persisted record are projections of one datum. Conflated renderings force every consumer to re-derive meaning.

### Decision: Size is the harness's problem

**Choice:** The harness owns folding, paging, and truncation. Omission is stated exactly and a stable handle returns more.

**Rationale:** When the harness handles size honestly, the model stops defending itself with pagers and truncation pipelines, and the prompt gets simpler and cheaper.

### Decision: Handles are store-typed

**Choice:** References are short, stable, and typed by the store. Resolution asks the store. No link table and no syntax guessing.

**Rationale:** A model can cite by typing a handle only when resolution is unambiguous.

### Decision: History is data first

**Choice:** Every call, result, and span is structured append-only data. Rendering is a query. It never re-runs tools.

**Rationale:** The sources agree that history is data first, and every rendering is a query over it. Re-rendering must not rerun anything.

### Decision: Silence is explainable

**Choice:** A call that exceeds the time bound or the byte bound detaches with a handle and a follow command. The runtime exposes a dump that names suspended work and suspension duration.

**Rationale:** A hung agent is a question you can ask the process. Silence is never endured.

### Decision: Cycles are supervised

**Choice:** A cycle is a supervised scope. Tool calls split into prepare and commit. Batches run as real parallel work. Parking is distinct from finishing. User side effects stay out of the synchronization boundary.

**Rationale:** The relevant seams are prepare-versus-commit and park-versus-finish. They make parallel, interruptible operation possible without false completion.

### Decision: Telemetry rides with results

**Choice:** Bounded per-call resource telemetry rides with the result when present, and claims nothing about correctness.

**Rationale:** Per-call telemetry makes resource behavior visible without importing product meaning.

## Dependency Direction

```text
runtime shell
      ^
      |
adapter boundaries -- tool-result and evidence rails
```

No new project dependencies.

## Decision Flow

```text
adapter returns a typed result
  -> runtime projects the model, human, and persisted views
  -> harness folds or pages by policy with a handle
  -> history records the call, result, and spans as data
  -> supervisor prepares, runs, and commits each call
  -> runtime explains silence on request
```

## Adversarial Audit

The audit must try to show that Animus:

- renders or persists an ad-hoc string as the canonical result;
- truncates without stating what was omitted;
- resolves a handle by syntax or link table;
- re-runs a tool to render past history;
- waits silently past a bound without a handle;
- runs user side effects inside the synchronization boundary;
- presents a parked cycle as finished;
- claims tool correctness from recorded history.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Typed results add contract surface at adapter boundaries. The audit requires it.
- Harness-owned size moves policy into the runtime. That is the intended authority.
- Supervised cycles raise orchestration complexity. They remove false completion paths.
- The convergence list is source evidence, not proof of fitness for Animus's exact adapters. Fixtures must demonstrate each point on Animus's own surfaces.

## Non-Claims

This change does not prove tool correctness, context freshness, provider behavior, model quality, or product correctness. Recorded history is evidence of what happened, not proof that it was right.
