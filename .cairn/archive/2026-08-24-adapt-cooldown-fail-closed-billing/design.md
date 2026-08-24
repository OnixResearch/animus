## Context

Animus already owns typed provider failure classification, bounded same-plan retry, injected timers, provider-neutral progress, and usage-accounting contracts. These surfaces do not yet bind account cooldown and billing-tier transitions into one fail-closed decision.

`digitallyinduced/haskell-agent` treats account cooldown and billing policy as runtime concerns. It preserves the pending turn across provider transitions and refuses silent subscription-to-credit conversion.

## Goals

- Skip providers that are in account cooldown during automatic failover.
- Make every automatic billing-tier change visible before admission.
- Preserve the pending turn and durable session state across provider transitions.
- Keep selection and transition decisions pure over caller-supplied facts.

## Non-goals

- Provider account discovery, billing reconciliation, or credential storage.
- A universal provider error taxonomy.
- Claims that a provider honors the declared billing tier.
- Automatic consent for pay-per-use spending.

## Decisions

### Decision: Cooldown blocks automatic selection

**Choice:** The provider-retry core consumes explicit cooldown facts and removes cooldown providers from automatic candidates. The shell owns clocks, account probes, and refresh.

**Rationale:** A retry cannot succeed safely when the selected account is known to reject work. Keeping cooldown facts explicit avoids ambient provider inspection inside the core.

### Decision: Billing-tier changes fail closed

**Choice:** An automatic transition from subscription use to pay-per-use credit requires an explicit tier-change record before request admission. Missing or unknown tier facts block the transition.

**Rationale:** Silent fallback can turn a subscription failure into unexpected spending. Visibility after the request is too late.

### Decision: Provider transitions preserve the pending turn

**Choice:** A transition plan binds one pending-turn identity, one durable session identity, the old target, the new target, and the selected billing tier. The shell commits the plan before it contacts the new provider.

**Rationale:** Provider changes are transport changes. They must not reset or duplicate the user turn.

## Dependency Direction

```text
provider shell and account probes
              ^
              |
provider-retry core -- usage-accounting contracts -- session transition plan
```

The core receives supplied cooldown and tier facts. It does not read clocks, credentials, provider APIs, or billing systems.

## Adversarial Audit

The audit must try to show that Animus:

- automatically selects a provider that is in cooldown;
- changes from subscription use to pay-per-use credit without a prior record;
- treats an unknown billing tier as permission;
- drops, duplicates, or changes the pending turn during transition;
- claims provider billing truth from caller-supplied metadata.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Fail-closed billing policy can reduce availability. That is preferable to unapproved spend.
- Cooldown facts can become stale. The shell must refresh them and bind their observed generation.
- Some transports cannot continue a provider-native response identifier. The transition preserves the Animus turn, not provider-private continuation semantics.

## Non-Claims

This change does not prove provider availability, billing accuracy, account ownership, credential validity, request success, or exact cross-provider semantic continuity.
