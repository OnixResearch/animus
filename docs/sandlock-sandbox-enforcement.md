# Sandlock-inspired sandbox enforcement

Animus uses Sandlock as a bounded Linux reference for sandbox-enforcement patterns. The implementation remains an Animus-owned contract: reusable crates validate portable descriptors and safe receipts, while host shells own kernel probing, filesystem staging, process launch, Landlock/seccomp integration, network/HTTP enforcement, and platform errors.

## Portable descriptor boundary

`SandboxDescriptor` captures execution-constraint intent without Linux kernel details:

- filesystem read/write posture and safe writable-root labels;
- workdir materialization and COW write disposition;
- process/resource supervision profile;
- environment visibility policy;
- network egress/bind policy;
- optional HTTP method/host/path ACL intent;
- deterministic time/randomness controls; and
- whether enforcement is required before dispatch.

Descriptor normalization is pure deterministic logic over typed inputs. It rejects missing fields, duplicate roots, unknown labels, COW settings without COW workdir staging, and HTTP ACLs combined with deny-all networking before any platform adapter can run.

## Policy and platform checks

`SandboxPolicy` is a data fixture boundary for Nickel-authored policy outputs. It lists allowed constraint classes such as filesystem, workdir/COW, process, environment, network, HTTP, and determinism. `evaluate_sandbox_dispatch` denies dispatch when a normalized descriptor asks for a class that policy does not allow.

`SandboxPlatformSupport` models host capabilities inspired by Sandlock: Landlock-style filesystem confinement, seccomp-bpf, seccomp user notification, COW staging, resource limits, network ACLs, HTTP ACLs, and determinism controls. Required descriptors fail closed when any needed platform capability is absent; best-effort descriptors still record partial-enforcement evidence.

## Receipts

`SandboxDispatchReceipt` stores safe metadata only: status, issue code, descriptor hash, platform capability labels, unsupported capability labels, bounded COW summary counts, optional output hash, and receipt hash. Receipt validation rejects raw prompts, credentials, absolute path dumps, raw network payloads, raw file contents, raw process output, and unbounded event streams.

## Optional Linux host adapter

The non-default `animus-std/sandlock-host` feature binds `sandlock-core` 0.7.0 to exact reviewed revision `e8c4cc0ef58e3fc145c4d147a61b889b4f291a10`. The upstream cohort is Apache-2.0 licensed. The Cargo dependency is scoped under `cfg(target_os = "linux")`; default and non-Linux dependency graphs do not include Sandlock.

The adapter uses a functional core and imperative shell:

- `plan_sandlock_launch` purely validates a current matching Animus/Basalt authorization result, safe confined root bindings, exact policy and command identities, requested mechanism classes, and a deterministic BLAKE3 plan identity;
- `evaluate_sandlock_preflight` accepts only an exact-plan, exact-cohort functional probe covering every requested filesystem, syscall, supervisor, COW, resource, network, HTTP, environment, and determinism mechanism;
- the Linux shell canonicalizes explicit capability roots, builds the typed upstream `Policy`, runs one host-profile-owned functional command for every requested mechanism class under the exact target policy, and only then launches the exact command bound by the authorization and plan; missing, duplicate, failed, or partial mechanism probes block launch;
- the initial COW profile supports auditable dry-run discard only: bounded before/after workdir manifests must match after upstream cleanup; commit is rejected because the reviewed seccomp-COW API does not expose a fallible commit/abort completion result that Animus can bind safely;
- stdout and stderr are drained through bounded pipes and retained only as byte counts and incremental BLAKE3 identities; timeout, cancellation, output exhaustion, nonzero exit, COW disposition, and cleanup remain distinct facts; and
- receipt projection recomputes its identity and rejects passing evidence unless every requested capability was enforced and cleanup completed.

No root path, command body, prompt, credential, environment value, process output body, network payload, file content, or Sandlock debug log is persisted in the receipt. Concrete host paths exist only in the authority-scoped shell and are never part of the portable launch plan.

### Operator prerequisites and upgrade process

Live operation requires Linux with Sandlock's reviewed minimum Landlock ABI, seccomp-bpf and seccomp user notification enabled, and permission to use the required unprivileged mechanisms. Hosts must supply explicit existing capability roots and a bounded probe executable; the adapter does not discover roots, commands, credentials, or fallback sandboxes.

To review an upgrade:

1. inspect the upstream license, source diff, public `sandlock-core` policy/process APIs, cleanup behavior, and minimum kernel requirements;
2. update the exact Cargo revision rather than a branch or floating version;
3. let Cargo regenerate `Cargo.lock`; never edit the lockfile manually;
4. rerun dependency-isolation, pure positive/negative, fake-host, receipt-safety, Clippy, and focused Nix checks; and
5. run the explicit real-kernel rail on a reviewed host before promoting live behavior evidence.

The deterministic focused rail is:

```sh
scripts/cairn/check-sandlock-host --root .
```

Real filesystem denial, loopback network denial, COW discard, resource-process denial, and clean-environment behavior are opt-in:

```sh
ANIMUS_SANDLOCK_LINUX_LIVE=1 scripts/cairn/check-sandlock-host --root .
```

When that variable is absent, the receipt records the real-kernel tier as blocked with its prerequisites; it is never reported as a passing skip.

## Authority and non-claims

Animus/Basalt authorization must already allow the exact action, subject, target, descriptor, policy, and command identities before root resolution, mechanism probing, or target launch. Sandlock success cannot mint, widen, refresh, or replace authorization. A successful receipt is not authorization, not proof of kernel correctness or complete containment, not build hermeticity, not cross-platform parity, and not production or release eligibility. `cleanup=complete` is bounded to child termination/reaping and observed COW workdir restoration; it does not prove cleanup of upstream-private temporary storage that the reviewed Sandlock API does not expose.

## Non-adopted Sandlock surfaces

Animus does not vendor Sandlock, shell out to the Sandlock CLI from reusable crates, copy Sandlock profile formats as the public descriptor schema, expose Landlock or seccomp types through `animus-core`, promise cross-platform parity for Linux-only features, or treat sandbox enforcement as authorization proof. The initial adapter also excludes upstream profile loading, dynamic policy callbacks, credential injection, GPU access, port remapping, OCI behavior, and COW fork/map-reduce.
