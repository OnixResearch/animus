/// Require tool, capability, authority, approval, mutation, lifecycle, and output gates.
///
/// # Errors
///
/// Returns the first stable denied gate before capability-relative file I/O.
// r[impl agent_harness_runtime.hashed_edit_host.boundary]
pub fn validate_host_gates(
    gates: crate::model::GateObservations,
) -> Result<(), crate::model::HostIssue> {
    let checks = [
        (
            gates.executor == crate::model::GateDecision::Denied,
            "executor",
        ),
        (
            gates.capability == crate::model::GateDecision::Denied,
            "capability",
        ),
        (
            gates.authority == crate::model::GateDecision::Denied,
            "authority",
        ),
        (
            gates.approval == crate::model::GateDecision::Denied,
            "approval",
        ),
        (
            gates.mutation == crate::model::GateDecision::Denied,
            "mutation-serialization",
        ),
        (
            gates.lifecycle == crate::model::LifecycleGate::Cancelled,
            "cancellation",
        ),
        (
            gates.lifecycle == crate::model::LifecycleGate::TimedOut,
            "deadline",
        ),
        (
            gates.output == crate::model::GateDecision::Denied,
            "output-boundary",
        ),
    ];
    for (is_denied, label) in checks {
        if is_denied {
            return Err(crate::model::HostIssue::GateDenied(label));
        }
    }
    debug_assert_eq!(gates.lifecycle, crate::model::LifecycleGate::Current);
    Ok(())
}

/// Revalidate exact root, resource, revision, content, policy, and writer facts.
///
/// # Errors
///
/// Returns the exact stale field before the first staging or rename effect.
// r[impl agent_harness_runtime.hashed_edit_host.freshness]
pub fn validate_freshness(
    plan: &crate::model::EditPlan,
    observation: crate::model::FreshnessObservation<'_>,
) -> Result<(), crate::model::HostIssue> {
    let snapshot = plan.snapshot();
    let checks = [
        (
            observation.root_identity != snapshot.root_identity(),
            crate::model::StaleField::Root,
        ),
        (
            observation.resource_identity != snapshot.resource_identity(),
            crate::model::StaleField::Resource,
        ),
        (
            observation.revision_identity != snapshot.revision_identity(),
            crate::model::StaleField::Revision,
        ),
        (
            observation.content_identity != snapshot.content_identity(),
            crate::model::StaleField::Content,
        ),
        (
            observation.metadata_policy_identity != snapshot.metadata_policy_identity(),
            crate::model::StaleField::MetadataPolicy,
        ),
        (
            observation.file_metadata_identity != snapshot.file_metadata_identity(),
            crate::model::StaleField::FileMetadata,
        ),
        (
            observation.writer_generation != snapshot.writer_generation(),
            crate::model::StaleField::WriterGeneration,
        ),
    ];
    for (is_stale, field) in checks {
        if is_stale {
            return Err(crate::model::HostIssue::Stale(field));
        }
    }
    debug_assert_eq!(observation.writer_generation, snapshot.writer_generation());
    Ok(())
}
