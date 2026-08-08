# Evidence: native evaluation composition conformance

## Scope

- Change: `add-evaluation-composition-conformance`
- Suite crate: `animus-eval-conformance`
- Root feature: `evaluation-conformance`, non-default
- Matrix: 11 mechanism families with 11 positive and 11 expected-negative fixtures
- Effect posture: finite injected fakes only

## Prerequisites and baseline

- `add-native-evaluation-core`: accepted and archived.
- `add-evaluation-adapter-family`: accepted and archived.
- `add-std-evaluation-host`: accepted and archived.
- Pre-change native core, evaluation adapter, standard host, Harbor, and composition rails: PASS.

## Architecture search

The search used three serial lenses. The lenses are correlated because one agent performed them.

| Family | Mechanism | Result | Evidence or blocker |
|---|---|---|---|
| Existing runtime rail | Extend `animus-runtime` composition checks with host execution | Rejected | A reusable runtime dependency on the standard host reverses the authority direction. The facade validator detects this class. |
| Root integration tests | Put the complete matrix in the compatibility crate | Rejected | This mixes fixture policy with the root facade and gives no focused profile or reusable summary owner. |
| Dedicated conformance crate | Use a non-default host-injection surface with checked metadata and public API calls | Selected | The crate remains outside default members, accepts injected fakes, and can depend on core, adapter, host, compatibility, and downstream contracts without changing their owners. |

Adversarial audit found one initial classification defect. The facade validator rejected a reusable conformance export that depended on the host shell. The final inventory classifies the suite as an injected host surface.

## Reviewed source cohort

- Repository: `PrimeIntellect-ai/verifiers`
- Revision: `1a997814e85a2e4f409add960aa852737388dbd8`
- License: MIT
- Ordered cohort identity: `b3:c6336b188596db9b18cb2f0a721c9f5bc551e619342e89807e20ec45f3bb676f`
- Scope: evaluation runner, resume, episode, interception, rollout, limiter, state, and artifact modules already reviewed for the standard host.

The cohort supplies scenario inspiration only. The suite does not load Verifiers, Python, or upstream traces.

## Implemented evidence

- Typed Nickel profile and strict generated JSON revalidation: PASS.
- Domain-separated profile, family, fixture, and summary BLAKE3 identities: PASS.
- Exact public entrypoint and fake service catalogs: PASS.
- All five built-in recipes through `execute_evaluation_host`: PASS.
- Taskset, recipe, episode, port, gateway, verifier, reward, persistence, compatibility, downstream, and evidence families: PASS.
- Unknown profile fields, live dependencies, private evidence, and overclaims: rejected.
- Missing services, unsafe gateway posture, hostile artifacts, malformed rewards, stale or torn resume evidence, cleanup failure, compatibility loss, and private dataset export: rejected.
- Static duplicate-implementation and private-path guards: PASS.
- Component catalog and runtime facade inventory: PASS.
- Focused suite tests and strict Clippy: PASS.
- Complete connected rail, including native core, adapter family, standard host, Harbor, composition, catalog, facade, Nickel, and evidence checks: PASS.
- `checks.x86_64-linux.evaluation-composition-conformance`: PASS after the configured remote builder was unavailable and Nix built locally.
- Cairn validation: PASS with 7 active changes and no issues.
- Proposal gate receipt: `628a9d316c0617d03d254dafab7eeadf4d8c464ee99de0610850a53cf3e0b222`.
- Design gate receipt: `16eda550b71f5a158009895cace49098984bab24a98982613e411f64b3a091f6`.
- Tasks gate receipt before final archive review: `5b41314148bb6d752d26751e6d5f69870d6bc0f94971cfe4d33143e681fbdc16`, with 25 of 27 tasks complete.
- Sync dry-run receipt: `fbd2b53b9443dce397c05f617894ffa895622385763621473350a0d0df2411e6`.
- The sync plan adds all six conformance requirements and changes the accepted spec to `4decc6678777f104e0f07e9e9c34f1fb89e0bcec622fa672089814efe412ff0f`.
- Initial archive dry-run receipt: `9ec0cc008762e581db2cf9e8dec60d0656e86962a01b16e79ee6eddcd95939e3`.
- The initial archive review correctly blocked while final review task D6 remained open.
- Final tasks gate receipt: `37d55ed993e1cbd4b3994617721dbf76b33962fd9bd33bb9dd9f35d4b3520d21`, with 27 of 27 tasks complete.
- Final sync dry-run receipt: `a1d1ea17fbe353253dde117c2a787cb018549924a551ddb6303e87535acc508d`.
- Final archive dry-run receipt: `9e10347b0f30060d240ea8936706592f05233b7c007d71e0b5ea2fce4dbd9727`.
- Sync execution receipt: `dd197aa05797c2493ee7522b3a778549df95bf43e77dd70337f0b7c7aca51db7`.
- Accepted specification hash after promotion: `4decc6678777f104e0f07e9e9c34f1fb89e0bcec622fa672089814efe412ff0f`.
- Archive execution receipt: `ec18bbc27152a158d9805e85f5e469d61711501c100bccd322947510c4198544`.

## Existing roadmap blocker

The roadmap rail reports two unrelated active changes that were already unclassified:

- `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-optional-basalt-authority-adapter`
- `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-pijul-vcs-adapter`

The rail reports no missing product safety paths. This change does not classify unrelated work.

## Evidence boundary

The summary contains profile, family, fixture, entrypoint, service, expected diagnostic, count, redaction, non-claim, and BLAKE3 identity metadata.

It excludes fixture bodies, transcripts, hidden tests, reasoning, provider bodies, tool bodies, secrets, environment values, physical paths, and unrestricted logs.

## Non-claims

This evidence does not prove live host behavior, provider correctness, tool correctness, sandboxing, network enforcement, storage durability, or exactly-once effects.

It does not prove verifier soundness, artifact authenticity, model quality, benchmark validity, training fitness, authoritative completion, production readiness, or release eligibility.
