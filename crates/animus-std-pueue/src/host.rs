use animus_adapters::{ComponentAdapter, ServiceOutcome};
use animus_adapters::{QueueInteractionKind, QueueServiceRequest, QueueServiceResponse};

use crate::PueueHostConfig;
use crate::process::{PueueProcessRequest, PueueProcessService};

#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::non_trait_imports,
        reason = "safe receipt DTO imports keep evidence construction separate from host orchestration"
    )
)]
mod evidence;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "typed outcome helpers keep process failures and queue service receipts explicit"
    )
)]
mod outcome;

use outcome::{
    HostFailure, HostPlan, HostResult, ProcessSuccess, classify_output, map_command_error,
    map_host_result, map_projection_error, task_is_terminal,
};

/// Concrete pueue queue host over one explicit namespace and process service.
pub struct PueueHost<Service> {
    config: PueueHostConfig,
    process: Service,
}

impl<Service> PueueHost<Service> {
    /// Builds a host without discovering executables, paths, or ambient daemons.
    #[must_use]
    pub const fn new(config: PueueHostConfig, process: Service) -> Self {
        Self { config, process }
    }

    /// Returns the validated host configuration.
    #[must_use]
    pub const fn config(&self) -> &PueueHostConfig {
        &self.config
    }

    /// Returns the injected process service.
    #[must_use]
    pub const fn process(&self) -> &Service {
        &self.process
    }

    /// Returns the injected process service mutably.
    #[must_use]
    pub const fn process_mut(&mut self) -> &mut Service {
        &mut self.process
    }
}

impl<Service> ComponentAdapter<QueueServiceRequest, ServiceOutcome<QueueServiceResponse>>
    for PueueHost<Service>
where
    Service: PueueProcessService,
{
    type Error = core::convert::Infallible;

    fn call(
        &mut self,
        request: &QueueServiceRequest,
    ) -> Result<ServiceOutcome<QueueServiceResponse>, Self::Error> {
        let result = self.execute(request);
        Ok(map_host_result(request, result))
    }
}

impl<Service> PueueHost<Service>
where
    Service: PueueProcessService,
{
    fn execute(&mut self, request: &QueueServiceRequest) -> HostResult {
        let plan = self.plan(request)?;
        self.ensure_daemon()?;
        self.execute_plan(request, plan)
    }

    fn plan(&self, request: &QueueServiceRequest) -> Result<HostPlan, HostFailure> {
        match request {
            QueueServiceRequest::Enqueue(enqueue) => {
                let resolved = self
                    .config
                    .resolve_profile(enqueue.command_label())
                    .map_err(|_error| HostFailure::Denied)?;
                let command = crate::command::enqueue(&self.config, enqueue, &resolved)
                    .map_err(map_command_error)?;
                Ok(HostPlan::Enqueue(command))
            }
            QueueServiceRequest::Control(control) => {
                let command =
                    crate::command::control(&self.config, control).map_err(map_command_error)?;
                Ok(HostPlan::Control(command))
            }
            QueueServiceRequest::Interaction(interaction) => self.plan_interaction(interaction),
            QueueServiceRequest::ReadLog(log) => {
                let bounds = log.bounds();
                let host_bounds = self.config.limits();
                if bounds.max_visible_lines() > host_bounds.max_log_lines
                    || bounds.max_visible_bytes() > host_bounds.max_log_bytes
                {
                    return Err(HostFailure::Denied);
                }
                let command =
                    crate::command::logs(&self.config, log.task_id()).map_err(map_command_error)?;
                Ok(HostPlan::Log(command))
            }
            QueueServiceRequest::ListGroups => {
                let command = crate::command::status(&self.config).map_err(map_command_error)?;
                Ok(HostPlan::Groups(command))
            }
        }
    }

    fn plan_interaction(
        &self,
        interaction: &animus_adapters::QueueInteractionRequest,
    ) -> Result<HostPlan, HostFailure> {
        if interaction.timeout_policy() != self.config.timeout_policy {
            return Err(HostFailure::Denied);
        }
        match interaction.kind() {
            QueueInteractionKind::Poll if self.config.capabilities().poll() => {
                let status = crate::command::status(&self.config).map_err(map_command_error)?;
                Ok(HostPlan::ObserveTask {
                    wait_until_terminal: false,
                    status,
                })
            }
            QueueInteractionKind::Wait if self.config.capabilities().wait() => {
                let status = crate::command::status(&self.config).map_err(map_command_error)?;
                Ok(HostPlan::ObserveTask {
                    wait_until_terminal: true,
                    status,
                })
            }
            QueueInteractionKind::Stream | QueueInteractionKind::SendInput => {
                Err(HostFailure::Denied)
            }
            QueueInteractionKind::Poll | QueueInteractionKind::Wait => Err(HostFailure::Denied),
        }
    }

    fn ensure_daemon(&mut self) -> Result<(), HostFailure> {
        let health = crate::command::health(&self.config).map_err(map_command_error)?;
        match self.run_process(&health)? {
            ProcessSuccess::Completed(_bytes) => Ok(()),
            ProcessSuccess::ExitFailed => Err(HostFailure::Unavailable),
        }
    }

    fn execute_plan(&mut self, request: &QueueServiceRequest, plan: HostPlan) -> HostResult {
        match (request, plan) {
            (QueueServiceRequest::Enqueue(enqueue), HostPlan::Enqueue(command)) => {
                let output = self.completed_bytes(&command)?;
                let task = crate::projection::enqueue_observation(output.as_slice(), enqueue)
                    .map_err(map_projection_error)?;
                Ok(QueueServiceResponse::Task(task))
            }
            (QueueServiceRequest::Control(control), HostPlan::Control(command)) => {
                self.completed_bytes(&command)?;
                Ok(QueueServiceResponse::ControlAccepted(control.operation()))
            }
            (
                QueueServiceRequest::Interaction(interaction),
                HostPlan::ObserveTask {
                    wait_until_terminal,
                    status,
                },
            ) => {
                let task = if wait_until_terminal {
                    self.wait_for_task(&status, interaction.task_id())?
                } else {
                    self.observe_task(&status, interaction.task_id())?
                };
                Ok(QueueServiceResponse::Task(task))
            }
            (QueueServiceRequest::ReadLog(log), HostPlan::Log(command)) => {
                let output = self.completed_bytes(&command)?;
                let log_view = crate::projection::log_viewport(output.as_slice(), log)
                    .map_err(map_projection_error)?;
                Ok(QueueServiceResponse::Log(log_view))
            }
            (QueueServiceRequest::ListGroups, HostPlan::Groups(command)) => {
                let output = self.completed_bytes(&command)?;
                let groups = crate::projection::group_observations(output.as_slice())
                    .map_err(map_projection_error)?;
                Ok(QueueServiceResponse::Groups(groups))
            }
            _mismatched_plan => Err(HostFailure::Malformed),
        }
    }

    fn observe_task(
        &mut self,
        status: &PueueProcessRequest,
        task_id: &str,
    ) -> Result<animus_adapters::QueueTaskObservation, HostFailure> {
        let output = self.completed_bytes(status)?;
        crate::projection::task_observation(output.as_slice(), task_id)
            .map_err(map_projection_error)
    }

    fn wait_for_task(
        &mut self,
        status: &PueueProcessRequest,
        task_id: &str,
    ) -> Result<animus_adapters::QueueTaskObservation, HostFailure> {
        let poll_delay_ms = std::time::Duration::from_millis(self.config.limits.poll_interval_ms);
        for _attempt in 0..self.config.limits.wait_poll_attempts {
            let task = self.observe_task(status, task_id)?;
            if task_is_terminal(task.status()) {
                return Ok(task);
            }
            std::thread::sleep(poll_delay_ms);
        }
        Err(HostFailure::TimedOut)
    }

    fn completed_bytes(&mut self, request: &PueueProcessRequest) -> Result<Vec<u8>, HostFailure> {
        match self.run_process(request)? {
            ProcessSuccess::Completed(bytes) => Ok(bytes),
            ProcessSuccess::ExitFailed => Err(HostFailure::Unavailable),
        }
    }

    fn run_process(
        &mut self,
        request: &PueueProcessRequest,
    ) -> Result<ProcessSuccess, HostFailure> {
        let output = self
            .process
            .run(request)
            .map_err(|_error| HostFailure::Unavailable)?;
        classify_output(output)
    }
}
