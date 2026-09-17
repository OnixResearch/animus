# Change: Admit the file-read, file-edit, and shell routes

## Why

The runtime already implements seven host route classes. `RouteClass` carries
`FileRead`, `FileEdit`, `Shell`, `Tool`, `Subagent`, `ContextRead`, and
`KernelClear`; `animus-std-rlm-host` carries a pack for each; and the Steel tool
surface already emits `file_read` from `(animus-host-read target)`.

One private constant keeps a product from admitting four of them.
`ALLOWED_OPERATIONS` in `animus-live-steel-agent` lists only
`context_read`, `subagent`, and `tool`, and `validate_operations` refuses any
admitted operation outside that set.

A product that serves the file-read route therefore cannot say so. The pinned
product `pi-steel-repl` carries `CapabilityPacks { file_read: None, file_edit:
None, process: None, .. }` for exactly this reason.

## What Changes

- Widen `ALLOWED_OPERATIONS` to the route classes the runtime implements:
  `context_read`, `file_edit`, `file_read`, `shell`, `subagent`, `tool`.
- Keep `REQUIRED_OPERATIONS` unchanged. Admission stays a subset choice: a
  product must still admit `context_read` and `subagent`, and still supplies a
  pack for every route it admits.
- Move the refused label in the admission boundary test from `shell` to
  `kernel_clear`, which the runtime implements but the vocabulary does not admit,
  so the boundary still bites at the new edge.

## Non-Goals

- No new route class, pack, or request.
- No change to `REQUIRED_OPERATIONS`, bounds, or authority.
- No degree-of-freedom for `kernel_clear`, which stays outside the vocabulary
  until a product carries a kernel policy to admit.

## Impact

- `crates/animus-live-steel-agent/src/config.rs`: the admitted vocabulary.
- `crates/animus-live-steel-agent/tests/config.rs`: the positive and negative
  admission cases.

## References

- `crates/animus-std-rlm-host/src/routes.rs`: the packs the vocabulary unlocks.
- `crates/animus-adapter-rlm-kernel/src/request.rs`: the operation labels.
