## Context

The accepted `agent-harness-runtime` specification already separates portable sandbox intent from host effects. `animus-adapters` can normalize descriptors and decide whether declared platform capabilities are sufficient, but it deliberately cannot inspect a kernel, resolve capability roots, launch a process, or collect an enforcement result. Sandlock supplies a Rust host implementation for Landlock, seccomp-bpf, seccomp user notification, COW staging, resource supervision, and network/HTTP controls.

The integration must preserve that boundary. Animus and Basalt decide whether an action is authorized. Sandlock is only an enforcement mechanism selected after that decision.

## Decisions

### 1. Keep the adapter opt-in and Linux-only

**Choice:** Add a non-default `sandlock-host` feature to `animus-std`, compiled only on Linux. Pin the reviewed Sandlock compatibility cohort through the lockfile and an exact source revision or released crate version. Default features, reusable crates, and non-Linux builds do not acquire Sandlock.

**Rationale:** Sandlock currently depends on Linux facilities and an evolving API. Experimental host enforcement must not widen Animus's portable core or default dependency closure.

### 2. Use the Rust core API, not Sandlock profile or CLI text

**Choice:** The imperative adapter invokes the reviewed `sandlock-core` API. It does not generate Sandlock TOML, parse human CLI output, or shell out through reusable logic. A pure translator maps a normalized Animus descriptor plus already-resolved safe root bindings into an owned `SandlockLaunchPlan`; a thin Linux shell realizes that plan.

**Rationale:** Typed API integration avoids accidental dependence on a beta-facing CLI/profile grammar and makes translation independently testable.

### 3. Resolve authority-scoped roots only in the host shell

**Choice:** Portable descriptors continue to carry safe root labels. The host receives capability-root handles or confined relative roots, validates them against its grant table, and only then resolves the concrete paths needed by Sandlock. Absolute paths may exist transiently in the shell but never cross back into portable DTOs or receipts.

**Rationale:** Model-visible plans must not acquire ambient filesystem authority or leak host topology.

### 4. Probe requested mechanisms, not kernel versions

**Choice:** Preflight asks the selected Sandlock cohort to prove the exact requested mechanisms are usable. Capability results distinguish filesystem confinement, syscall filtering, user-notification supervision, COW staging, process and memory limits, egress/bind controls, HTTP interception, environment filtering, and deterministic time/randomness. A version string alone is not sufficient evidence.

**Rationale:** Kernel version, configuration, LSM state, seccomp support, and host policy can disagree. Functional capability probes are the enforceable boundary.

### 5. Do not silently waive or partially enforce constraints

**Choice:** The initial adapter supports only fail-closed dispatch. If any requested class cannot be translated or probed, no child process starts. Animus may choose a different adapter before selecting Sandlock, but once selected the host adapter never drops a constraint or turns a required control into an upstream waiver.

**Rationale:** Partial enforcement is especially dangerous for network, HTTP, COW, and resource constraints because a process may already have started before the gap becomes visible.

### 6. Preserve authorization ordering

**Choice:** Adapter entry requires a typed, matching authorization decision produced by existing Animus/Basalt checks. The pure admission core verifies action, subject, target, descriptor, and policy identities agree before producing a launch plan. Sandbox preflight or execution receipts cannot mint, widen, or substitute for authority.

**Rationale:** Confinement limits an authorized action; it does not authorize an otherwise denied action.

### 7. Bound process results and receipts separately

**Choice:** Live command output remains an ephemeral host result governed by existing output limits. Durable sandbox receipts contain only safe status and issue classes, descriptor/policy/authorization/cohort identities, enforced capability labels, bounded COW counts or digests, bounded output digests, exit classification, and a domain-separated BLAKE3 receipt identity.

**Rationale:** Operators need enforcement evidence without persisting prompts, credentials, file contents, network payloads, raw stdout/stderr, or host paths.

### 8. Test the pure core without Linux and behavior on dedicated hosts

**Choice:** Translation, admission, capability decisions, and receipt projection use fake capability/root/executor inputs in ordinary tests. An opt-in Linux rail runs positive confined commands and negative escape/network/COW cases against the pinned Sandlock cohort. Missing host prerequisites are reported as blocked, never passed.

**Rationale:** Most regressions should be cheap and deterministic, while kernel behavior still needs real execution evidence.

## Risks / Trade-offs

- Upstream Sandlock APIs and minimum kernel support can change. The feature pin and explicit cohort receipt make upgrades deliberate.
- Direct library integration adds an optional async/process dependency surface to `animus-std`; keeping it feature-gated limits that cost.
- Functional probes add startup work, but caching a probe across incompatible policies or kernels would weaken evidence. Any cache must bind the full host/cohort capability identity.
- Sandlock shares the host kernel. Passing this rail is not equivalent to a VM boundary or proof of kernel/LSM correctness.

## Non-Goals

- Replacing Mantle's `bwrap` build sandbox or claiming build hermeticity.
- Moving Sandlock types into `animus-core` or `animus-adapters`.
- Supporting upstream profiles, OCI runtime integration, dynamic policy callbacks, credential injection, GPU access, port remapping, or COW fork/map-reduce in the first adapter.
- Treating sandbox success as Basalt authorization, release readiness, or universal containment proof.
