# Optional Bubblewrap agent guardrail

`animus-std-agent-guardrail` is a non-default Linux host adapter for reducing accidental disclosure and host damage by interactive agent processes. It is distinct from the stricter Sandlock backend and from Mantle/Nix build sandboxes. Passing evidence is scoped to one exact checked profile, explicit root facts, Bubblewrap 0.11.2 executable, and observed child outcome.

## Reviewed reference cohort

The design adapts bounded mechanisms from dpc's `isolate` at Radicle repository `rad:z3qqqx5cpk5jk9ioEGaw54dihfDwb`, commit `8e17bc6f5ec87c52d4c69c59a1fe0abc9eda3776`, licensed MIT OR Apache-2.0. Adopted mechanisms are typed deterministic mount planning, active-context validation, read-only defaults, sealed NUL-delimited argument transport, central trusted configuration, and explicit non-claims. Animus does not copy the YAML profile format, SHA-256 identity, raw Bubblewrap arguments, command prefixes, passthrough, broad mounts, host command environment sources, implicit shells, privileged sidecar, or fail-open integration.

## Profile authoring and update

The source of truth is `config/agent-guardrail-profile.ncl`. It declares the exact backend/cohort, central trust source, safe root labels and child targets, read-only or writable mode, required source kind, network `deny` or `inherit`, denied broad devices and host sockets, environment-key allowlist, bounds, disabled escape hatches, identity domains, redaction, and non-claims. Runtime Rust loads only `config/generated/agent-guardrail-profile.json`; it never evaluates Nickel or discovers project-local policy.

After review, regenerate and check the projection:

```console
scripts/config/export-nickel-configs
scripts/config/check-nickel-configs
```

The checked `tool-store` root is read-only, `worktree` is the only writable root, and the optional fixed-target `guardrail-fixture` file exists solely for explicit behavior evidence. Host paths and environment values are invocation facts, never profile or receipt fields.

## Functional core and authority order

The pure planner consumes the immutable checked profile, normalized descriptor/policy identities, active context, matching prior Animus/Basalt authorization, host-resolved root/symlink facts, admitted environment values, and exact Bubblewrap facts. It performs no filesystem, environment, process, clock, network, or kernel effects. It:

1. rejects denied or drifted authorization before planning;
2. requires every binding to match a profile label, target, kind, capability root, and non-escaping symlink observation;
3. orders ancestor targets before descendants, deduplicates compatible operations, and rejects conflicts;
4. starts from an empty private root, adds only explicit binds, keeps root read-only, and preserves only explicit writable child mounts;
5. uses a private network namespace unless `inherit` is selected;
6. clears the environment and adds only profile-admitted explicit key/value bindings; and
7. binds the deterministic plan with domain-separated BLAKE3.

Guardrail success never creates or widens authority. Existing portable sandbox policy, authorization, approval, tool admission, and confirmation boundaries remain authoritative.

## CLI and Tile/PTY integration

The thin executable accepts only explicit paths:

```console
animus-agent-guardrail describe --profile PROFILE.json
animus-agent-guardrail validate --profile PROFILE.json --request REQUEST.json --bwrap /absolute/path/to/bwrap
animus-agent-guardrail exec --profile PROFILE.json --request REQUEST.json --bwrap /absolute/path/to/bwrap -- COMMAND ARG...
```

`describe` emits safe labels and identities. `validate` reads bounded facts and computes the exact prospective plan without creating configured paths, opening a PTY, launching Bubblewrap, or running the requested child. `exec` re-observes the explicit Bubblewrap executable, requires exact expected executable/cohort identity, reruns preflight, and then launches only through sealed transport. Tile or another PTY host may supervise `animus-agent-guardrail` as its child; the guardrail does not own Tile layout, terminal state, transcript/session state, provider/tool effects, or completion.

## Sealed option transport

The Linux shell converts only typed operations to finalized Bubblewrap options, encodes them as NUL-delimited bytes, writes and rewinds an anonymous memfd, and applies shrink/grow/write/seal locks. Before creating that descriptor it audits `/proc/self/fd` and rejects every descriptor above stderr that is not already close-on-exec, preventing inherited host/control descriptors from bypassing mount or socket policy. It invokes the exact Bubblewrap 0.11.2 cohort as `bwrap --args FD -- COMMAND`. There is no finalized-option argv fallback, `PATH` discovery, implicit shell, transport switch, or unguarded retry. The descriptor is anonymous and sealed, but this is not same-UID secrecy: a same-UID host process may still inspect process memory or descriptors.

The private root hides inherited host paths. `/nix/store` is explicitly read-only, `/workspace` is explicitly writable, `/tmp` and `/dev` are private tmpfs mounts, and the private root is remounted read-only after child mounts are installed. Device and socket exposure remains denied. `network = "inherit"` restores host networking for connectivity but proves no hostname, address, HTTP, or egress filtering.

## Evidence

```console
scripts/cairn/check-agent-guardrail --root .
ANIMUS_GUARDRAIL_LINUX_LIVE=1 \
ANIMUS_GUARDRAIL_BWRAP=/absolute/path/to/bwrap \
scripts/cairn/check-agent-guardrail --root .
nix build .#checks.x86_64-linux.agent-guardrail -L
```

The ordinary rail covers Nickel freshness, malformed/escape-hatch profiles, pure deterministic plans, authorization drift, root/symlink/conflict denial, environment admission, plan drift, safe receipts, real memfd sealing, fake Bubblewrap descriptor consumption, child exits/signals, CLI redaction, and default/no-default/Wasm isolation. The opt-in Linux test additionally proves worktree writes while denying credential and sibling reads, symlink escape, outside-root writes, and host-loopback connectivity for the exact host prerequisites. If Bubblewrap or user namespaces are unavailable, live evidence is `blocked`, not a passing skip.

## Receipts and non-claims

Receipts contain only backend/cohort labels, profile/descriptor/policy/authorization/executable/plan identities, requested and enforced capability labels, network posture, root labels, safe status/issue/exit/cleanup classes, bounded option count, redaction, non-claims, and a domain-separated BLAKE3 receipt identity. They reject prompts, credentials, environment values, full host paths, mount payloads, command bodies, stdout/stderr, debug logs, and unbounded events.

Passing evidence does **not** establish hostile-code containment, kernel correctness, same-UID secrecy, filtered network egress, authorization, build hermeticity, TOCTOU-safe path resolution, Sandlock equivalence, production readiness, or release eligibility.
