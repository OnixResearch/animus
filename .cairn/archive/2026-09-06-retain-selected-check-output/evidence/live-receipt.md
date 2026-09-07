schema: animus.authoritative-completion-check.v1
status: pass
registry_source: config/agent-check-registry.ncl
registry_export: config/generated/agent-check-registry.json
registry_blake3: 1d9f97f93a918ae32c301cbc8c4eb0548706eb91b6c2e72a6468436255273555
steel_script_blake3: c49363bf4ab5ecd8dd519138acb3ca3b251edb0646f3ac4e98b9393c9ac93a88
positive_nickel_fixtures: 1
negative_nickel_fixtures: 5
runtime_positive_and_negative_tests: pass
pass_only_status_matrix: pass
exact_completion_candidate: pass
incidental_complete_substrings: rejected
continuation_empty_queue: awaits-authoritative-evidence
cairn_schema: cairn.agent-completion.v2
cairn_checks_executed: false
cairn_response_mode: live
cairn_complete_report_parity: pass
cairn_stale_policy_fixture: rejected
cairn_positive_receipt_hash: 78fc4f60a136e42f71deff7450ffecfd77d38476e42b12d7120561428ad3f199
cairn_stale_subject_fixture: rejected
authoritative_completion: bounded-selected-scope-only
non_claims:
- no check-oracle soundness claim
- no honest-execution claim beyond supplied host evidence
- no model or whole-product correctness claim
- no production, release, push, or pull-request authority
