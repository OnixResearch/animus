# Risk analysis

## Assets and hazards

Protected stream content, effect uniqueness, subscription capacity, and retained buffer ownership are the protected assets. Hazards include lost wakeups, stale handle reuse, repeated effects, revoked-user delivery, and false clean-shutdown claims.

## Mitigations and residual risks

The selected host owns permits, timers, current authority observations, and teardown. Existing cores retain replay and effect semantics. Generation fences, bounded drains, fixed deadlines, no-rerun tests, and explicit unresolved states mitigate the hazards. Fake services do not prove real provider cancellation, storage durability, or complete host coverage.

## Review boundary

The authoring agent reviews this plan. That review is not independent security evidence. No live action or product activation is authorized. The active universal action-receipt change remains separate.
