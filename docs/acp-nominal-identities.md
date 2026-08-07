# ACP nominal identities

The ACP stdio host admits raw protocol frames before reducer or facade use. Private Rust roles prevent selected identity categories from entering the wrong internal API.

## Baseline

The pre-change `scripts/cairn/check-acp-stdio-host --root .` rail passed all 32 ACP tests and its linked coding-session, TUI, facade, Clippy, and isolation checks.

The baseline focused source identity was `af369f96a99d89b12e3cf0020727fdcd9d580b9211f0d96d54d796d35b4bf707`.

<!-- r[impl agent_harness_runtime.acp_nominal_identities.inventory] -->
<!-- r[verify agent_harness_runtime.acp_nominal_identities.inventory.exclusions] -->

## Inventory

| Field class | Selected values | Nominal posture |
| --- | --- | --- |
| External ACP identity | correlation, ACP session, run, entry, session reference | Bounded non-empty external wrapper |
| Animus identity | Animus session, connection, session, authority profile, workspace, resource, receipt | Lowercase BLAKE3 wrapper |
| Protocol constant | method, protocol version, cohort | Keep as protocol text |
| Display text | safe name, status label | Keep as bounded text |
| Diagnostic text | failure detail, redaction marker | Keep as safe diagnostic text |
| Deferred | tool permission request ID | Keep under the existing permission contract |

## Wire boundary

`codec` decodes the reviewed JSON-RPC shape into bounded raw DTOs. `admission` converts supported requests into private nominal roles.

Malformed, oversized, whitespace-bearing, or unsupported digest values fail before state reduction. Unsupported methods keep safe method text and optional correlation metadata.

## State and facade boundary

Connection state uses nominal connection, session, correlation, run, and authority roles. Facade commands use nominal correlation, session, run, entry, workspace, and session-reference roles.

JSON-RPC responses and metadata-only receipts convert roles back to their original strings only at projection. Existing frame and receipt tests establish compatibility.

<!-- r[verify agent_harness_runtime.acp_nominal_identities.compatibility] -->
<!-- r[verify agent_harness_runtime.acp_nominal_identities.compatibility.golden] -->

## Compiler evidence

The opt-in `nominal-compile-fixtures` feature exposes hidden fixture constructors only for Rust documentation tests. Same-role input compiles.

Compile-fail fixtures cover these substitutions:

- ACP session for Animus session;
- session for connection;
- correlation for run;
- run for entry;
- workspace for resource;
- session for authority profile.

<!-- r[impl agent_harness_runtime.acp_nominal_identities.docs] -->

## Authority boundary

Typed identities prove local category separation and checked syntax only. They do not grant client authorization or permission.

They do not prove sandbox enforcement, provider correctness, session replay equivalence, arbitrary ACP client compatibility, production readiness, or release eligibility.
