## Result

The composed local live path now preserves disabled reasoning and advances the terminal cursor for every private runtime event.

## Deterministic evidence

- Baseline `cargo test -p animus-live-steel-agent --all-features` passed before the change.
- Final `cargo test -p animus-live-steel-agent --all-features` passed, including disabled, low, and unknown reasoning projections.
- `cargo test -p animus-live-terminal --all-features` passed with 33 positive and negative tests.
- The terminal tests include ordered private cursor advancement plus stale and skipped sequence rejection.
- Both changed crates passed all-target and all-feature Clippy with warnings denied.
- Formatting passed.
- Cairn proposal, design, tasks, and validation gates passed.

## Downstream live evidence

Animus Steel was built with this committed Animus worktree as an explicit temporary Nix input override. Its complete release build ran all 30 product tests successfully.

A bounded daemon and pseudo-terminal run selected provider `local-dspark` and model `deepseek-v4-flash-0731-ablit-100`. The model returned the expected private assistant value. Retained evidence records only:

```text
product_live_smoke=passed
provider=local-dspark
model=deepseek-v4-flash-0731-ablit-100
response_items=1
private_output=omitted
credential=omitted
```

The pseudo-terminal restored its screen and cursor state. The scripted harness later reached end-of-input, so a clean operator-requested terminal exit is not claimed by this live run. Deterministic lifecycle tests cover idempotent restoration and explicit detach.

A separate local tool-capable initial stream completed through the production parser with zero tool calls. A forced live `steel_repl` continuation is not accepted as release evidence: the local server produced a tool call and tool result, but the continuation did not complete. Fixture-backed continuation remains covered for the reviewed OpenAI event cohort.

## Existing repository blockers

The full Animus Octet derivation remains blocked by pre-existing findings outside these changed crates. The consuming product's strict Octet check is the release gate for the selected closure.

The broad `live-steel-agent-shell` check remains blocked by pre-existing runtime-facade inventory entries from the terminal-control branch. Focused Rust and downstream product checks pass.

## Non-claims

This evidence does not prove network confidentiality, service reliability, model quality, live local tool continuation, clean scripted end-of-input handling, production readiness, or general endpoint compatibility.
