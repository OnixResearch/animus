pub(crate) struct ProviderAdapter<Provider, SnapshotStore> {
    pub(super) provider: Provider,
    pub(crate) snapshot_store: SnapshotStore,
    pub(super) config: std::rc::Rc<crate::ValidatedLiveSteelAgentConfig>,
    pub(super) shared: std::rc::Rc<
        std::cell::RefCell<crate::adapters::responses::continuation::SharedProviderState>,
    >,
    pub(crate) dispatch_count: u32,
}

impl<Provider, SnapshotStore> animus_std::coding_session_host::CodingSessionProviderService
    for ProviderAdapter<Provider, SnapshotStore>
where
    Provider: crate::ProviderStreamService,
    SnapshotStore: animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort,
{
    fn dispatch(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionProviderReply,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        self.advance_count()?;
        let (model_call_id, continuation, correlation_id) = {
            let mut shared = self.shared.borrow_mut();
            if shared.is_blocked() {
                return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale);
            }
            let model_call_id = shared
                .next_model_call_id()
                .map_err(super::policy::service_issue)?;
            let correlation_id = shared
                .pending()
                .map(|pending| String::from(pending.correlation_id()));
            (model_call_id, shared.pending().is_some(), correlation_id)
        };
        let planned = crate::adapters::responses::planning::plan_request(
            self.config.as_ref(),
            request,
            continuation,
            correlation_id.as_deref(),
        )
        .map_err(super::policy::service_issue)?;
        let dispatch = if continuation {
            None
        } else {
            Some(
                crate::adapters::responses::continuation::initial_dispatch(
                    crate::adapters::responses::continuation::InitialCall {
                        model_id: self.config.model_id(),
                        model_call_id: model_call_id.as_str(),
                    },
                )
                .map_err(super::policy::service_issue)?,
            )
        };
        let prepared = super::prerequisite::prepare_live_model_request_snapshot(
            self.config.as_ref(),
            request,
            &planned.canonical,
            &planned.plan,
        )?;
        let mut pending_dispatch = if continuation {
            Some(
                self.shared
                    .borrow_mut()
                    .block_pending_for_effect()
                    .map_err(super::policy::service_issue)?,
            )
        } else {
            dispatch
        };
        let mut provider_effect =
            |_snapshot_plan: &animus_runtime::model_request_snapshot::ModelRequestDispatchPlan,
             _canonical: &animus_adapters::CanonicalModelRequest| {
                let dispatch = pending_dispatch.take().ok_or(
                    animus_runtime::model_request_snapshot::ModelRequestProviderTerminalStatus::Failed,
                )?;
                self.provider
                    .stream(&planned.plan, dispatch)
                    .map_err(|_error| {
                        animus_runtime::model_request_snapshot::ModelRequestProviderTerminalStatus::Failed
                    })
            };
        let outcome = animus_std::model_request_snapshot_host::dispatch_initial_model_request(
            &mut self.snapshot_store,
            &mut provider_effect,
            prepared,
        )
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?
        .output;
        self.project_reply(&outcome)
    }
}

impl<Provider, SnapshotStore> ProviderAdapter<Provider, SnapshotStore> {
    fn advance_count(
        &mut self,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        self.dispatch_count = self
            .dispatch_count
            .checked_add(1)
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
        Ok(())
    }

    fn project_reply(
        &mut self,
        outcome: &animus_std_openai::OpenAiStreamOutcome,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionProviderReply,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        if outcome.tool_calls().len() > 1 {
            return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed);
        }
        if let Some(call) = outcome.tool_calls().first() {
            if call.tool_name() != animus_std_openai::STEEL_REPL_TOOL_NAME {
                return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied);
            }
            let arguments: super::policy::SteelArguments =
                serde_json::from_str(call.arguments_json()).map_err(|_error| {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
                })?;
            let tool_request = super::policy::steel_tool_request(
                super::policy::CellSource {
                    correlation_id: call.continuation_binding().tool_call_id(),
                    code: arguments.code.as_str(),
                },
                self.config.max_tool_result_chars(),
            )?;
            let assistant =
                animus_adapters::ModelResponse::new("steel_repl requested").map_err(|_error| {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
                })?;
            self.shared.borrow_mut().install(
                crate::adapters::responses::continuation::pending_from_call(
                    self.config.model_id(),
                    call,
                )
                .map_err(super::policy::service_issue)?,
            );
            return Ok(
                animus_runtime::coding_session_host::CodingSessionProviderReply::ToolCall {
                    assistant_response: assistant,
                    tool_request,
                },
            );
        }
        let text = crate::adapters::responses::planning::final_text(outcome)
            .map_err(super::policy::service_issue)?;
        let response = animus_adapters::ModelResponse::new(text.as_str()).map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
        self.shared.borrow_mut().clear_blocked();
        Ok(animus_runtime::coding_session_host::CodingSessionProviderReply::Final(response))
    }
}
