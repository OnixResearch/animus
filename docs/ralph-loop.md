# Ralph loop profile

Animus includes `ralph-loop` as a bounded Steel runtime policy profile inspired by Geoffrey Huntley's Ralph loop pattern, the `snarktank/ralph` script, and Anthropic's Ralph Loop plugin.

## Runtime shape

- Policy profile: `ralph-loop`
- Script binding: `policy/steel-runtime/scripts/ralph-loop.scm`
- Completion promise: `COMPLETE`
- Policy turn ceiling: ten runtime turns

The script calls the model with compact Ralph instructions and the runtime input summary as task state. On later turns, it checks the previous model message. If the message contains `COMPLETE`, the script returns that message as the final response. Otherwise, it calls the model again with the original task state plus the previous iteration summary.

## Host-owned effects

The profile does not copy Ralph's shell script or Claude plugin hook behavior. Hosts still own every effectful boundary: loading PRD/progress files, creating fresh model sessions, checking branches, committing, running validation, updating `AGENTS.md`, browser checks, shell commands, credentials, provider sessions, network access, and persistence.

Use `ralph-loop` only when the host supplies explicit capabilities and safe task state. A missing completion promise does not grant more authority; the normal runtime max-turn policy stops the run.

## Verification

Run the focused rail after changing this profile:

```sh
scripts/cairn/check-steel-runtime-policy --root .
```

The rail checks Nickel export drift, script hash binding, positive activation, Ralph prompt/reprompt/completion behavior, and negative fail-closed policy/runtime fixtures.
