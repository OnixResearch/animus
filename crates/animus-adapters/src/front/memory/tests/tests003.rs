    const EXPECTED_SINGLE_ITEM_U32: u32 = 1;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct GenericEnvelopeFixture {
        id: &'static str,
        body_hash: &'static str,
        scope: Option<MemoryTrustScope>,
        freshness: MemoryFreshness,
        admission_status: MemoryAdmissionStatus,
        redaction_class: MemoryRedactionClass,
        confidence_percent: u8,
        budget_units: u32,
        conflict_group: Option<&'static str>,
        superseded_by_id: Option<&'static str>,
    }

    impl GenericEnvelopeFixture {
        const fn valid() -> Self {
            Self {
                id: RECORD_ID,
                body_hash: VALID_HASH,
                scope: Some(MemoryTrustScope::Project),
                freshness: MemoryFreshness::Fresh,
                admission_status: MemoryAdmissionStatus::Admitted,
                redaction_class: MemoryRedactionClass::Public,
                confidence_percent: MEMORY_CONFIDENCE_PERCENT,
                budget_units: SMALL_MEMORY_UNITS,
                conflict_group: None,
                superseded_by_id: None,
            }
        }

        const fn draft(self) -> MemoryRecordEnvelopeDraft<'static> {
            MemoryRecordEnvelopeDraft {
                id: self.id,
                kind: GenericMemoryRecordKind::Fact,
                scope: self.scope,
                source_label: SOURCE_LABEL,
                evidence_handle: Some(EVIDENCE_HANDLE),
                admission_status: self.admission_status,
                freshness: self.freshness,
                confidence_percent: self.confidence_percent,
                redaction_class: self.redaction_class,
                body_hash: self.body_hash,
                budget_units: self.budget_units,
                expiration_label: None,
                supersedes_id: None,
                superseded_by_id: self.superseded_by_id,
                conflict_group: self.conflict_group,
                raw_body_marker: None,
                hidden_body_marker: None,
            }
        }
    }

    fn generic_envelope(fixture: GenericEnvelopeFixture) -> MemoryRecordEnvelope {
        MemoryRecordEnvelope::validate(&fixture.draft()).expect("record envelope should validate")
    }

    fn adapter_descriptor(
        role: GenericMemoryAdapterRole,
        capability: GenericMemoryAdapterCapability,
    ) -> MemoryAdapterFamilyDescriptor {
        MemoryAdapterFamilyDescriptor::validate(&MemoryAdapterFamilyDescriptorDraft {
            adapter_label: role.as_str(),
            role,
            capabilities: &[capability, GenericMemoryAdapterCapability::SafeReceipts],
            availability: MemoryAvailabilityStatus::Available,
            owner_label: "host-owned-memory",
        })
        .expect("adapter descriptor should validate")
    }

    fn generic_receipt_draft() -> MemoryOperationReceiptDraft<'static> {
        MemoryOperationReceiptDraft {
            operation_label: "memory-operation",
            adapter_family_label: "retrieval-adapter",
            operation_kind: MemoryOperationKind::Retrieval,
            record_id: Some(RECORD_ID),
            candidate_id: None,
            item_count: EXPECTED_SINGLE_ITEM_U32,
            status: MemoryOperationStatus::Complete,
            redaction_class: MemoryRedactionClass::Public,
            policy_hash: POLICY_HASH,
            body_hash: VALID_HASH,
            raw_memory_body: None,
            credential: None,
            provider_payload: None,
            full_path: None,
            hidden_memory_body: None,
            unbounded_prompt_text: None,
        }
    }

    #[test]
    fn generic_memory_adapter_family_fails_closed_without_ambient_fallback() {
        // r[verify agent_harness_runtime.memory_adapters.family]
        // r[verify agent_harness_runtime.memory_adapters.family.missing]
        let source = adapter_descriptor(
            GenericMemoryAdapterRole::SourceImport,
            GenericMemoryAdapterCapability::SourceImport,
        );
        let retrieval = adapter_descriptor(
            GenericMemoryAdapterRole::RetrievalSearch,
            GenericMemoryAdapterCapability::RetrievalSearch,
        );
        let admission = adapter_descriptor(
            GenericMemoryAdapterRole::Admission,
            GenericMemoryAdapterCapability::AdmissionReview,
        );
        let required = &[
            GenericMemoryAdapterRole::SourceImport,
            GenericMemoryAdapterRole::RetrievalSearch,
            GenericMemoryAdapterRole::Admission,
        ];
        assert_eq!(
            evaluate_memory_adapter_operation(MemoryAdapterOperationInput {
                enabled: true,
                required_roles: required,
                adapters: &[source.clone(), retrieval.clone(), admission],
            }),
            MemoryAdapterOperationAvailability::Ready
        );
        assert_eq!(
            evaluate_memory_adapter_operation(MemoryAdapterOperationInput {
                enabled: true,
                required_roles: required,
                adapters: &[source.clone(), retrieval],
            }),
            MemoryAdapterOperationAvailability::Unavailable {
                role: GenericMemoryAdapterRole::Admission,
            }
        );
        let adapter_without_store_capability = MemoryAdapterFamilyDescriptor::validate(
            &MemoryAdapterFamilyDescriptorDraft {
                adapter_label: "store-adapter",
                role: GenericMemoryAdapterRole::DurableStore,
                capabilities: &[GenericMemoryAdapterCapability::SafeReceipts],
                availability: MemoryAvailabilityStatus::Available,
                owner_label: "host-owned-memory",
            },
        )
        .expect("descriptor with wrong capability should validate metadata");
        assert_eq!(
            evaluate_memory_adapter_operation(MemoryAdapterOperationInput {
                enabled: true,
                required_roles: &[GenericMemoryAdapterRole::DurableStore],
                adapters: &[adapter_without_store_capability],
            }),
            MemoryAdapterOperationAvailability::Unsupported {
                role: GenericMemoryAdapterRole::DurableStore,
            }
        );
        assert!(matches!(
            MemoryAdapterFamilyDescriptor::validate(&MemoryAdapterFamilyDescriptorDraft {
                adapter_label: "empty-adapter",
                role: GenericMemoryAdapterRole::Extraction,
                capabilities: &[],
                availability: MemoryAvailabilityStatus::Available,
                owner_label: "host-owned-memory",
            }),
            Err(MemoryError::MissingAdapterCapabilities { .. })
        ));
    }

    #[test]
    fn memory_record_envelopes_validate_required_metadata_and_reject_malformed_records() {
        // r[verify agent_harness_runtime.memory_adapters.record_envelope]
        // r[verify agent_harness_runtime.memory_adapters.record_envelope.invalid]
        let record = generic_envelope(GenericEnvelopeFixture::valid());
        assert_eq!(record.id(), RECORD_ID);
        assert_eq!(record.evidence_handle(), EVIDENCE_HANDLE);
        assert_eq!(record.scope(), MemoryTrustScope::Project);
        assert_eq!(record.confidence().percent(), MEMORY_CONFIDENCE_PERCENT);
        assert_eq!(record.body_hash().as_str(), VALID_HASH);

        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                evidence_handle: None,
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::MissingEvidence { .. })
        ));
        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                scope: None,
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::MissingRecordScope { .. })
        ));
        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                confidence_percent: INVALID_CONFIDENCE_PERCENT,
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::InvalidConfidence { .. })
        ));
        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                budget_units: u32::default(),
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::ZeroBudgetUnits { .. })
        ));
        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                freshness: MemoryFreshness::Expired,
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::ExpiredDescriptor { .. })
        ));
        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                body_hash: "not-a-hash",
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::InvalidHash(_))
        ));
        assert!(matches!(
            MemoryRecordEnvelope::validate(&MemoryRecordEnvelopeDraft {
                raw_body_marker: Some(RAW_BODY),
                ..GenericEnvelopeFixture::valid().draft()
            }),
            Err(MemoryError::ReceiptLeak { .. })
        ));
    }

    fn generic_retrieval_policy() -> MemoryRetrievalPolicy<'static> {
        MemoryRetrievalPolicy {
            query_label: "session-query",
            allowed_scopes: &[MemoryTrustScope::Project],
            include_stale: false,
            budget: MemoryInjectionBudget::new(MEMORY_UNITS).expect("budget should validate"),
        }
    }

    fn assert_retrieval_status(
        plan: &MemoryRetrievalPlan,
        record_id: &str,
        status: MemoryRetrievalOutcomeStatus,
    ) {
        assert!(plan.outcomes().iter().any(|outcome| {
            outcome.record_id() == record_id && outcome.status() == status
        }));
    }

    #[test]
    fn retrieval_planning_ignores_backend_labels_for_equivalent_records() {
        // r[verify agent_harness_runtime.memory_adapters.retrieval_plan]
        // r[verify agent_harness_runtime.memory_adapters.retrieval_plan.backend_equivalence]
        let selected = generic_envelope(GenericEnvelopeFixture::valid());
        let skipped = generic_envelope(GenericEnvelopeFixture {
            id: SECOND_RECORD_ID,
            body_hash: SECOND_HASH,
            scope: Some(MemoryTrustScope::Host),
            ..GenericEnvelopeFixture::valid()
        });
        let first = plan_memory_retrieval(MemoryRetrievalInput {
            policy: generic_retrieval_policy(),
            candidates: &[
                MemoryRetrievalCandidate {
                    record: &selected,
                    match_reason: MemoryMatchReason::ManualPin,
                    relevance_percent: MEMORY_CONFIDENCE_PERCENT,
                    backend_label: "mcp-backend",
                },
                MemoryRetrievalCandidate {
                    record: &skipped,
                    match_reason: MemoryMatchReason::ExactScope,
                    relevance_percent: MEMORY_CONFIDENCE_PERCENT,
                    backend_label: "sql-backend",
                },
            ],
        })
        .expect("retrieval planning should succeed");
        let second = plan_memory_retrieval(MemoryRetrievalInput {
            policy: generic_retrieval_policy(),
            candidates: &[
                MemoryRetrievalCandidate {
                    record: &selected,
                    match_reason: MemoryMatchReason::ManualPin,
                    relevance_percent: MEMORY_CONFIDENCE_PERCENT,
                    backend_label: "another-mcp-backend",
                },
                MemoryRetrievalCandidate {
                    record: &skipped,
                    match_reason: MemoryMatchReason::ExactScope,
                    relevance_percent: MEMORY_CONFIDENCE_PERCENT,
                    backend_label: "another-sql-backend",
                },
            ],
        })
        .expect("second retrieval planning should succeed");
        assert_eq!(first, second);
        assert_eq!(first.selected_count(), EXPECTED_SINGLE_ITEM_U32);
    }
