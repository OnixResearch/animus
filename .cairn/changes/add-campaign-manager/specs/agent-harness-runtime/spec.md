# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Campaign manager is an optional effect-free role

r[agent_harness_runtime.campaign_manager.boundary] Animus MUST provide the campaign manager as the optional crate `animus-campaign-manager`, outside the workspace's default members. The manager reaches the model only through `animus_adapters::ModelAdapter` and durability only through a host-supplied `DurableControlStore`, and it performs no filesystem, process, network, clock, environment, or persistence effect of its own. Campaign's review MUST remain the only authority that turns advice into control events.

#### Scenario: Default builds do not carry the manager

- GIVEN the Animus workspace with its default members
- WHEN a default build or default package set is resolved
- THEN `animus-campaign-manager` and its Campaign dependencies are not built
- AND the default license and dependency footprint are unchanged.

#### Scenario: The manager acts only through supplied ports

- GIVEN a host supplies a frontier state, a manager setup, a candidate menu, a model adapter, and a durable store
- WHEN the manager advises and publishes
- THEN every model call goes through the supplied model adapter and every durable write goes through the supplied store
- AND the manager itself performs no other effect.

### Requirement: The rendered brief presents handles, never identities

r[agent_harness_runtime.campaign_manager.brief] When rendering Campaign's advisory brief for a model, the manager MUST present:
- the objective and the reply schema;
- the rank dimension count and magnitude bound;
- each action the mandate grants with its per-reply limit, and each action it refuses;
- the campaign counters and remaining budgets;
- each listed candidate under the handle `c<n>` in brief order, with its host label, latest ranks, and selection count;
- each host offer under the handle `o<n>` in menu order.

The rendered text MUST NOT contain a Campaign identity, and it MUST fit the model request contract.

#### Scenario: Listed candidates and offers appear under handles with labels

- GIVEN a campaign with listed candidates, host labels for some of them, and host candidate offers
- WHEN the manager renders the brief
- THEN each candidate appears under its `c<n>` handle with its label, or `-` without one, and each offer appears under its `o<n>` handle with its label
- AND no candidate, campaign, or brief identity appears in the text.

#### Scenario: Invalid setup text is refused before rendering

- GIVEN an objective or label that is empty, longer than its bound, or holds a control character, or more offers than the offer bound
- WHEN the manager renders the brief
- THEN it refuses with `InvalidText` naming the field, or with `TooManyOffers`
- AND no model request is produced.

### Requirement: Replies are one JSON object in the declared schema

r[agent_harness_runtime.campaign_manager.reply] The manager MUST parse a model reply as the one JSON object between its first `{` and its last `}`. The object's fields are `scores`, `add`, `retire`, `prune`, and `stop`, each optional. Ranks MUST be integers. The manager MUST refuse with `MalformedReply` a reply without such an object, with an unknown field, with a wrong field type, or with a non-integer rank.

#### Scenario: A fenced reply parses

- GIVEN a reply that wraps a schema-conforming object in prose and a code fence
- WHEN the manager parses it
- THEN it returns the object's scores, additions, retirements, prune, and stop.

#### Scenario: A malformed reply is refused

- GIVEN a reply with no JSON object, an unknown field, a string where an object belongs, or a fractional rank
- WHEN the manager parses it
- THEN it refuses with `MalformedReply`.

### Requirement: Replies map to one advisory proposal bound to its brief

r[agent_harness_runtime.campaign_manager.proposal] The manager MUST build one `AdvisoryProposal` bound to the brief's campaign, state, brief, mandate, and advisor identities:
- A score of a candidate without guidance becomes generation-1 guidance.
- A score lexicographically greater than the candidate's latest ranks becomes guidance at the next generation.
- A lower score becomes a revision of the latest guidance with the advisor-scoped reason `advisor-rescore`.
- An equal score adds nothing.
- Each added offer becomes a member of the candidate batch, with generation-1 guidance when the reply scores it.
- Retirements, the prune, and the stop carry `Reason` identities derived from the advisor identity and the reply's reason label.
- The proposal's source facts MUST be the BLAKE3 identity of the whole reply text.

#### Scenario: Scores, offers, and removals become proposal actions

- GIVEN a reply that scores listed candidates, adds an offer with a score, retires one candidate, prunes others, and requests a stop
- WHEN the manager builds the proposal and Campaign reviews it
- THEN the admitted events are the candidate batch, the guidance records, the revisions, the retirements, the prune, and the stop, in Campaign's review order.

#### Scenario: Rescoring raises, lowers, and repeats

- GIVEN candidates with guidance and a reply that scores one higher, one lower, and one equal to its latest ranks
- WHEN the manager builds the proposal
- THEN the higher score is next-generation guidance, the lower score is a revision, and the equal score adds nothing.

#### Scenario: A reply that names nothing presented is refused

- GIVEN a reply that names a handle beyond the brief or the menu, or a non-canonical handle such as `c01`
- WHEN the manager builds the proposal
- THEN it refuses with `UnknownHandle`
- AND it never submits a proposal for that reply.

#### Scenario: A score outside the rank domain is refused

- GIVEN a reply whose ranks have another count than the presented dimensions or exceed the presented magnitude
- WHEN the manager builds the proposal
- THEN it refuses with `RankLength` or `RankMagnitude` naming the handle.

### Requirement: One turn calls the model once and submits to Campaign's review

r[agent_harness_runtime.campaign_manager.turn] One manager turn MUST refuse a stopped campaign before any model call. It then briefs the campaign under the mandate, renders the brief, calls the model exactly once, builds the proposal from the reply, and submits it to `review_advisory_proposal` under the setup's admission rule and coalescing policy. It returns the proposal with Campaign's admission.

#### Scenario: Admitted advice steers the planner

- GIVEN unscored candidates and a reply that scores one best
- WHEN a turn runs and its admitted events are published
- THEN the published state equals the admission receipt's after-state
- AND Campaign's ranked planner selects the candidate the reply scored best.

#### Scenario: A model failure ends the turn

- GIVEN a model adapter that fails
- WHEN a turn runs
- THEN the turn returns the model's error
- AND no proposal is reviewed.

### Requirement: Publication is ordered, durable, and reports its prefix

r[agent_harness_runtime.campaign_manager.publication] The manager MUST publish admitted events in order. For each event it plans the publication through the session, obtains durable-publication evidence from the host store, and commits with that evidence. When the store or the session fails, the manager MUST stop and return the session after the last commit, the number of committed events, and the cause.

#### Scenario: Every admitted event is committed

- GIVEN an admission with several events and a store whose writes are durable
- WHEN the manager publishes them
- THEN the store observes one write per event
- AND the session's state equals the receipt's after-state.

#### Scenario: A store failure keeps the committed prefix

- GIVEN an admission with two events and a store that fails its second write
- WHEN the manager publishes them
- THEN it returns a failure with one committed event, the store's cause, and a session holding that event
- AND publishing the remaining event later reaches the receipt's after-state.

### Requirement: Refused advice publishes nothing

r[agent_harness_runtime.campaign_manager.refusals] The manager MUST return a typed refusal and publish nothing for:
- a stopped campaign;
- an invalid setup or menu;
- a rank dimension count that is zero or above the campaign's bound;
- a malformed reply;
- an unknown or non-canonical handle;
- an out-of-domain rank;
- an invalid reason label;
- any proposal Campaign's review refuses, such as ungranted advice or advice over a grant limit.

It MUST refuse a stopped campaign and an invalid setup before calling the model.

#### Scenario: Ungranted advice is refused by Campaign

- GIVEN a mandate that grants guidance only and a reply that requests a stop
- WHEN a turn runs
- THEN it refuses with Campaign's `UngrantedAdvice` for the stop action.

#### Scenario: A stopped campaign is never shown to the model

- GIVEN a campaign whose stop was accepted
- WHEN a turn runs
- THEN it refuses with Campaign's `CampaignStopped`
- AND the model is never called.

### Requirement: Campaign-manager verification

r[agent_harness_runtime.campaign_manager.verification] The change MUST verify the manager with positive and negative integration tests over real Choregraph history and a scripted model. The focused Nix checks `cargo-campaign-manager` (tests and Clippy with `-D warnings`) and `campaign-manager-octet` (Octet deny-all) MUST pass.

#### Scenario: Focused rails pass

- GIVEN the change's crate and tests
- WHEN `nix build .#checks.x86_64-linux.cargo-campaign-manager` and `nix build .#checks.x86_64-linux.campaign-manager-octet` run
- THEN both succeed.

#### Scenario: A regression is caught

- GIVEN the manager published unadmitted events, showed a Campaign identity, or called the model for a stopped campaign
- WHEN the focused tests run
- THEN at least one test fails.
