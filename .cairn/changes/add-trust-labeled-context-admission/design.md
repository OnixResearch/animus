# Design: Trust-labeled context admission

## Existing boundary

`animus_runtime::request_context` remains the pure reducer. Existing source owners still load and admit source data before reduction.

This change adds a trust-fencing stage between contribution validation and final prompt assembly.

## Trust facts

Each protected external contribution carries:

- origin class and origin identity
- source-owner identity
- trust tier
- screening profile and observation identity
- screening disposition
- quarantine state
- observation freshness class
- redaction and sensitivity classes
- exact payload identity
- policy and authority decision identities
- explicit non-claims

Closed initial trust tiers are `authored`, `collaborative`, `external`, and `unknown`.

Closed screening dispositions are `not-screened`, `clear-observation`, `flagged-observation`, `classifier-unavailable`, and `screening-failed`.

A `clear-observation` value does not mean safe content. A `flagged-observation` value does not prove malicious content.

## Pure admission

The pure stage receives a contribution, trust facts, context-use class, policy facts, and an optional Basalt-compatible decision.

It returns one of these results:

- admitted with retained fencing metadata
- omitted by policy
- quarantined
- review required
- denied due to missing or stale facts

Quarantined content cannot enter a model payload. Unknown or stale protected context fails closed unless an exact policy permits a narrower review-only projection.

## Policy composition

Basalt owns the content-context policy decision. Animus validates exact subject, action, resource, contribution, payload, policy, and decision bindings.

A Basalt allow admits only the context-read action. It does not grant model, tool, filesystem, process, network, memory, retention, or workflow authority.

Missing, denied, stale, mismatched, or unavailable policy decisions block protected context before payload loading or provider dispatch.

## Host boundaries

Hosts retain:

- catalog queries
- content loading
- classifier execution
- quarantine storage
- policy loading
- UCAN verification and currentness
- provider calls
- durable payload storage

The pure reducer reads no file, database, clock, environment, network, credential, model, or classifier.

## Projection and snapshots

Contribution selection retains trust facts and policy identity. Final payload validation binds each loaded body to the selected contribution and fencing decision.

`ModelRequestSnapshot` records safe trust classes, observation identities, quarantine disposition, policy identity, and payload BLAKE3. It stores no raw external content.

## Receipts

Receipts contain only safe labels, hashes, counts, decisions, freshness classes, redaction classes, and non-claims.

Receipt validation rejects raw prompt bodies, indexed records, email text, message text, document text, classifier payloads, credentials, transport headers, and host paths.

## Verification

Positive tests cover authored, collaborative, policy-admitted external, and review-only context.

Negative tests cover missing trust facts, unknown tier, quarantine, flagged content, stale observations, unavailable classifiers, mismatched Basalt decisions, policy denial, payload mismatch, and receipt leaks.

## Claim boundary

Passing admission proves only that supplied metadata met current policy. It does not prove content safety, semantic truth, classifier correctness, provider compliance, or task correctness.
