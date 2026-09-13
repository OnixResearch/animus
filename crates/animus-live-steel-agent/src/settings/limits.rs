impl super::ValidatedLiveSteelAgentConfig {
    /// Returns the finite host-executor bounds.
    #[must_use]
    pub fn executor_limits(&self) -> animus_std_rlm_host::ExecutorLimits {
        animus_std_rlm_host::ExecutorLimits {
            max_file_read_bytes: self.raw.bounds.max_file_read_bytes,
            max_context_bytes: self.raw.bounds.max_context_bytes,
            max_context_lines: self.raw.bounds.max_context_lines,
            max_result_chars: self.raw.bounds.max_tool_result_chars,
            max_child_status_checks: self.raw.bounds.max_child_status_checks,
        }
    }

    /// Returns the finite child-session bounds.
    #[must_use]
    pub const fn child_session_limits(
        &self,
    ) -> animus_runtime::child_session_service::ChildSessionLimits {
        animus_runtime::child_session_service::ChildSessionLimits {
            depth: self.raw.bounds.max_child_depth,
            fan_out: self.raw.bounds.max_child_fan_out,
            wall_time_ms: self.raw.bounds.max_child_wall_time_ms,
            tokens: self.raw.bounds.max_child_tokens,
            turns: self.raw.bounds.max_bridge_nesting,
            tool_calls: self.raw.bounds.max_host_calls_per_cell,
            result_chars: self.raw.bounds.max_child_result_chars,
        }
    }

    /// Returns the exact child service label.
    #[must_use]
    pub fn child_service_label(&self) -> &str {
        self.raw.services.child.as_str()
    }

    /// Returns the exact context service label.
    #[must_use]
    pub fn context_service_label(&self) -> &str {
        self.raw.services.context.as_str()
    }

    /// Returns one exact authority identity.
    #[must_use]
    pub fn authority_component_identity(&self) -> &str {
        self.raw.authority.component_identity.as_str()
    }

    /// Returns one exact authorization identity.
    #[must_use]
    pub fn authority_authorization_identity(&self) -> &str {
        self.raw.authority.authorization_identity.as_str()
    }

    /// Returns one exact route binding identity.
    #[must_use]
    pub fn route_binding_identity(&self) -> &str {
        self.raw.authority.route_binding_identity.as_str()
    }

    /// Returns one exact route scope identity.
    #[must_use]
    pub fn route_scope_identity(&self) -> &str {
        self.raw.authority.route_scope_identity.as_str()
    }

    /// Returns one exact operation identity.
    #[must_use]
    pub fn operation_identity(&self) -> &str {
        self.raw.authority.operation_identity.as_str()
    }

    /// Returns one exact resource identity.
    #[must_use]
    pub fn resource_identity(&self) -> &str {
        self.raw.authority.resource_identity.as_str()
    }

    /// Returns required evidence non-claims.
    #[must_use]
    pub fn non_claims(&self) -> &[String] {
        self.raw.evidence.non_claims.as_slice()
    }
}

pub(super) fn validate_bounds(
    bounds: super::shape::BoundConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    let values = [
        bounds.provider_timeout_ms,
        bounds.max_provider_response_bytes,
        bounds.max_output_tokens,
        u64::from(bounds.max_tool_result_chars),
        u64::from(bounds.max_queue_items),
        bounds.max_file_read_bytes,
        u64::from(bounds.max_context_bytes),
        u64::from(bounds.max_context_lines),
        u64::from(bounds.max_child_status_checks),
        u64::from(bounds.max_source_chars),
        u64::from(bounds.max_environment_chars),
        u64::from(bounds.max_bindings),
        u64::from(bounds.max_definitions),
        bounds.max_evaluation_fuel,
        u64::from(bounds.max_recursion_depth),
        bounds.max_allocation_bytes,
        u64::from(bounds.max_host_request_chars),
        u64::from(bounds.max_child_depth),
        u64::from(bounds.max_child_fan_out),
        bounds.max_child_tokens,
        bounds.max_child_wall_time_ms,
        u64::from(bounds.max_child_result_chars),
        u64::from(bounds.max_host_calls_per_cell),
        u64::from(bounds.max_host_result_chars),
        u64::from(bounds.max_cumulative_host_result_chars),
        u64::from(bounds.max_bridge_nesting),
        bounds.max_bridge_evaluation_fuel,
        bounds.max_bridge_wall_time_ms,
    ];
    if values.contains(&0) {
        return Err(super::LiveSteelAgentConfigError::InvalidBound);
    }
    debug_assert!(bounds.max_queue_items > 0);
    debug_assert!(bounds.max_bridge_wall_time_ms > 0);
    Ok(())
}
