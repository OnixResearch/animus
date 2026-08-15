## Why

Animus assembles agent context and re-invokes tool results across a session. Repeating work without a bounded retention contract grows memory without limit and hand-rolls eviction.

The stack now owns a product-neutral retention contract: the `bounded-cache` component and its reference design. Animus should own a bounded tool-result and context cache. Retention decisions belong in the pure core. Keys are BLAKE3 digests over canonical request projections.

## What Changes

- Add a `no_std` retention decision core in the Animus core crate: canonical key projection, eviction selection, promotion disposition, watermark trim plans, and deferred-release plans from supplied values.
- Add a bounded store in the Animus std shell that executes core decisions and releases results after the guard releases.
- Key every entry by BLAKE3 digest. Require an explicit capacity. Add no implicit default bound.
- Record the cross-repo contract: the cache policy follows the `bounded-cache` retention contract, and the store can later adopt a published `bounded-cache` revision.

## Impact

- **Files**: core decision module, std shell store, tests, and this lifecycle package.
- **Testing**: workspace tests, Clippy, formatting, Cairn validation and gates, and the core `no_std` wasm check where the repo supports it.

## Dependencies

New sibling component: `bounded-cache` (consumed by an exact published Git revision once available; retained as a policy reference until then).

External reference: `adanil-code/LRUHashTable` under Apache-2.0.

## Non-goals

- Do not change agent semantics. The cache is advisory only.
- Do not add sibling-path dependencies.
- Do not read clocks in the core.
- Do not claim that a cache hit proves tool correctness or context freshness.
