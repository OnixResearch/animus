## Why

The dogfood readiness rail validates claim-hygiene labels, but it does not bind a packaged-headless claim to a clean current-head receipt. The release evidence index records head and check facts, but it cannot express the packaged dogfood claim. This gap permits two separate passing rails without evidence that the claimed dogfood rung matches the recorded head.

## What Changes

- Add an exact `packaged-headless-dogfood` release-index claim.
- Require a clean head and a passing, head-matched `packaged-headless-dogfood-smoke` receipt for that claim.
- Add positive, missing, stale, duplicate, conflicting, and dirty index fixtures.
- Compose release-index validation into the existing dogfood readiness rail.
- Update the readiness, release-evidence, focused-rail, roadmap, and verification documentation.

## Impact

- **Files**: release evidence checker, generator, Nickel schema, fixtures, dogfood rail, Nix check inputs, docs, roadmap, and Cairn lifecycle artifacts
- **Testing**: dogfood readiness, release evidence index, current-head generator, roadmap, Cairn validation and gates, and the focused Nix check
