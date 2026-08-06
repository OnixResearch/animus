# Model-facing Steel RLM kernel

Animus provides an optional persistent Steel environment through the `rlm-kernel` feature. This environment follows the reviewed Prime Agent RLM pattern.

Prime Agent uses persistent IPython as its model tool. Animus uses Steel and retains its existing Rust adapter boundaries.

## Components

The implementation has three layers:

1. `animus-adapter-rlm-kernel` provides pure no-std contracts.
2. `animus-steel::SteelRlmKernel` provides transactional Steel evaluation.
3. `animus-runtime::rlm_kernel` provides the injected host execution seam.

`animus-steel::SteelRlmToolAdapter` implements the standard Animus `ToolAdapter` contract. The tool name is `steel_repl`.

The tool schema is `animus.steel-repl.v1`. It accepts one string argument named `code`.

## Persistent environment

One `SteelRlmKernel` instance retains admitted context bindings and successful Steel cells. Later cells can use earlier variables and helper definitions.

The evaluator reconstructs a candidate engine before each cell. It commits the candidate only after all checks pass.

A failed cell does not change the committed environment. A timeout, bound violation, or malformed host request also leaves the environment unchanged.

Prompt compaction does not clear the kernel. `plan_compaction` records binding and definition counts and marks the state as preserved.

Only `clear_explicit` resets the environment. Compaction, goal changes, and child completion cannot request an implicit clear.

## Host requests

Model-written Steel has no direct effect channel. It returns typed request values through these functions:

```scheme
(animus-host-read "resource:README")
(animus-host-edit "resource:src/lib.rs" "replacement")
(animus-host-shell "cargo test")
(animus-host-tool "search" "typed-payload")
(animus-context-read "task_context")
(rlm "focused-child" "Review this module" 1 2 8000 60000 4096)
(animus-kernel-clear)
```

Rust parses and revalidates each request. The optional runtime then calls one injected `RlmKernelHostExecutor`.

The executor owns all concrete effects. File access, commands, tools, context stores, child lifecycle, clocks, credentials, and persistence remain outside the kernel.

## Bounds

`config/rlm-kernel-profile.ncl` owns the reviewed policy. Rust revalidates its generated JSON before use.

The policy names these finite bounds:

- source and environment characters
- binding and definition counts
- planned evaluation fuel
- structural recursion depth
- planned allocation bytes
- host request characters
- child depth and fan-out
- child tokens and wall time
- child result characters

The Steel adapter also interrupts a cell that exceeds its evaluator timeout. This timeout is containment for a stuck evaluation, not a security boundary.

## Receipts

Receipts contain policy identities, operation classes, bounded counts, child lineage identities, artifact identities, and compaction status.

Receipts reject variable values, code bodies, child transcripts, credentials, and raw paths. They also reject sandbox, correctness, task-success, readiness, and release claims.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-rlm-kernel --root .
```

Run the relevant Rust checks:

```sh
cargo test -p animus-adapter-rlm-kernel --locked
cargo test -p animus-steel --features rlm-kernel rlm --locked
cargo test -p animus-runtime --features rlm-kernel rlm_kernel --locked
cargo check -p animus --features rlm-kernel --locked
```

## Non-claims

The Steel environment is not a security sandbox. It does not prove model correctness, child correctness, task completion, production readiness, or release eligibility.

The profile covers the reviewed Prime Agent revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`. A later upstream revision requires a new review and policy update.
