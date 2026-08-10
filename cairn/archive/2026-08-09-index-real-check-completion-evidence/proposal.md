# Index real-check completion evidence

## Why

The bounded real selected-check smoke now produces a clean-current-head receipt, but the release evidence index cannot represent or validate that receipt. A caller can record only a supplied pass status, which loses the selected check identities, source identity, pinned Cairn binding, and receipt head.

## What changes

- Add a separate `bounded-real-check-completion` evidence claim.
- Add a typed index record for the real-check receipt metadata.
- Let the current-head generator parse one explicit receipt path for this claim.
- Fail closed on absent, dirty, stale, malformed, fixture-mode, non-passing, source-unbound, wrong-check, or wrong-Cairn receipt data.
- Keep release readiness, production readiness, executable authenticity, sandboxing, hermeticity, and check-oracle soundness as non-claims.

## Success

The schema fixtures, generated-index rail, dogfood readiness rail, Nix check, Cairn gates, and a clean-current-head claim receipt pass. Positive and negative fixtures cover each new binding.
