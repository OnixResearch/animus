pub(crate) struct LockedPlan {
    pub(crate) admitted: cap_root_core::RootRelativePath,
    pub(crate) plan: crate::model::EditPlan,
    lease: Option<crate::concrete::WriterLease>,
}

fn release_for_outcome(
    outcome: &crate::model::PublicationOutcome,
) -> crate::concrete::WriterRelease {
    match outcome.state() {
        crate::model::PublicationState::FailedBeforePublication => {
            crate::concrete::WriterRelease::NotPublished
        }
        crate::model::PublicationState::Published
        | crate::model::PublicationState::DurabilityPending
        | crate::model::PublicationState::DurabilityComplete => {
            crate::concrete::WriterRelease::Published
        }
        crate::model::PublicationState::UncertainAfterPublication
        | crate::model::PublicationState::Prepared
        | crate::model::PublicationState::Staged => {
            crate::concrete::WriterRelease::PublicationUncertain
        }
    }
}

fn expected_released_generation(
    current: u64,
    release: crate::concrete::WriterRelease,
) -> Result<u64, crate::concrete::HostError> {
    match release {
        crate::concrete::WriterRelease::NotPublished => Ok(current),
        crate::concrete::WriterRelease::Published
        | crate::concrete::WriterRelease::PublicationUncertain => {
            current
                .checked_add(1)
                .ok_or(crate::concrete::HostError::Service(
                    crate::concrete::HostServiceIssue::ReleaseFailed,
                ))
        }
    }
}

fn release_locked(
    services: &mut impl crate::concrete::HostServices,
    locked: &mut LockedPlan,
    release: crate::concrete::WriterRelease,
) -> Result<u64, crate::concrete::HostError> {
    let lease = locked
        .lease
        .take()
        .ok_or(crate::concrete::HostError::Service(
            crate::concrete::HostServiceIssue::ReleaseFailed,
        ))?;
    services
        .release_writer(lease, release)
        .map_err(crate::concrete::HostError::Service)
}

fn fail_with_release<T>(
    services: &mut impl crate::concrete::HostServices,
    locked: &mut LockedPlan,
    release: crate::concrete::WriterRelease,
    error: crate::concrete::HostError,
) -> Result<T, crate::concrete::HostError> {
    release_locked(services, locked, release)?;
    Err(error)
}

fn fail_not_published<T>(
    services: &mut impl crate::concrete::HostServices,
    locked: &mut LockedPlan,
    error: crate::concrete::HostError,
) -> Result<T, crate::concrete::HostError> {
    fail_with_release(
        services,
        locked,
        crate::concrete::WriterRelease::NotPublished,
        error,
    )
}

fn prepare_locked_plan(
    root: &cap_std::fs::Dir,
    services: &mut impl crate::concrete::HostServices,
    request: &crate::concrete::HostRequest<'_>,
) -> Result<LockedPlan, crate::concrete::HostError> {
    crate::planning::validate_host_gates(request.gates)?;
    let admitted = cap_root_core::RootRelativePath::new(
        request.logical_resource,
        cap_root_core::AdmissionMode::NonEmptyRelative,
    )
    .map_err(|_| crate::concrete::HostError::PathDenied)?;
    let resource_identity =
        crate::concrete::logical_resource_identity(request.root_identity, admitted.as_path())?;
    if resource_identity != request.resource_identity {
        return Err(crate::concrete::HostError::PathDenied);
    }
    if request.base_snapshot.resource_identity() != request.resource_identity {
        return Err(crate::concrete::HostError::PathDenied);
    }
    let initial = services
        .observe(request.resource_identity)
        .map_err(crate::concrete::HostError::Service)?;
    crate::concrete::observation::validate_initial(request, &initial)?;
    let first_read = crate::concrete::observation::read_resource(root, &admitted)?;
    crate::concrete::observation::enforce_file_policy(request, first_read.facts)?;
    let snapshot = crate::planning::admit_snapshot(&crate::model::SnapshotDraft {
        base: request.base_snapshot,
        root_identity: request.root_identity,
        resource_identity: request.resource_identity,
        revision_identity: request.revision_identity,
        metadata_policy_identity: request.metadata_policy_identity,
        file_metadata_identity: &first_read.metadata_identity,
        writer_generation: request.writer_generation,
        source_bytes: &first_read.bytes,
        line_ending_mode: request.line_ending_mode,
        final_newline: request.final_newline,
        replacement_ending: request.replacement_ending,
        mixed_endings_admitted: request.mixed_endings_admitted,
        visible_spans: request.visible_spans,
        visible_boundaries: request.visible_boundaries,
        end_boundary_visible: request.end_boundary_visible,
    })?;
    let plan = crate::planning::plan_edit(&snapshot, request.operations)?;
    let lease = services
        .acquire_writer(
            request.resource_identity,
            request.revision_identity,
            request.writer_generation,
        )
        .map_err(crate::concrete::HostError::Service)?;
    let mut locked = LockedPlan {
        admitted,
        plan,
        lease: Some(lease),
    };
    let lease = locked
        .lease
        .as_ref()
        .ok_or(crate::concrete::HostError::Service(
            crate::concrete::HostServiceIssue::ReleaseFailed,
        ))?;
    if let Err(error) = crate::concrete::observation::validate_lease(request, lease) {
        return fail_not_published(services, &mut locked, error);
    }
    debug_assert!(locked.lease.is_some());
    Ok(locked)
}

fn validate_locked_freshness(
    root: &cap_std::fs::Dir,
    services: &mut impl crate::concrete::HostServices,
    request: &crate::concrete::HostRequest<'_>,
    locked: &mut LockedPlan,
) -> Result<(), crate::concrete::HostError> {
    if let Err(issue) = services.checkpoint(crate::concrete::HostCheckpoint::BeforeSecondRead) {
        return fail_not_published(services, locked, crate::concrete::HostError::Service(issue));
    }
    let second_observation = match services.observe(request.resource_identity) {
        Ok(observation) => observation,
        Err(issue) => {
            return fail_not_published(
                services,
                locked,
                crate::concrete::HostError::Service(issue),
            );
        }
    };
    let second_read = match crate::concrete::observation::read_resource(root, &locked.admitted) {
        Ok(read) => read,
        Err(error) => return fail_not_published(services, locked, error),
    };
    let second_content_identity = blake3::hash(&second_read.bytes).to_hex().to_string();
    let freshness = crate::planning::validate_freshness(
        &locked.plan,
        crate::model::FreshnessObservation {
            root_identity: request.root_identity,
            resource_identity: request.resource_identity,
            revision_identity: &second_observation.revision_identity,
            content_identity: &second_content_identity,
            metadata_policy_identity: &second_observation.metadata_policy_identity,
            file_metadata_identity: &second_read.metadata_identity,
            writer_generation: second_observation.writer_generation,
        },
    );
    if let Err(issue) = freshness {
        return fail_not_published(services, locked, issue.into());
    }
    debug_assert_eq!(
        second_content_identity,
        locked.plan.snapshot().content_identity()
    );
    Ok(())
}

fn finish_publication(
    services: &mut impl crate::concrete::HostServices,
    request: &crate::concrete::HostRequest<'_>,
    mut locked: LockedPlan,
    classification: &durable_file_publication::core::Classification,
) -> Result<crate::evidence::EditReceipt, crate::concrete::HostError> {
    let outcome = match crate::planning::reduce_publication(classification) {
        Ok(outcome) => outcome,
        Err(issue) => {
            return fail_with_release(
                services,
                &mut locked,
                crate::concrete::WriterRelease::PublicationUncertain,
                issue.into(),
            );
        }
    };
    let release = release_for_outcome(&outcome);
    let expected_generation = match expected_released_generation(request.writer_generation, release)
    {
        Ok(generation) => generation,
        Err(error) => {
            return fail_with_release(
                services,
                &mut locked,
                crate::concrete::WriterRelease::PublicationUncertain,
                error,
            );
        }
    };
    let released_generation = release_locked(services, &mut locked, release)?;
    if released_generation != expected_generation {
        return Err(crate::concrete::HostError::Service(
            crate::concrete::HostServiceIssue::ReleaseFailed,
        ));
    }
    debug_assert_eq!(released_generation, expected_generation);
    crate::evidence::project_receipt(crate::evidence::ReceiptDraft {
        plan: &locked.plan,
        publication: &outcome,
        released_writer_generation: released_generation,
        leak_facts: crate::evidence::ReceiptLeakFacts::default(),
    })
    .map_err(crate::concrete::HostError::Receipt)
}

pub(crate) fn apply_with_publisher(
    root: &cap_std::fs::Dir,
    services: &mut impl crate::concrete::HostServices,
    request: &crate::concrete::HostRequest<'_>,
    publisher: impl FnOnce(
        &cap_root_core::RootRelativePath,
        &[u8],
    ) -> Result<
        durable_file_publication::core::Classification,
        crate::concrete::HostError,
    >,
) -> Result<crate::evidence::EditReceipt, crate::concrete::HostError> {
    let mut locked = prepare_locked_plan(root, services, request)?;
    validate_locked_freshness(root, services, request, &mut locked)?;
    if let Err(issue) = services.checkpoint(crate::concrete::HostCheckpoint::BeforePublication) {
        return fail_not_published(
            services,
            &mut locked,
            crate::concrete::HostError::Service(issue),
        );
    }
    let classification = match publisher(&locked.admitted, locked.plan.after_bytes()) {
        Ok(classification) => classification,
        Err(error) => return fail_not_published(services, &mut locked, error),
    };
    debug_assert!(locked.lease.is_some());
    finish_publication(services, request, locked, &classification)
}
