# Design

## Why a vocabulary and not a capability

`RouteClass` and the packs already exist. The runtime can read a file, gate an
edit on a revision, and run a bounded process. Nothing in the vocabulary change
adds capability; it removes an accidental ceiling that made a product's own
choice of routes impossible to express.

## Two places, one vocabulary

Admission is validated twice: `validate_operations` bounds the configured labels,
and `build_rlm_policy` maps each label to its `HostOperationClass`. Widening one
without the other produces a configuration that validates and then fails policy
construction, so both carry the same six labels.

## Why the negative case moved

The boundary test refused `shell`. Leaving it there would make the test assert
that an implemented route is refused, which is the defect being fixed. The test
now refuses `kernel_clear`, an operation the runtime implements but the
vocabulary deliberately withholds, so the negative case still proves the boundary
rather than the accident.

## Bounds kept

- `REQUIRED_OPERATIONS` is unchanged, so a product still must admit context read
  and children.
- Admission remains a subset choice; the packs stay product-supplied, and
  `validate_packs` still refuses a binding without a pack.
