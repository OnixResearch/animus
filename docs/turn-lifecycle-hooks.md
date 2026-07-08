# Turn lifecycle hooks

Animus models turn-level extension points in a pure runtime core and keeps hook execution in host-owned adapters.

Startup uses the existing `session_start` lifecycle hook point. Animus does not define a duplicate `startup` hook label.

`pre_turn` is emitted before loop-fabric evaluation. It may observe, add bounded context, warn, or deny before the turn starts.

`post_turn` is emitted after a turn result is recorded. It may observe, warn, or request continuation, but cannot deny completed work, request revalidation of already-used payloads, execute tools, rewrite denied actions, weaken sandbox policy, or suppress approval.

The reusable runtime exposes `advance_turn_hooks` as the pure FSM boundary. It returns typed lifecycle hook effects and performs no process, filesystem, network, clock, provider, persistence, Steel, or hook-adapter execution.
