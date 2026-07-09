# Proposal: Adapt Goose remote server admission

## Why

Goose supports driving a Desktop UI through a remote `goose serve` backend with TLS, a shared secret, and certificate fingerprint pinning. Animus already has remote admission receipts, headless protocol, host-control envelopes, provider/tool injection, and safe transport boundaries, but it lacks a concrete contract for admitting remote runtime backends and recording TLS/auth handshake evidence safely.

Animus should adapt the useful remote-backend ergonomics as data-only descriptors, host-supplied handshake summaries, admission decisions, and safe receipts.

## What Changes

- Define remote backend descriptors for endpoint identity, transport kind, TLS requirement, certificate fingerprint or trust-on-first-use posture, secret source label, platform role, protocol role, capability labels, and provenance.
- Add pure admission validation over descriptors, host-supplied handshake summaries, remote-admission policy, credential-source availability, certificate pin state, scope, and capability ceilings.
- Add typed failure classes for missing TLS, fingerprint mismatch, trust-on-first-use pending, secret unavailable, unauthorized, unsupported protocol, stale pin, scope denial, and remote fallback denial.
- Require safe receipts for descriptor validation, connection admission, TLS pinning, auth result, denial, and unavailable-backend outcomes.

## Impact

- **Files**: future std/headless host DTOs, remote admission helpers, receipt validation, docs, tests, and Cairn artifacts.
- **Testing**: positive pinned-TLS and trust-on-first-use admissions plus negative HTTP, fingerprint mismatch, secret-value leak, wrong audience, missing credential, fallback, and receipt leak fixtures.
- **Non-claims**: this does not implement a TCP server, ACP transport, TLS stack, socket client, certificate store, background daemon, Desktop setting, or credential persistence from reusable crates.
