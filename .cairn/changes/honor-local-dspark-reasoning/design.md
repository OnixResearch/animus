## Context

Endpoint and request-body admission already support explicit local reasoning disable. The higher live-session planner still creates every canonical request with enabled low reasoning. This contradiction appears only in the composed live path.

## Decisions

### Decision: Project the validated cohort setting

**Choice:** Add a read-only configuration accessor and a pure reasoning projection helper. The planner uses that helper for every initial and continuation request.

**Rationale:** The validated configuration owns provider policy. The planner must not replace it with a second hard-coded policy.

### Decision: Keep the accepted set closed

**Choice:** Map `none` to disabled canonical reasoning and `low` to the existing enabled-low request. Return a planning error for every other value.

**Rationale:** A fallback could re-enable hidden reasoning or widen provider behavior.

## Risks / Trade-offs

- Future reasoning levels require an explicit reviewed mapping.
- The fix does not weaken hidden-reasoning rejection in the stream parser.
