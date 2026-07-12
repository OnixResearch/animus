## Why

Animus already rejects malformed tool-call/result ordering in session replay and compaction, but it does not define how a host should recover an active branch after cancellation, process loss, or a crash leaves an assistant tool call without a matching tool result. A later provider request can reject that transcript, while blindly rerunning the tool could repeat mutations.

Tau demonstrates a useful product invariant: before another provider request, every model-visible assistant tool call has a terminal result, and recovery is made durable. Animus should adapt that behavior as a stricter pure repair plan with typed interruption reasons, append-only history, no tool re-execution, and safe receipts rather than copying Tau's Python implementation.

## What Changes

- Define an active-branch transcript invariant and a pure planner that identifies exactly the unmatched tool calls requiring repair. r[agent_harness_runtime.tau_tool_repair.invariant] r[agent_harness_runtime.tau_tool_repair.plan]
- Classify explicit user cancellation, host interruption, unclean shutdown, and unknown missing-result cases without inferring user intent from absence alone. r[agent_harness_runtime.tau_tool_repair.reason]
- Apply repairs through append-only branch or replay-mutation records while preserving prior history and matched tool results. r[agent_harness_runtime.tau_tool_repair.append_only]
- Require a successful, durable repair before another provider request and prohibit repair from invoking a tool executor. r[agent_harness_runtime.tau_tool_repair.provider_gate]
- Emit metadata-only BLAKE3-bound repair receipts and add positive and negative recovery fixtures. r[agent_harness_runtime.tau_tool_repair.receipts] r[agent_harness_runtime.tau_tool_repair.verification]

## Impact

- **Runtime core**: gains deterministic repair DTOs, validation, planning, and receipt projection over already-loaded active-branch data.
- **Host shells**: persist admitted repair actions and move the active leaf before provider continuation; they do not rediscover sessions or tools.
- **Sessions**: existing history remains immutable, matched results are preserved, and repeated recovery is idempotent.
- **Claims**: a repair proves only that reconstructed provider context contains terminal synthetic failures for identified unmatched calls. It does not prove why the process stopped, that the tool never produced an external effect, or that arbitrary provider transcript formats will accept the result.
