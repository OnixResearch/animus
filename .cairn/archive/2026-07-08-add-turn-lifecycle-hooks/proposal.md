# Proposal: Add turn lifecycle hook boundaries

## Why

Animus already has session, model, tool, mutation, validation, workflow, and stop hook points, but embedders also need stable turn-level seams. `stop` is terminal-run scoped, and `workflow_completion` is workflow scoped; neither expresses work that should happen before each loop-fabric turn or immediately after a turn result is recorded.

Turn hooks let hosts schedule bounded memory updates, checkpoints, validation follow-ups, notifications, or continuation decisions without giving hook adapters direct runtime authority.

## What Changes

- Add stable `pre_turn` and `post_turn` lifecycle hook points.
- Treat startup as the existing `session_start` lifecycle boundary instead of adding a duplicate startup label.
- Model turn lifecycle transitions in a pure runtime core that emits typed lifecycle hook effects.
- Keep hook execution host-owned and adapter-backed; reusable runtime code only validates descriptors, safe inputs, outputs, capabilities, and no-bypass policy binding.
- Add positive and negative tests for startup/pre-turn/post-turn hook vocabulary, allowed effects, rejected post-turn denials, safe payloads, and pure FSM sequencing.

## Impact

- **Files**: runtime lifecycle hook DTOs, pure turn lifecycle core, tests, docs, and accepted runtime specs.
- **Testing**: focused hook and turn-lifecycle tests, plus Cairn validation/gates.
- **Non-claims**: this change does not add shell hook files, Steel hook execution, background daemons, persistence, network exporters, direct tool execution, or a concrete host hook runner.
