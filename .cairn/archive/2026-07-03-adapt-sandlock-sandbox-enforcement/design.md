# Design: Sandlock-inspired sandbox enforcement for the std shell

## Context

Sandlock is a lightweight Linux process sandbox that uses Landlock, seccomp-bpf, seccomp user notification, COW filesystem handling, and network/HTTP ACLs to confine untrusted processes without container setup. Those mechanisms are valuable reference points for Animus because guarded runtime actions can require execution constraints after capability authorization has succeeded.

Animus should keep the reusable runtime model portable and deterministic. The functional core can validate a descriptor and produce a normalized enforcement request, but the imperative shell must own kernel capability probing, process spawning, filesystem staging, seccomp/Landlock interaction, environment setup, and platform errors. Sandlock therefore informs the shape and tests of the `animus-std` shell adapter rather than becoming a core dependency.

## Decisions

### 1. Sandlock is a bounded reference, not a dependency plan

**Choice:** Use Sandlock as a reference for Landlock/seccomp-style confinement, COW workdir staging, network and HTTP ACL semantics, resource-limit supervision, deterministic time/randomness knobs, and explainable enforcement outcomes. Do not vendor Sandlock, call Sandlock from core crates, or copy its CLI/profile format as the Animus descriptor contract.

**Rationale:** Animus needs a portable descriptor boundary and optional platform adapters. A direct dependency would couple embedders to Linux kernel features and Sandlock release behavior before the Animus-owned contract is stable.

### 2. Descriptors are portable execution constraints

**Choice:** Define Animus sandbox descriptors in terms of filesystem reads/writes, working directory behavior, process limits, environment visibility, network egress/bind policy, optional HTTP method/host/path policy, deterministic time/randomness settings, and COW or dry-run write disposition.

**Rationale:** These fields capture the intent that policy and receipts need without exposing seccomp rule details or Landlock ABI versions in the reusable runtime surface.

### 3. Pure normalization precedes shell enforcement

**Choice:** Descriptor parsing, normalization, conflict detection, and safe receipt projection should be pure deterministic logic over typed inputs. The std-shell adapter consumes the normalized descriptor and performs platform probing and process orchestration.

**Rationale:** This preserves the functional-core / imperative-shell split. Positive and negative descriptor tests can run without kernel support, while platform smoke tests can remain focused and optional.

### 4. Required enforcement fails closed

**Choice:** When a policy caveat or capability request marks a sandbox descriptor as required, dispatch must fail before host execution if the descriptor is malformed, policy-denied, unsupported by the selected platform, or only partially enforceable.

**Rationale:** A sandbox descriptor is defense-in-depth, not authorization. Once required, silently relaxing it would make receipts and user expectations unsound.

### 5. Linux translation is explicit and capability-aware

**Choice:** The Linux std-shell adapter should translate normalized descriptors into a capability matrix inspired by Sandlock: Landlock for filesystem and supported network scopes, seccomp-bpf for syscall denial, seccomp notification for supervised operations, COW staging for protected workdirs, and proxy-style handling only when HTTP ACL inspection is explicitly requested and support exists.

**Rationale:** Naming the capability matrix keeps platform gaps reviewable. It also lets other adapters report unsupported features without pretending to implement Linux-only behavior.

### 6. Receipts record safe sandbox evidence

**Choice:** Dispatch receipts should record sandbox status, issue code, normalized descriptor hash, platform capability labels, enforcement outcome, COW change counts or hashes, and BLAKE3 receipt hashes. They must omit raw prompts, credentials, full path dumps, raw network payloads, raw file contents, and unbounded process output.

**Rationale:** Sandbox evidence is useful for audit and release review, but it can reveal sensitive paths or data if copied directly from process events. Receipts should prove what was checked without becoming a data leak.

## Risks / Trade-offs

- Linux-specific implementation detail can leak into portable crates. Keep kernel APIs behind `animus-std` or optional platform adapters.
- Sandlock capabilities may be broader than Animus needs. Start with descriptor intent and add fields only with positive and negative tests.
- HTTP interception and CA injection are sensitive. Treat them as explicit opt-in constraints with clear receipt evidence and denial modes.
- Kernel feature availability varies. Capability probing and unsupported-feature diagnostics must be deterministic and fail closed when enforcement is required.
- COW summaries can leak filenames. Use bounded labels, counts, and hashes unless policy explicitly permits more detailed reporting.