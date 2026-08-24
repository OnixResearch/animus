# Adapt Cooldown Fail Closed Billing Specification

## Purpose

Defines the `adapt-cooldown-fail-closed-billing` capability.

## Requirements

### Requirement: Automatic failover is cooldown-aware

r[animus.billing.failover] Automatic provider failover MUST NOT select a provider whose account is in cooldown. A provider in cooldown MUST be recorded as unavailable until a current shell observation clears the cooldown.

#### Scenario: Cooldown provider is skipped

r[animus.billing.failover.cooldown_skip]
- GIVEN the preferred provider account is in cooldown
- WHEN automatic failover selects a provider
- THEN failover MUST skip the cooldown provider
- AND MUST record the skip reason

#### Scenario: Cooldown provider selection fails

r[animus.billing.failover.cooldown_selected]
- GIVEN an implementation selects a cooldown provider automatically
- WHEN the failover record is inspected
- THEN the inspection MUST report the invalid selection

### Requirement: Billing-tier conversion is visible before admission

r[animus.billing.tier] Automatic fallback MUST NOT silently convert subscription usage into pay-per-use API-credit spending. A tier change MUST be visible in the turn or session record before the fallback request is admitted.

#### Scenario: Tier change is recorded

r[animus.billing.tier.recorded]
- GIVEN a fallback changes the billing tier from subscription to pay-per-use
- WHEN the request is admitted
- THEN the turn or session record MUST name the new tier
- AND MUST name the reason for the change

#### Scenario: Silent tier conversion fails

r[animus.billing.tier.silent]
- GIVEN a fallback changes tier with no recorded reason
- WHEN the request is admitted
- THEN admission MUST fail and report the silent conversion

### Requirement: Provider transition preserves the pending turn

r[animus.billing.pending] A provider transition MUST preserve the pending turn and durable session state. The transition MUST NOT drop, duplicate, or corrupt the pending turn.

#### Scenario: Pending turn survives transition

r[animus.billing.pending.survives]
- GIVEN a provider transition occurs during a pending turn
- WHEN the new provider continues the turn
- THEN the pending turn MUST continue without loss
- AND the durable session state MUST remain intact

#### Scenario: Pending turn loss fails

r[animus.billing.pending.loss]
- GIVEN a provider transition drops or duplicates the pending turn
- WHEN the session is inspected
- THEN the inspection MUST report the transition defect

### Requirement: Billing policy verification is complete

r[animus.billing.verification] Positive, negative, and boundary fixtures MUST cover cooldown-aware failover, visible billing-tier conversion, and pending-turn preservation.

#### Scenario: Billing policy matrix passes

r[animus.billing.verification.matrix]
- GIVEN the billing rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming inputs MUST behave as declared
- AND every cooldown-selection, silent-tier, or turn-loss input MUST fail as declared
