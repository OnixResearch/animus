# Ralph loop profile

Animus includes `ralph-loop` as a bounded Steel runtime policy profile inspired by Geoffrey Huntley's Ralph loop pattern, the `snarktank/ralph` script, Anthropic's Ralph Loop plugin, and the broader backpressure pattern of giving agents fast automated feedback.

## Runtime shape

- Policy profile: `ralph-loop`
- Script binding: `policy/steel-runtime/scripts/ralph-loop.scm`
- Completion promise: `COMPLETE`
- Policy turn ceiling: ten runtime turns

The script calls the model with compact Ralph instructions and the runtime input summary as task state. On later turns, it checks the previous model message. If the message contains `COMPLETE`, the script returns that message as the final response. Otherwise, it calls the model again with the original task state plus the previous iteration summary.

## Project-facing loop state

Animus does not use a root `specs/` directory or root `IMPLEMENTATION_PLAN.md` as its durable Ralph state. The native Cairn lifecycle owns that role:

- `AGENTS.md` is the compact operational guide loaded by agents.
- `cairn/specs/agent-harness-runtime/spec.md` is the accepted requirement surface.
- `cairn/changes/<slug>/proposal.md`, `design.md`, `specs/**/spec.md`, and `tasks.md` are the active planning and implementation state.
- `scripts/ralph-drain-cairns.scm` emits bounded Ralph prompts for draining active Cairn changes without granting ambient authority.

A Ralph-style iteration should pick one unfinished Cairn task, confirm the work is not already implemented, make focused changes, run the smallest relevant check, update the change package, and stop. If the plan is stale, regenerate or repair the Cairn change package rather than creating a parallel root plan.

## Backpressure rules

Backpressure is mandatory for looped agent work:

- Requirements and tasks should name observable success criteria and the checks that reject false completion.
- Programmatic checks come first: Cairn validation/gates, focused `scripts/cairn/check-*` rails, Cargo tests, Clippy, Nickel drift checks, fixture validators, and receipt-safety scans.
- Subjective or non-deterministic checks must be explicit host-owned gates with binary pass/fail results and safe receipts before they are treated as release evidence.
- A failing check is a task blocker. Do not mark a task done by weakening non-claims, deleting fixtures, or replacing focused validation with a broader uninspected command.

The host decides which capabilities exist. The reusable runtime only sees typed task state, policy, DTOs, and safe receipts.

## Host-owned effects

The profile does not copy Ralph's shell script or Claude plugin hook behavior. Hosts still own every effectful boundary: loading lifecycle files, creating fresh model sessions, checking branches, committing, running validation, updating `AGENTS.md`, browser checks, shell commands, credentials, provider sessions, network access, and persistence.

Use `ralph-loop` only when the host supplies explicit capabilities and safe task state. A missing completion promise does not grant more authority; the normal runtime max-turn policy stops the run.

## Verification

Run the focused rail after changing this profile or its Steel policy binding:

```sh
scripts/cairn/check-steel-runtime-policy --root .
```

Run the Cairn drain helper self-test after changing `scripts/ralph-drain-cairns.scm`:

```sh
steel scripts/ralph-drain-cairns.scm --self-test
```

If the host shell does not provide a `steel` executable, run the same file through Pi's Steel runner. The rail checks Nickel export drift, script hash binding, positive activation, Ralph prompt/reprompt/completion behavior, and negative fail-closed policy/runtime fixtures.
