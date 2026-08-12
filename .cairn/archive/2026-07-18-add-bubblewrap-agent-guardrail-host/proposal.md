## Why

Animus already owns portable sandbox descriptors, pure policy decisions, platform capability matching, and redaction-safe receipts. The active Sandlock host-adapter change targets stricter Linux enforcement with Landlock, seccomp, COW, resource, network, and HTTP controls, but it does not provide a smaller interactive-agent guardrail optimized for hiding unrelated host files while preserving a familiar development filesystem layout.

Dpc's `isolate` repository demonstrates a useful bounded pattern: compile one trusted central profile into a deterministic Bubblewrap plan, default binds to read-only, validate the active context without effects, and carry sensitive Bubblewrap options through a sealed memfd. Animus should adapt those mechanisms behind its existing authorization and receipt boundaries without adopting `isolate` wholesale, its YAML source of truth, its raw escape hatches, or its hostile-code non-claims as stronger guarantees.

## What Changes

- Add a non-default Linux-only Bubblewrap agent-guardrail host component and `animus-agent-guardrail` executable, isolated from reusable and default crate surfaces. r[agent_harness_runtime.bubblewrap_agent_guardrail.feature_gate]
- Define typed Nickel-authored guardrail profiles with checked runtime projections, explicit filesystem/network/device/socket/environment posture, and no project-local authority by default. r[agent_harness_runtime.bubblewrap_agent_guardrail.profile]
- Translate normalized Animus sandbox intent, authorization identity, active-context facts, and host-resolved root bindings into one deterministic typed launch plan with read-only defaults and explicit writable upgrades. r[agent_harness_runtime.bubblewrap_agent_guardrail.plan]
- Provide side-effect-free active-context validation that reuses the launch planner and fails before path creation or child execution. r[agent_harness_runtime.bubblewrap_agent_guardrail.preflight]
- Launch Bubblewrap through NUL-delimited options in a sealed anonymous memfd, require a compatible Bubblewrap cohort, and fail closed without an argv fallback. r[agent_harness_runtime.bubblewrap_agent_guardrail.sealed_transport]
- Preserve Animus/Basalt authorization ordering and keep the first tranche free of raw Bubblewrap arguments, command prefixes, passthrough, host command sources, and an independent privileged-command sidecar. r[agent_harness_runtime.bubblewrap_agent_guardrail.authority]
- Emit BLAKE3-bound safe receipts and add positive, negative, fake-Bubblewrap, and opt-in Linux behavior evidence. r[agent_harness_runtime.bubblewrap_agent_guardrail.receipts] r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]

## Dependencies and Sequencing

- This is a successor to `add-sandlock-linux-host-adapter` and `split-std-host-adapter-crates` so both Linux backends reuse the same portable descriptor, authorization, capability, and receipt boundaries while retaining distinct claim classes.
- Sandlock remains the strict Linux enforcement backend. Bubblewrap guardrail evidence remains bounded to accidental disclosure and damage reduction for the exact mounted profile.
- Mantle remains the owner of build sandboxing and build-hermeticity claims.
- The upstream `isolate` repository at canonical commit `8e17bc6f5ec87c52d4c69c59a1fe0abc9eda3776` is a reviewed reference cohort only, not a runtime dependency or copied public profile format.

## Impact

- **Rust**: a focused optional std host-adapter crate plus a thin executable shell; reusable crates retain only existing portable DTOs.
- **Policy**: typed Nickel profile contracts and checked runtime projections; no runtime Nickel evaluation.
- **Dependencies**: Linux and Bubblewrap are opt-in host prerequisites; default and non-Linux builds remain independent.
- **Security**: authorization remains external, writable roots are explicit, and successful guardrail launch does not prove hostile-code containment, kernel correctness, fine-grained egress control, or build hermeticity.
- **Testing**: pure plan/receipt tests, malformed and escape-hatch rejection, fake invocation checks, and opt-in real-Linux filesystem/network-mode behavior.

## Non-Goals

- Replacing Sandlock, Mantle, Nix build sandboxes, VMs, or kernel-level hostile-code containment.
- Adopting upstream YAML configuration, automatic writable repository discovery, SHA-256 stack identity, broad device exposure, raw Bubblewrap escape hatches, command prefixes, passthrough modes, or fail-open launch behavior.
- Claiming filtered network egress. The first tranche supports only explicit deny or inherited-host-network posture and records that distinction.
- Adding a privileged host-command sidecar. A later change may bind such a bridge to existing exec-policy, confirmation, and Basalt authorization contracts if concrete workflows require it.
