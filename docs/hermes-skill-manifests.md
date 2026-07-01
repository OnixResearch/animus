# Hermes-style skill manifests

Animus adapts the portable Hermes `SKILL.md` manifest shape as typed host-provided data, not as a dependency on Hermes directories or runtime state.

Hosts parse skills from any store they own — an application database, embedded bundle, project index, or Hermes-style markdown file — and pass validated fields into `animus-adapters`. Reusable crates validate names, descriptions, semantic versions, category metadata, tags, related skill names, provenance labels, trust status, and BLAKE3 content identities. They do not inspect `~/.hermes`, `~/.pi`, repository skill directories, symlinks, caches, or raw files.

The default advertisement surface is a `SkillResourceSummary`: name, description, category, tags, and safe provenance only. Full skill bodies are omitted until a host-approved load selects exactly one trusted skill by name and matching content identity. Untrusted project-local bodies, unrelated bodies, duplicate names, unresolved related skills, empty bodies, and mismatched content identities stay out of model context.

Validation evidence lives in `crates/animus-adapters/src/skill/tests/`: positive fixtures cover valid summaries, explicit selected-body loading, and non-Hermes stores; negative fixtures cover malformed fields, duplicate tags, unsafe handles, duplicate names, unresolved related skills, untrusted-body omission, mismatched body identity, and empty body omission.
