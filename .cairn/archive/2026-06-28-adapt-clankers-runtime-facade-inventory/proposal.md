# Proposal: Adapt Clankers runtime facade inventory for Animus

## Why

Animus is growing from a small crate scaffold into a reusable agent-harness runtime. Clankers has already exposed a failure mode Animus should avoid early: a public runtime facade can look embeddable while quietly mixing reusable contracts, host-injected service seams, executable policy, and product-shell adapters. Once those surfaces are mixed, downstream users cannot tell which APIs are safe no-ambient contracts and which ones imply filesystem, provider, credential, process, TUI, daemon, or plugin authority.

Animus should adopt Clankers' runtime-facade inventory discipline as a focused architecture rail. The goal is not to import Clankers runtime crates or product behavior; it is to classify Animus public exports, keep reusable contracts free of ambient authority, and catch accidental shell coupling before it becomes API.

## What Changes

- Define a runtime facade export inventory for Animus public APIs that classifies exports as reusable contracts, host-injection surfaces, or shell/product adapters.
- Require every inventory entry to name an owning crate, source module, authority class, stability intent, and forbidden side effects.
- Add a focused static rail that builds the inventory from crate exports or maintained markers and fails when new public exports are unclassified.
- Add positive fixtures for allowed reusable contracts and host-injection traits, plus negative fixtures for ambient filesystem, environment, credential, network, process, clock, TUI, daemon, plugin, and provider coupling in reusable surfaces.
- Publish the inventory or summary in architecture docs so embedders can depend on green contracts without confusing them with host shells.

## Impact

- **Files**: future architecture docs, public-export inventory fixtures or generated evidence, a focused validation script or Rust test, possible crate-level marker comments or metadata, and Cairn evidence.
- **Testing**: positive and negative static fixtures, dependency-isolation checks, focused inventory rail, workspace checks if public APIs change, and Cairn validation/gates before archive.
- **Non-claims**: this does not add a runtime facade implementation, daemon, TUI, provider router, plugin runtime, process manager, auth flow, or Clankers dependency. It only adopts the export-classification and drift-rail pattern.