# Clankers cost-aware model routing

Animus cost-aware routing is a pure policy contract over host-supplied task signals, usage summaries, and safe model catalog projections. It does not contact providers or perform live health checks.

## Policy

Routing policy is authored as Nickel in `policy/model-routing/policy.ncl` and validated in Rust checked fixtures. It defines role labels, score thresholds, complexity weights, downgrade targets, missing-price behavior, budget behavior, and policy hash.

Policy validation rejects duplicate roles, negative weights, unordered thresholds, unknown downgrade targets, unspecified missing-price behavior, empty roles, zero schema versions, and unordered budgets.

## Inputs

Routing consumes:

- `RoutingTaskSignals`: bucketed token, file, tool, and risk signals plus an optional safe user role hint
- `RoutingUsage`: current cost, soft budget, and hard budget units
- `RoutingModelCatalog`: injected safe model labels, role labels, capability class, optional price units, availability, and policy tags
- `ModelRoutingPolicy`: validated policy metadata and BLAKE3 policy identity

## Decisions

`route_model` computes a complexity score, maps it into a simple/medium/complex score bucket, selects a policy role, applies soft/hard budget behavior, filters injected catalog candidates, and returns `ModelRoutingDecision`.

Soft budget pressure can downgrade to a cheaper role. Hard budget pressure downgrades if policy provides a cheaper role, otherwise routing is denied. Missing prices are denied when the active policy requires pricing metadata. Unavailable models do not trigger provider fallback.

Every decision records `provider_request_sent = false` because routing is pure and side-effect-free.

## Receipts

`ModelRoutingReceipt::validate` records safe decision status, selected role/model label, score bucket, budget status, candidate count, policy hash, fallback class, and deterministic BLAKE3 receipt hash.

Receipt validation rejects raw prompts, provider payloads, credentials, full pricing-store dumps, and hidden reasoning text.

## Non-claims

This contract does not implement provider routing, OAuth, live provider health checks, provider discovery, price scraping, Clankers role names, or concrete provider calls.
