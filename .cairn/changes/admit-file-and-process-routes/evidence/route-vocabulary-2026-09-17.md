# Evidence: admitted route vocabulary

Date: 2026-09-17
Base revision: `a4b916e5c2da6293dc94c802432595d459473b75`

## What was verified

### The vocabulary admits every implemented route

`crates/animus-live-steel-agent/src/config.rs`: `ALLOWED_OPERATIONS` now lists
`context_read`, `file_edit`, `file_read`, `shell`, `subagent`, and `tool`.
`build_rlm_policy` maps each label to its `HostOperationClass`, so admission and
policy construction agree on the same vocabulary.

### The admission boundary still bites

`cargo test -p animus-live-steel-agent --test config` runs 7 tests, all passing:

| Case | Result |
|---|---|
| A configuration that admits `file_read` | admitted |
| A configuration that admits `kernel_clear` | refused, `InvalidCollection` |
| Generated Nickel configuration revalidates | pass |
| Provider cohort, drift, and malformed cases | pass |

`kernel_clear` is the refused label because the runtime implements a kernel route
but the vocabulary does not admit it, so the boundary is proved at the new edge.

```text
cargo clippy -p animus-live-steel-agent --all-targets -- -D warnings   clean
```

## Limits of this evidence

- This change admits routes. It does not supply a pack. A product still fails
  `validate_packs` unless it supplies the pack for every route it admits.
- `kernel_clear` remains unadmitted; a product with a kernel policy needs a
  separate widening.
- The worktree runs the focused crate tests and clippy. The Animus workspace-wide
  gate (`nix flake check`) is not run here.
