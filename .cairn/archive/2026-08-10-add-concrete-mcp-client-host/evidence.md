# Concrete MCP client host evidence

## Scope

This change adds one separate non-default MCP client host. The host supports only newline-delimited JSON-RPC over standard input and output for MCP `2025-06-18`.

The change does not add Streamable HTTP, SSE, remote catalogs, transport fallback, shell fallback, inherited environment variables, ambient server discovery, or an MCP SDK runtime dependency.

## Baseline

The pre-change baseline passed these focused surfaces:

- existing MCP bridge and tool executor tests;
- request-context contribution tests;
- SecretSpec host-resolution tests;
- Basalt authority tests;
- runtime authority-seal tests; and
- dogfood-readiness validation.

The baseline did not provide a concrete MCP client process host.

## Reviewed protocol cohort

| Item | Exact value |
| --- | --- |
| MCP cohort | `2025-06-18` |
| Specification revision | `b25c0874bf0ba699a58e21ef06f659d839659de3` |
| Schema BLAKE3 | `528797f6970a8f24175daf1f994c742f2fe24f0b8230b9456885452a08132a90` |
| License BLAKE3 | `fd12707e19e104da035c74958a676b0567b7dc2df01875ae07f058b79e59422f` |
| Runtime dependency | none |

The reviewed upstream cohort includes the MCP specification schema and license. The host implements only the bounded methods needed for initialization, initialized notification, tool listing, tool calls, and cancellation notification.

## Behavioral reference

Autolith revision `8755dc516c9f6c1126b148d3fc426312095de871` is an ISC-licensed behavioral reference. This change adapts failure scenarios for startup, discovery, cancellation, credential drift, process-tree teardown, and instruction trust.

Animus does not depend on Autolith at runtime. It does not adopt Autolith's Common Lisp architecture, saved images, S-expression configuration, Bash supervisor, global mutable process state, transport set, or security claims.

## Implemented boundaries

- `config/mcp-client-host.ncl` owns typed exact configuration and finite limits.
- `config/generated/mcp-client-host.json` is the deterministic runtime projection.
- Rust revalidates the projection before any process or credential effect.
- Pure modules own configuration, lifecycle, discovery, authority, context, protocol decoding, and receipt projection.
- The process shell owns direct launch, cleared environment, newline framing, deadlines, cancellation observation, and teardown.
- Existing Animus MCP mapping, tool authority, request-context, and safe-evidence contracts remain authoritative.
- Unix uses an owned process group. Other platforms report direct-child scope only.
- Teardown uncertainty blocks same-server restart.

## Deterministic conformance

The local fixture server covers:

- successful initialization, instructions, complete discovery, call, and teardown;
- optional and required server failure visibility;
- wrong protocol cohort;
- malformed initialization and call frames;
- early exit and oversized frames;
- partial, duplicate, oversized, and schema-drift catalogs;
- current authority and stale or widened authority denial;
- credential resolver failure and credential drift;
- explicit cancellation and deadline cancellation;
- Unix descendant-process teardown;
- untrusted advisory current-request instructions;
- safe receipt projection and every prohibited leak class; and
- default dependency isolation plus the explicit root feature.

The packaged smoke uses the local fixture server. It uses no network and no live credential.

## Current verification

The final focused rail passed with receipt BLAKE3 `ab3f2b3aea7eef11181cca56a83fed2a86c5da2eeb1a710224d249099e6ea19f`.

The receipt binds these implementation identities:

- source: `0daf1f69119a61f4b3208920e98380a5db1f6691da62e7f7efdd22a4e3d5a37d`;
- tests: `f59f5cb7541a6e7a5cff2d3db3a644e0c3387c1e8f9fcd7c526d61972bf1e745`;
- generated configuration: `bc3a0c9ad0cb9607e1cff7809a70d1815931ae3e1bbb7dd838954c84750fbca0`;
- documentation: `f5c8975344f816c273911114fb86ec652a0829479572afe17192cc7d83ebe83a`; and
- focused rail script: `d0f22f47bd881aed577c2a80417f15e68e4446a8064d523d94460e4f6c011f66`.

These checks passed:

```text
nix develop -c scripts/cairn/check-concrete-mcp-client-host --root .
nix build .#checks.x86_64-linux.concrete-mcp-client-host -L --no-link
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo tigerstyle check
scripts/cairn/check-component-catalog --root .
scripts/cairn/check-runtime-facade-inventory --root .
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal add-concrete-mcp-client-host --root .
nix run .#cairn -- gate design add-concrete-mcp-client-host --root .
nix flake check -L
```

The proposal gate receipt is `d02e57839ab3fe59d7d9f42d9e95a8603e1c22b5f3f22b835c357d2737db95ec`. The design gate receipt is `ee20cdd8f6d0abace0ffff1a311c0b0f05bc5c83077471f6fa41319fce879a7e`.

The configured remote Nix builder was unavailable over SSH. Nix used the local builder and completed the requested checks.

## Requirement promotion and traceability

The sync dry-run passed with receipt `5755aacf44fd5f25ed7d5e9a4b93a2fdf323029afae9386fc87721bd9447b088` and plan `554ad532160d4c8a59b975d268205128daf73676cced1226d5955f270fdef6cc`.

The executed sync passed with receipt `7a0fd5a2910ef0fd0d932ab75c9233d64736ab12203bffd9b5bee30a2643e392`, plan `f323f5f55d8ed88e589b1f6ab85ec829a579f7c27cf82980e35582ae4f217270`, and mutation manifest `b3a52e5dd0aec604ca13812ed0fdb734dba6246635f89f8e11900640d217805a`.

The accepted `agent-harness-runtime` spec now has BLAKE3 `ab8eb87178865512ca88d007e4f9211dcd2a1c0fcac16fd09c8db05fbc02f795`. Post-sync Cairn validation passed.

Scoped traceability for `agent_harness_runtime.mcp_client_host.*` reports zero missing and zero dangling requirements. The full traceability report has BLAKE3 `eb0137bc4377fbf8376d62c3097fe6326be4c71c5feb1971f0f11a58fe60a298` and receipt `003ab7fb19f3b8bd2401d7d3475e11c2fa50a065df866aed1364c91be0045f07`.

The repository-wide traceability verdict remains invalid because of unrelated accepted-requirement gaps. This change does not claim a global traceability pass.

The task gate passed with receipt `caef123c565a531671fcb97885771b44a3b458aa0778bfde0d4840979a606a93`.

The archive dry-run passed with receipt `91330aa99778f52710cef1f9c21014843388dc46a0ff658f39c3782ce3561196` and plan `e13c9212c3056354f457ad2c50f67436562c883d987c38d4657ce2dc07f89afb`.

Archive execution passed with receipt `c4d823524af15d2feca2d1dea0742873258ee977088f8bc1993c58d733894644`, plan `b4ef5b5899ad75386dd039e74d82059fce45891deb6683d8f2f608fcebdbe646`, and mutation manifest `41766053e5741b3fbf2510a4188bc72f9f69a4fa621fa7c0bd700bb29bbb346b`.

Post-archive Cairn validation passed with no active changes. Exact-head publication evidence remains pending.

## Non-claims

This evidence does not prove server trust, server correctness, tool safety, sandboxing, network isolation, exactly-once effects, arbitrary MCP compatibility, full MCP parity, production readiness, or release eligibility.
