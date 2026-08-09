## Context

Animus has two separate validation paths. The dogfood readiness checker maps product claims to evidence-class labels. The release evidence checker validates head identity, dirty state, check status, and receipt freshness. Neither path currently proves that the packaged-headless dogfood claim uses the indexed current-head smoke receipt.

## Success Contract

The change is complete when one focused dogfood rail rejects a packaged-headless claim for missing, stale, failed, or dirty evidence. The rail must accept a clean index with one passing, head-matched packaged smoke receipt. Passing claim-hygiene fixtures alone, passing release fixtures alone, or an unclaimed packaged check are not completion evidence.

## Decisions

### Decision: Extend the release evidence pure core

**Choice:** Add `packaged-headless-dogfood` to the existing release evidence schema and checker. Bind it to `packaged-headless-dogfood-smoke`.

**Rationale:** The release checker already owns deterministic parsing, head checks, receipt freshness, and claim validation. A second parser can create drift and conflicting diagnostics.

### Decision: Keep the claim narrower than full current-head evidence

**Choice:** Require only the packaged smoke receipt, a clean head, and an exact head match. Do not require all pure release modes.

**Rationale:** The claim proves one dogfood evidence rung. It does not prove the full `current-head-pure-evidence` surface.

### Decision: Compose existing rails through the dogfood shell

**Choice:** The dogfood shell runs its claim-hygiene checker and the release evidence fixture rail. The Nix check supplies Rust, Nickel, BLAKE3, and linker tools.

**Rationale:** This creates one operator command without moving file or process authority into either Rust validation core.

## Considered Approaches

- **Duplicate release parsing in the dogfood checker:** Rejected because two parsers can diverge on dirty and stale semantics.
- **Run both rails independently:** Rejected because separate success does not establish one dogfood completion gate.
- **Treat packaged dogfood as full current-head evidence:** Rejected because that claim requires unrelated workspace, Clippy, flake, Nickel, and Cairn modes.

## Risks / Trade-offs

- The dogfood Nix rail gains a Nickel dependency because release fixtures use typed Nickel.
- The composed rail runs some release-index fixtures twice in broad repository validation.
- The claim remains bounded to packaged headless dogfood. It does not establish live-provider, TUI, daemon, plugin, production, or Clankers-parity readiness.
