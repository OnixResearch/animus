# Ornith offline evaluation feedback loop

Animus adapts Ornith's scaffold self-improvement loop as a local, reviewed evidence workflow. It does not implement RL training, live runtime self-mutation, hidden-test access, cloud benchmark runners, provider-specific eval APIs, or public benchmark claims.

The reusable runtime surface lives in `animus-runtime::eval_feedback` and is re-exported by `animus-runtime`. Hosts own batch execution, model/provider calls, verifier execution, filesystem/process/network access, review UI, and scaffold admission.

## Plan data

Offline evaluation plans are deterministic DTOs that name a local task-set label, scaffold revision ID/hash, model route, verifier identity/hash, trust-boundary hash, tool-surface hash, sampling profile, resource labels, positive run count, and output format. Validation rejects remote task sets, missing hashes, unsupported output formats, non-positive run counts, provider-specific public benchmark claims, hidden-test access, cloud runners, live self-mutation, and RL training requests.

## Redacted receipts

Evaluation receipts expose comparison metadata only: model ID, scaffold hash, trust-boundary hash, tool-surface hash, verifier hash, runtime version label, context-window class, sampling labels, resource labels, run count, aggregate status, and redaction state. Receipt validation rejects raw prompts, hidden tests, provider payloads, credentials, full paths, verifier bodies, and unbounded tool outputs.

## Review and freshness

Feedback candidates may recommend promote, demote, revise, or investigate, but they cannot activate scaffold revisions or alter runtime selection policy directly. Approved candidates may request scaffold admission; denied, unavailable, stale, or missing review outcomes remain non-authoritative evidence.

Freshness compares model route, scaffold hash, verifier hash, trust-boundary hash, tool-surface hash, runtime version label, and sampling profile against current policy. Mismatches mark evidence stale or reduced-weight according to policy, and stale evidence cannot justify promotion by itself.

## Focused verification

Run the focused tests with:

```sh
nix develop -c cargo test -p animus-runtime eval_feedback
```

The tests cover valid local plans, remote task rejection, missing hashes, unsupported formats, non-positive run counts, unsupported scope requests, receipt leak rejection, review non-auto-acceptance, exact freshness, stale evidence, reduced-weight evidence, and live self-improvement scope rejection.
