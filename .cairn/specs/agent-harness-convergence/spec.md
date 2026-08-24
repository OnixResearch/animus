# Agent Harness Convergence Specification

## Purpose

Defines the `agent-harness-convergence` capability.

## Requirements

### Requirement: Tool results are typed values with multiple renderings

r[animus.convergence.typed_results] The runtime MUST treat a tool result as a typed value. The model-facing rendering, the human-facing view, and the persisted record MUST be projections of that one value. The runtime MUST NOT canonicalize ad-hoc strings as the result datum.

#### Scenario: One datum, several renderings

r[animus.convergence.typed_results.projections]
- GIVEN a tool returns a typed result
- WHEN a model-facing text and a human-facing view are produced
- THEN both MUST derive from the same typed datum

#### Scenario: Ad-hoc string datum fails

r[animus.convergence.typed_results.string]
- GIVEN an implementation records a rendered string as the canonical result
- WHEN the record is inspected
- THEN the inspection MUST report the string-only record as a defect

### Requirement: Size is the harness's problem

r[animus.convergence.size] The harness MUST own truncation and paging. An oversized result MUST be folded or paged with an exact statement of what was omitted and a stable handle to retrieve more. Truncation MUST NOT be silent.

#### Scenario: Oversized result is folded with a handle

r[animus.convergence.size.fold]
- GIVEN a tool result exceeds the configured bound
- WHEN the result enters the session
- THEN the harness MUST state what it omitted
- AND MUST return a stable handle for the omitted part

#### Scenario: Silent truncation fails

r[animus.convergence.size.silent]
- GIVEN a result is truncated with no omission statement
- WHEN the result is inspected
- THEN the inspection MUST flag the silent truncation

### Requirement: Handles are short, stable, and store-typed

r[animus.convergence.handles] References to past results, sessions, and documents MUST be short, stable, and typed by the store. The runtime MUST resolve a handle by asking the store, not by parsing a link table or by guessing syntax.

#### Scenario: Handle resolves through the store

r[animus.convergence.handles.resolve]
- GIVEN a model supplies a handle
- WHEN the runtime resolves it
- THEN the runtime MUST resolve it through the store
- AND an unknown handle MUST fail closed with the exact handle named

### Requirement: History is data first

r[animus.convergence.history] Every tool call, result, and span MUST be recorded as structured append-only data. Any later rendering MUST be a query over that data and MUST NOT require re-running the tool.

#### Scenario: Re-render without rerun

r[animus.convergence.history.render]
- GIVEN a past tool call exists in the record
- WHEN a view is rendered later
- THEN the view MUST be derived from the recorded data
- AND the tool MUST NOT run again

### Requirement: Silence is a defect the runtime can explain

r[animus.convergence.silence] A tool call that exceeds a time bound or an output byte bound without returning MUST detach with a handle within the bound. The runtime MUST expose a dump that names suspended work and how long each item has been suspended.

#### Scenario: Detach within the bound

r[animus.convergence.silence.detach]
- GIVEN a tool call exceeds the time bound or the byte bound
- WHEN the runtime enforces the bound
- THEN the call MUST return a handle and the follow command
- AND MUST NOT leave the caller waiting silently

#### Scenario: Dump names suspended work

r[animus.convergence.silence.dump]
- GIVEN the runtime is asked why it waits
- WHEN the dump runs
- THEN the dump MUST name each suspended item and its suspension duration

### Requirement: A cycle is a supervised scope

r[animus.convergence.cycle] A tool-use cycle MUST be a supervised scope. The runtime MUST split each tool call into prepare and commit. A batch MUST run as real parallel work. The runtime MUST distinguish parking from finishing and MUST hold user side effects outside the synchronization boundary.

#### Scenario: Prepare and commit are separate

r[animus.convergence.cycle.split]
- GIVEN a batch of tool calls runs
- WHEN each call executes
- THEN each call MUST prepare an executable plan
- AND MUST commit its side effects after the guarded work completes

#### Scenario: Park does not finish

r[animus.convergence.cycle.park]
- GIVEN a cycle needs a human decision
- WHEN the decision is not present
- THEN the runtime MUST park the cycle
- AND MUST NOT end the cycle

#### Scenario: Side effect inside the guard fails

r[animus.convergence.cycle.guard]
- GIVEN user destructors or side effects run inside the synchronization boundary
- WHEN the guard is inspected
- THEN the inspection MUST flag the boundary violation

### Requirement: Telemetry rides with the result

r[animus.convergence.telemetry] Bounded resource telemetry for each tool call, when available, MUST ride along with the result and MUST NOT claim product correctness.

#### Scenario: Telemetry is bounded and visible

r[animus.convergence.telemetry.visible]
- GIVEN a tool call runs under the supervisor
- WHEN telemetry is collected
- THEN the bound MUST be named
- AND the telemetry MUST be visible beside the result

### Requirement: Fixture coverage

r[animus.convergence.verification] Positive, negative, and boundary fixtures MUST cover typed renderings, harness-owned size, store-typed handles, data-first history, silence detach and dump, supervised cycles, and telemetry.

#### Scenario: Focused convergence matrix passes

r[animus.convergence.verification.matrix]
- GIVEN the convergence rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming inputs MUST behave as declared
- AND every silent-truncation, guessed-handle, rerun-render, silent-wait, unguarded-effect, or overclaim input MUST fail as declared
