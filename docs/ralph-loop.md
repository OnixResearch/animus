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
- `scripts/ralph-drain-cairns.scm` emits bounded prompts for one Animus lifecycle; its explicit `--run` mode is an operator-owned process effect.
- `../ralph-drain-stack.scm` is the workspace-local temporary Pi-hosted supervisor for selected sibling native Cairn repositories; it is not packaged by Animus, plan mode is effect-free, and run mode is explicit.

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

## Temporary Pi-hosted stack drain

The stack helper uses a new Pi print-mode process for each bounded round. It never passes continue, resume, fork, session, name, alternate-mode, or argument-terminator overrides through operator options. A named one-hour host timeout sends `TERM` and applies a bounded kill grace so a stalled provider call becomes a local repository blocker. Each prompt tells Pi to load local instructions and Cairn drain guidance, inspect repository-local dependency readiness, preserve existing work, and drain at most one change without touching siblings, pushing, or managing a pull request.

Run Pi from the parent OnixResearch workspace root and plan all direct child repositories with native Cairn lifecycle roots:

```text
file: ralph-drain-stack.scm
args: []
```

Run one round in an explicitly ordered repository:

```text
file: ralph-drain-stack.scm
args: ["--repo", "cairn", "--run", "--limit", "1"]
```

The supervisor compares active change names, unchecked task counts, and archive directory names before and after Pi exits. One matching removal/archive is archived progress; one decreased task count is task progress; unchanged lifecycle state blocks that repository for the current run. Added changes, multiple removals, task changes in multiple packages, missing archive evidence, historical archive removal, or failed post-archive Cairn validation stop the stack run.

Pi exit status and model text never establish completion. `COMPLETE` is reported only when every selected native Cairn lifecycle is empty. Repository ordering is advisory until Cairn's receipt-backed cross-repository dependency contract is implemented, so this helper does not establish dependency satisfaction, release eligibility, or stack correctness. It is a temporary operator tool, not the embedded Animus production host.

## Verification

Run the focused rail after changing this profile or its Steel policy binding:

```sh
scripts/cairn/check-steel-runtime-policy --root .
```

Run both drain helper self-tests through Pi's Steel tool after changing their respective surfaces:

```text
file: scripts/ralph-drain-cairns.scm
args: ["--self-test"]

file: ../ralph-drain-stack.scm
args: ["--self-test"]
```

Also inspect stack discovery without `--run`:

```text
file: ../ralph-drain-stack.scm
args: ["--workspace-root", "..", "--limit", "1"]
```

A host-provided `steel` wrapper may run the same files when it supplies Pi's argument prelude; the current Animus development shell does not package a standalone Steel executable. Run the stack helper from the workspace root when relying on its default `.` discovery root. The policy rail checks Nickel export drift, script hash binding, positive activation, Ralph prompt/reprompt/completion behavior, and negative fail-closed policy/runtime fixtures. The workspace helper self-test covers positive and negative planning, progress classification, repository selection, task parsing, scope breaches, and Pi session-freshness arguments without launching Pi.
