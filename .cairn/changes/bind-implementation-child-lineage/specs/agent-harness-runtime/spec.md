## ADDED Requirements

### Requirement: Exact implementation child lineage

r[animus.implementation_child_lineage] Animus must export bounded metadata-only child lineage from the selected implementation parent’s own lifecycle events.

#### Scenario: Exact child receipt

- GIVEN an explicitly selected attempt and a genuine child lifecycle observation
- WHEN the owner exports lineage for the exact bound request
- THEN the authorized consumer can display the parent and child without private content

#### Scenario: Crossed or missing receipt

- GIVEN a missing or mismatched observation
- WHEN the consumer reads lineage
- THEN it must not report a child fact from that observation
