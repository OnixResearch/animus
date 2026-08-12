## Context

`animus-adapters` already normalizes portable `SandboxDescriptor` values, evaluates sandbox policy and declared platform support, and projects safe receipts. `animus-std` owns host effects. The active Sandlock change supplies a strict optional Linux backend, while interactive coding agents also need a lower-cost profile that can hide credentials, sibling repositories, unrelated configuration, and host sockets while keeping an admitted worktree and development tool closure usable.

The useful `isolate` mechanisms are deterministic typed filesystem planning, active-context validation, sealed Bubblewrap argument transport, central trusted configuration, and explicit nested state. Its threat model remains a convenience guardrail for accidental reads, leaks, and damaging mistakes. Its trusted raw arguments, command prefixes, passthrough rules, broad mounts, command environment sources, and privileged sidecar are not accepted into the first Animus tranche.

## Decisions

### 1. Keep Bubblewrap and Sandlock as distinct backend claim classes

**Choice:** Add Bubblewrap as `agent-guardrail`, separate from the stricter Sandlock host backend and from Mantle's build sandbox. Backend selection occurs only after portable policy and authorization admission.

**Rationale:** Bubblewrap mount/namespace isolation is useful for interactive agent privacy, but it does not provide Sandlock's complete requested mechanism set or Mantle's build-hermeticity evidence. Separate claim classes prevent a passing lightweight profile from being promoted into a stronger security statement.

### 2. Isolate host authority in a focused crate and thin executable

**Choice:** Implement pure profile normalization, typed plan construction, validation decisions, and receipt projection in a focused authority-scoped std crate. A thin `animus-agent-guardrail` executable owns host path inspection, memfd creation, Bubblewrap probing, process replacement, exit handling, and ephemeral diagnostics. Default features and reusable crates do not acquire Bubblewrap or process authority.

**Rationale:** Tile and other PTY hosts can launch one executable as the supervised child while Animus retains the policy and evidence semantics. The functional core remains testable without namespaces, PTYs, filesystems, or subprocesses.

### 3. Use Nickel as the profile source and checked data at runtime

**Choice:** Operator-authored profiles use typed Nickel contracts and export deterministic checked runtime data. Profiles name safe root labels, filesystem modes, network posture, device/socket classes, inherited environment-key allowlists, required backend/cohort facts, and resource bounds. Project-local files cannot contribute policy unless a separate trust decision admits a reviewed projection.

**Rationale:** A central trusted profile avoids repository-controlled sandbox weakening. Runtime Rust consumes checked data rather than evaluating Nickel or loading upstream YAML.

### 4. Build one deterministic typed launch plan

**Choice:** The pure planner accepts a normalized descriptor, matching authorization and policy identities, one selected profile, canonical active-context facts, and host-resolved root bindings. It returns ordered typed operations and a domain-separated BLAKE3 plan identity. Read-only bind exposure is the default; writable, device-capable, socket, network-inherit, and environment inheritance are explicit. Ancestors precede descendants, compatible duplicate binds normalize deterministically, and incompatible target operations fail.

Symlink observations are supplied by the shell as bounded facts. The core rejects unresolved, escaping, type-mismatched, or conflicting bindings. Raw Bubblewrap operations are absent from the public profile and plan schema.

**Rationale:** Bubblewrap option order changes the resulting namespace. A typed intermediate plan makes conflict handling, review, validation, and receipt identity independent of incidental config ordering.

### 5. Validate the active context without mutation

**Choice:** `animus-agent-guardrail validate` reuses profile selection, authorization binding checks, root resolution facts, typed filesystem planning, environment-key admission, executable/cohort checks, and capability decisions. It may inspect supplied paths and bounded metadata, but it does not create configured paths, open a PTY, start Bubblewrap, run profile commands, or launch the requested child.

**Rationale:** Validation should predict the exact selected launch plan while remaining safe as a preflight. Inactive profiles require separate representative invocations rather than guessed contexts.

### 6. Carry Bubblewrap options through a sealed memfd

**Choice:** The executable encodes finalized Bubblewrap options as NUL-delimited bytes in a rewound sealed anonymous memfd and launches a compatible Bubblewrap implementation through `bwrap --args FD -- COMMAND`. The descriptor becomes inheritable only for the final exec boundary. Preparation or compatibility failure stops execution; there is no option-on-argv fallback.

The initial compatibility cohort requires Bubblewrap `0.11.2` or a separately proven implementation of the required `--args` behavior. The executable and cohort identities are BLAKE3-bound in plan and receipt data.

**Rationale:** Generated environment and mount options should not appear routinely in process argv. Sealing prevents mutation, while documentation remains explicit that same-UID host processes may still read the descriptor or process memory.

### 7. Make every host exposure explicit

**Choice:** The first profile schema supports explicit network `deny` or `inherit`, but no hostname, address, or HTTP filtering claim. Device and host-socket exposure default to denied. The worktree is writable only through an admitted root binding; Nix store/tool closures and required system material are read-only. Credential directories, sibling repositories, unrelated config, display/input sockets, and agent-control sockets are absent unless a reviewed profile names a specific capability class.

**Rationale:** A broad convenience baseline would undermine the agent privacy goal. Provider connectivity sometimes requires inherited networking, so the receipt must distinguish connectivity from egress confinement.

### 8. Preserve authorization ordering and reject escape hatches

**Choice:** Plan construction requires a matching prior Animus/Basalt authorization identity. Sandbox success cannot mint or widen authority. Managed profiles reject raw Bubblewrap arguments, command prefixes, passthrough, host command environment sources, implicit shell strings, and automatic fallback to an unguarded process.

The first tranche does not implement an outside privileged-command sidecar. Existing Animus exec-policy and confirmation contracts remain the only planned basis for a later bridge.

**Rationale:** Escape hatches and independent command policy would create a second authority system and make the effective profile difficult to review.

### 9. Keep receipts bounded and backend-specific

**Choice:** Durable receipts contain safe profile/backend/cohort labels, descriptor/policy/authorization/plan identities, requested and enforced capability labels, network posture, root-label summaries, executable identity, status/issue class, exit classification, cleanup status, and a domain-separated BLAKE3 receipt identity. They exclude raw prompts, credentials, environment values, full host paths, mount payloads, command bodies, stdout/stderr bodies, and debug logs.

**Rationale:** Operators need evidence that the selected plan was enforced without turning the receipt into a second disclosure channel.

### 10. Verify the pure core normally and namespaces on explicit Linux rails

**Choice:** Ordinary tests use fake path facts, capability probes, memfd/Bubblewrap executors, and execution observations. An opt-in Linux rail proves one exact profile can edit its admitted worktree while denying credential, sibling-repository, outside-write, symlink-escape, and network-deny fixtures. Missing namespaces or Bubblewrap are blocked outcomes, not passing skips.

**Rationale:** Most regressions are deterministic planning errors, but real namespace behavior still requires host evidence.

## Risks / Trade-offs

- Bubblewrap shares the host kernel and same user identity; passing evidence is not hostile-code containment or same-UID secrecy.
- Provider access may require inherited networking or explicit local sockets. Such profiles reduce filesystem exposure but do not prove egress filtering.
- Host path inspection before Bubblewrap launch is not a TOCTOU-safe defense against same-UID races; capability-root resolution and exact source facts reduce but do not eliminate that risk.
- A separate executable adds packaging and version-handshake work, but avoids leaking Bubblewrap and memfd mechanics into PTY hosts.
- The reference project is young. Animus pins the reviewed documentation/source cohort for comparison but implements and tests an Animus-owned contract.

## Validation Plan

- Cairn proposal, design, tasks, and validation gates.
- Nickel export freshness and positive/negative profile fixtures.
- Focused pure planner, authorization, preflight, receipt, and fake invocation tests.
- Default/no-default/non-Linux dependency isolation checks.
- Opt-in real-Linux behavior checks for supported hosts, with exact blockers recorded when unavailable.
- Documentation and safe-receipt leak checks before archive.
