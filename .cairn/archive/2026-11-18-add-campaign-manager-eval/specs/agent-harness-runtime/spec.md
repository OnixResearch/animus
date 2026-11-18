# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Campaign-manager evaluation is an optional Harbor-compatible lane

r[agent_harness_runtime.campaign_manager_eval.boundary] Animus MUST provide the campaign-manager evaluation lane as the optional crate `animus-campaign-manager-eval`, outside the workspace's default members. The lane MUST implement Animus's Harbor environment and verifier ports and drive them only through `animus_runtime::harbor_eval` and `animus_std::harbor`, without Harbor's Python application, Hub, cloud, or container backends. It MUST NOT contact a model provider unless its `live` feature is built and a live route is invoked explicitly.

#### Scenario: Default builds do not carry the lane

- GIVEN the Animus workspace with its default members
- WHEN a default build is resolved
- THEN `animus-campaign-manager-eval` and its Campaign dependencies are not built.

#### Scenario: Scripted evaluation runs offline

- GIVEN a build without the `live` feature
- WHEN the evaluation binary runs without arguments
- THEN every trial runs against scripted routes with no network access
- AND asking for live mode fails with a usage error.

### Requirement: Evaluation tasks are typed Nickel revalidated in Rust

r[agent_harness_runtime.campaign_manager_eval.tasks] The lane MUST load its tasks from the checked projection of `config/campaign-manager-eval-tasks.ncl`. It MUST refuse a projection that has:
- an unknown field;
- schema drift, or a Harbor cohort other than the pinned release, revision, task schema, and ATIF schema;
- an unsafe identity or key;
- a verifier identity equal to the environment identity;
- a repeated task identity or candidate key;
- a task whose objective or label is out of bounds or holds control characters, whose signal exceeds 9, whose turns exceed 24, whose offers exceed the manager's offer bound, or that has other than exactly one target.

#### Scenario: The checked task set is admitted

- GIVEN the generated projection exported from the Nickel source
- WHEN the lane parses it
- THEN every task has exactly one target, and the mandate terms are canonical.

#### Scenario: Drift is refused

- GIVEN a projection with an unknown field, another schema, another Harbor revision, a shared verifier, an unsafe identity, a duplicate, a second target or none, a signal of 10, an 81-character label, a multi-line objective, too many turns, or too many offers
- WHEN the lane parses it
- THEN it refuses with the matching `TaskError`.

### Requirement: A trial alternates manager turns with host planner steps

r[agent_harness_runtime.campaign_manager_eval.trial] For each trial the environment MUST open the task's campaign on fresh Choregraph history and admit the task's candidates. It then runs at most the task's turns, each as:
1. one manager turn with the trial's route, offering every unadded offer and labelling every admitted candidate, with `[signal N]` appended once it has run;
2. one host planner step.

In a host planner step, a selection MUST be published, released by the publication fence, run against its hidden outcome, and settled by a published result. A wait MUST publish nothing, and the planner's stop MUST be published. The trial MUST end when the campaign stops.

A manager reply MUST be classified as admitted, unchanged (`UnchangedState`), refused, or a model failure.

#### Scenario: A manager reaches an offered target

- GIVEN the parser task, whose target is an offer
- WHEN a trial runs the signal-greedy route
- THEN the manager adds offers, the target's run records the finding, and the planner's policy stop ends the campaign.

#### Scenario: A manager that never adds offers misses it

- GIVEN the same task
- WHEN a trial runs the explore-only route
- THEN no offer is added, no finding is recorded, and the budget stop ends the campaign.

#### Scenario: Unknown routes and tasks fail closed

- GIVEN a trial request that names an unknown route or an unknown task
- WHEN the environment executes it
- THEN it returns `Unavailable` or `Denied` and leaves no artifact.

#### Scenario: An unreachable model completes with failed turns

- GIVEN a route whose every call fails
- WHEN a trial runs
- THEN the trial completes with every turn recorded as a model failure.

### Requirement: The separate verifier grades Campaign's replay

r[agent_harness_runtime.campaign_manager_eval.verifier] The verifier MUST reload the trial's control projection from its recorded Choregraph graph, branch, payloads, and envelope. It MUST refuse:
- a missing artifact as `Stale`;
- a history that does not load as `Malformed`;
- a replayed final state that differs from the environment's claim as `Stale`;
- an unknown task as `Denied`.

It MUST report:
- `found` as 1 only when the replayed findings hold the task's finding and the target's candidate was selected;
- `efficiency` as `(budget + 1 - selections) / budget` for a found target, and 0 otherwise;
- `refusal-rate` as the share of refused or failed turns in the environment's ledger, which jobs mark advisory.

#### Scenario: A tampered claim is refused

- GIVEN a completed trial whose artifact claims another final state
- WHEN the verifier runs
- THEN it refuses the trial as `Stale`.

#### Scenario: A foreign history is refused

- GIVEN an artifact whose branch head belongs to another trial's history
- WHEN the verifier runs
- THEN it refuses the trial as `Malformed`.

### Requirement: Trials project validated ATIF trajectories

r[agent_harness_runtime.campaign_manager_eval.trajectory] Each trial MUST project an ATIF-v1.7 trajectory through `project_atif`. Each manager turn is an agent step with one correlated `campaign.advise` call. Each host step is a system step with one correlated `campaign.execute`, `campaign.wait`, or `campaign.stop` call, and the last step is terminal. The projection MUST carry no prompt, reply, or label text.

#### Scenario: A trial's steps project

- GIVEN a finished trial
- WHEN its trajectory is projected
- THEN it has one step per manager turn plus one per host step, each with one call.

#### Scenario: An empty trial does not project

- GIVEN no turns
- WHEN a trajectory is projected
- THEN projection refuses with `Bound`.

### Requirement: Jobs run the task × route matrix through the Harbor slice

r[agent_harness_runtime.campaign_manager_eval.job] A job MUST:
1. plan every task against every route with `plan_trials`;
2. run each trial through `execute_separate_verifier`;
3. report every trial that does not complete;
4. reduce each route's verifier rewards with `reduce_rewards`;
5. build the metadata-only receipt with `receipt` from the completed trials' trajectory projections.

#### Scenario: The job separates managers

- GIVEN the scripted routes
- WHEN the job runs every task
- THEN every trial completes, signal-greedy finds every target with no refusals, explore-only finds fewer, and malformed finds none with every turn refused.

### Requirement: The evaluation report is metadata only

r[agent_harness_runtime.campaign_manager_eval.report] The evaluation binary's report MUST hold only the job and profile identities, the planned, completed, and failed counts, per-route completion counts and mean rewards with advisory markers, failure classes, the trajectory count, and the non-claims. It MUST never hold prompts, replies, labels, or credentials. Unsupported arguments MUST fail without a report.

#### Scenario: The scripted report leaks nothing

- GIVEN the scripted job
- WHEN the binary reports it
- THEN no objective, label, signal annotation, or reply text appears in the report.

### Requirement: The live route is opt-in and reviewed

r[agent_harness_runtime.campaign_manager_eval.live] Under the `live` feature, the lane MUST offer a route to the reviewed `OpenAI` or `OpenRouter` Responses endpoint with an explicit model and credential. Each turn is one text-only request planned through `plan_provider_dispatch`. The route MUST refuse an inadmissible credential or model before contacting the provider. Its route identity MUST be a safe identity derived from the endpoint and model.

#### Scenario: Inadmissible live routes are refused offline

- GIVEN an empty or multi-word credential, or an empty model
- WHEN a live route is constructed
- THEN it refuses with `Credential` or `Model` without contacting the provider.

### Requirement: Campaign-manager evaluation verification

r[agent_harness_runtime.campaign_manager_eval.verification] The change MUST verify the lane with positive and negative tests for its task set, trials, verifier, trajectories, jobs, report, and live-route construction. The focused checks `cargo-campaign-manager-eval` and `campaign-manager-eval-octet` MUST pass, and the generated task set MUST pass `check-nickel-configs`.

#### Scenario: Focused rails pass

- GIVEN the change's crate, config, and tests
- WHEN the focused checks and the Nickel config rail run
- THEN all succeed.

#### Scenario: A regression is caught

- GIVEN a lane that graded environment claims instead of the replay, leaked prompt text, or counted no-op replies as refusals
- WHEN the focused tests run
- THEN at least one test fails.
