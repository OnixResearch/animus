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

## Non-adopted Sandlock surfaces

Animus does not vendor Sandlock, shell out to the Sandlock CLI from reusable crates, copy Sandlock profile formats as the public descriptor schema, expose Landlock or seccomp types through `animus-core`, promise cross-platform parity for Linux-only features, or treat sandbox enforcement as authorization proof.
