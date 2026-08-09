## Baseline

The required `scripts/check-drain-evidence-guard.rs` path did not exist in Animus or another OnixResearch workspace repository. Cairn drains therefore retained completed worktrees instead of claiming safe teardown.

## Architecture review

A bounded serial search compared the workflow contract, current Animus Rust-checker conventions, and an existing standalone guard shape from `brittonr/workday-rs` revision `0f57e8c868d7fd822490340a163d5f41f4fde29f`. Animus keeps only the general standalone-script pattern. Its implementation adds strict linked-worktree classification, missing-target denial, internal-symlink denial, bounded counts, sanitized labels, and metadata-only output.

## Implementation

- Guard source BLAKE3: `1d03f7f94ddefc068f8cfce1c85b605ed197d81c4792268d29e660b1cf4f21e6`.
- The pure core classifies primary, linked, malformed, missing, unreadable, empty, live-entry, and symlink observations.
- The shell lists registered Git worktrees, observes bounded metadata, and does not read evidence content.
- Output retains target labels, classifications, bounded counts, truncation state, and verdicts only.

## Verification

- `rustc --edition=2021 -D warnings scripts/check-drain-evidence-guard.rs` passed.
- `scripts/check-drain-evidence-guard.rs --self-test` passed positive and negative classification, argument, filesystem, symlink, bounded-count, and output-redaction cases.
- An explicit scan of the implementation worktree passed with `.pi/` absent.
- `nix build .#checks.x86_64-linux.drain-evidence-guard --no-link -L` passed after proving the Nix build includes the required linker.
- The first broad `nix flake check -L` found a pre-existing continual-harness check that still required its archived task file from `cairn/changes/`. The checker now binds the existing `2026-08-07` archive path. Its focused rail and all twelve continual-harness tests pass after the repair.
- The second broad run found that the Octet derivation rewrote only an obsolete exact vendor-directory form. The Nix shell now rewrites all generated `cargo-vendor-dir/` source paths to the immutable store vendor root.
- The repaired Octet derivation reaches source linting and reports `348` existing `animus-adapters` findings across host-authority, messaging, Pijul, SecretSpec, skill-source, and trajectory files. This blocks a full flake verdict. It does not affect the guard source, its focused Nix check, the continual-harness repair, or Cairn validation.
- Final proposal receipt: `a5d46e9181dd0086bfa5b3d46a4d306624200e8ecc8dca430964dbfaec2243d5`.
- Final design receipt: `49c8369aa6ec6d0352d42e85067c6f3ad5a1c178c38212124f147db43e0d835f`.
- Tasks receipt with four implementation tasks complete: `ac8397ba7459fb98efdab186d297cbb50227cddcc435cbfb12119e4b19d8824e`.
- Accepted-spec dry-run plan: `b92d6dbe6fcc46761fe55258acf0ccdd8b58b05c540f88f7388ba6ff6b7f6b84`.
- Accepted-spec dry-run receipt: `dcbef92b467d3f7413b3b7d8ebaebf333e1586baba901b95127b4eeeaaa0ad4c`.
- Final tasks receipt: `d27716854ce5884f784b0d0e4c836a9c455f09e6e869a7d0e4043571f3e9c6f1`.
- Accepted-spec execution plan: `8d85f58156aec591077e288c29cef6562a2b3c1f955a895a9a90eb8428162cc9`.
- Accepted-spec sync receipt: `4072dff6dd19ee844c79ddad71636305e5e8b252896e18db4a8d16f7986aeb0e`.
- Accepted specification hash after sync: `2168cc27e32e12b6015b3d6280770dcd6384c0cde3f3b19af62fe1f3deaace65`.

## Non-claims

Passing proves only that the scanned linked worktree held no removable `.pi/` entries under the checked metadata rules. It does not prove evidence authenticity, completeness, correctness, durability, confidentiality, ownership, freshness, safe worktree removal, repository cleanliness, branch integration, or release eligibility.
