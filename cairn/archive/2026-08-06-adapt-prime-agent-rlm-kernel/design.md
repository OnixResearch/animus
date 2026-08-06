## Context

Prime Agent (PrimeIntellect-ai/prime-agent, MIT) builds its agent on two ideas. First, the RLM pattern: the model works inside a persistent IPython kernel where context is held in variables, and tools plus recursive subagents are callable functions. Second, typed host requests: kernel code that needs a real effect returns a typed operation to the TypeScript session, which executes it and returns the result. Workers and kernels are separate processes for lifecycle containment, explicitly not security sandboxes.

Animus already has the pieces this pattern needs: `animus-steel` as an embeddable Scheme fabric, adapter boundaries where Rust revalidates every selected action, subagent contracts with lineage, and compaction contracts under `pi-compaction-branch-summaries` and `zerostack-midturn-compaction`. What it lacks is the model-facing direction: today Steel scripts are host policy, and the model cannot hold state, define helper functions, or call subagents programmatically across turns.

Completion requires a bounded contract family that makes Steel model-facing without granting ambient authority, plus the compaction-survival invariant as an explicit contract rather than an emergent property. A kernel that executes model code directly against the host, a second script engine, or receipts containing variable contents is false completion.

## Decisions

### Decision: Steel is the model-facing kernel language

**Choice:** The RLM kernel is a non-default profile over `animus-steel`. The model receives a persistent Steel environment. Context binds to named variables. Helper definitions persist across turns. No Python or second interpreter enters reusable surfaces.

**Rationale:** `animus-steel` already embeds Scheme and revalidates actions at adapter boundaries. Reusing it keeps one fabric, one policy surface, and one receipt family. Steel is smaller than IPython and has no ambient OS authority of its own.

### Decision: typed host requests are the only effect channel

**Choice:** Kernel code performs effects only by returning typed host requests: read, edit, shell, tool call, subagent spawn, context read. The runtime admission layer revalidates every request against the admitted policy before the host shell executes it. Kernel evaluation itself is pure with respect to the outside world.

**Rationale:** This is the Animus functional-core/imperative-shell rule applied to model code. It is also exactly the prime-agent boundary that makes RLM auditable. Direct effects inside the interpreter would defeat every existing adapter conformance rail.

### Decision: child spawn is bounded and typed

**Choice:** `rlm(...)` is a host request that names a child profile, an explicit context slice, and result expectations. Policy bounds depth, fan-out, token and wall-clock budgets, and result size. Results return as typed values with lineage into the existing subagent contracts.

**Rationale:** Recursive agents are the highest-risk surface in RLM. Bounded spawn keeps it inside the existing subagent facade and receipt family instead of creating an ambient agent factory.

### Decision: kernel state survives compaction by contract

**Choice:** Compaction summarizes prompt context but never clears kernel variables, imports, or definitions. The compaction planner records which context the kernel still holds so summaries can reference rather than repeat it. Clearing the kernel is an explicit host action with its own receipt.

**Rationale:** Prime Agent's persistent kernel is the main reason long tasks stay coherent across compaction. Making survival a contract prevents a future compaction change from silently destroying task state.

### Decision: evidence stays metadata-only

**Choice:** Receipts bind policy identity, operation classes, bounded counts, and BLAKE3 identities of admitted artifacts. They never contain variable values, model-written code bodies, or child transcripts.

**Rationale:** Kernel state holds working context, which can contain secrets or private content. Existing safe-receipt hygiene rules apply unchanged.

## Alternatives Considered

### Expose the host-owned Steel loop directly to the model

Rejected because host policy scripts carry different trust assumptions than model-written code. The model-facing profile has its own admission policy and capability set.

### Adopt IPython or a Python kernel for parity with prime-agent

Rejected because it adds a Python process, package, and environment authority surface. Steel gives the same persistent-environment semantics without a second language stack.

### Let the kernel read and write files through Steel's own I/O

Rejected because it bypasses adapter admission and every existing authority boundary. All effects remain typed host requests.

## Risks / Trade-offs

- A persistent model-facing environment raises the value of correct admission; a single admitted operation class is a direct effect path. Mitigation: fail-closed admission, named bounds, negative fixtures for every operation class.
- Long-lived kernel state can accumulate stale or poisoned variables. Mitigation: explicit clear operations, bounded environment size, receipts for state transitions.
- Steel evaluation of arbitrary model code must stay bounded. Mitigation: fuel, depth, and allocation limits in the admitted profile; timeouts remain host-owned.

## Non-Claims

This design does not establish security sandboxing, model output correctness, arbitrary recursion safety, cross-session state sharing, Python or IPython compatibility, task completion, production readiness, or release eligibility.
