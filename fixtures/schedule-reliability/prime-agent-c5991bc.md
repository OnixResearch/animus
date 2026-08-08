# Prime Agent schedule reliability cohort

- Revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- License: MIT
- Selected surfaces: `/heartbeat`, `rlm_heartbeat`, and `prime-agent schedule`
- Selected mechanisms: claim a due tick before delivery and coalesce missed ticks into one catch-up decision.
- Rejected behavior: ambient timers, direct persistence, cross-session jobs, agent mutation of the user heartbeat, and unbounded missed-tick replay.

This fixture records reviewed semantics only. It does not include Prime Agent code, prompt bodies, live schedules, or execution evidence.
