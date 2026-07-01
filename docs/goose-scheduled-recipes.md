# Goose scheduled recipe workflow adaptation

Animus adapts Goose scheduled recipes as policy-admitted workflow job descriptors. Reusable core validates schedule data and deterministic next-run plans only; hosts own timers, queues, process execution, persistence, and controls.

## Runtime boundary

- `ScheduledWorkflow` validates schedule identity, recipe identity/version hash, parameter hashes, cadence, calendar policy, execution mode, concurrency policy, workspace scope, retry bounds, retention policy, provenance hash, and admitted recipe trust.
- `plan_next_run` uses explicit logical clock inputs. It does not start timers, edit cron, launch processes, mutate files, or read environment state.
- `admit_scheduled_run` requires recipe trust, capability policy, sandbox policy, queue admission, cancellation handle, and current policy before dispatch.
- `evaluate_scheduled_concurrency` prevents overlapping writes from merging silently by serializing, isolating, or denying according to policy.
- `ScheduledRunRecord` stores hashes, counts, outcome labels, retry counts, and redaction markers without raw prompts, secrets, paths, or tool output.
- `ScheduleControlRequest` models pause, resume, edit, run-now, and cancel as host-mediated controls requiring admission and confirmation.

## Verification

Focused verification for this surface:

```sh
cargo test -p animus-adapters --locked
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal adapt-goose-scheduled-recipes --root .
nix run .#cairn -- gate design adapt-goose-scheduled-recipes --root .
nix run .#cairn -- gate tasks adapt-goose-scheduled-recipes --root .
```

The adapter tests include positive recurring and run-now schedules, deterministic next-run planning, run admission, concurrency policy, safe run records, and host-mediated controls plus negative invalid cadence, missing recipe, untrusted recipe, invalid planning input, missing queue service, missing cancellation handle, write conflicts, record leaks, and unauthorized controls.
