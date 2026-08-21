//! Bounded live-terminal orchestration over injected shell ports.

/// Terminal input, time, cancellation, and rendering port.
pub trait TerminalHostPort {
    /// Returns the current admitted viewport.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalHostError`] when dimensions are unavailable.
    fn viewport(&mut self) -> Result<(u16, u16), TerminalHostError>;
    /// Polls at most one terminal input under the selected timeout.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalHostError`] for terminal input failure.
    fn poll_input(
        &mut self,
        timeout_ms: u64,
    ) -> Result<Option<crate::TerminalInput>, TerminalHostError>;
    /// Returns whether caller cancellation was observed.
    fn cancelled(&self) -> bool;
    /// Returns whether a supported signal was observed.
    fn signalled(&self) -> bool;
    /// Renders one projected view.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalHostError`] for backend or rendering failure.
    fn render(&mut self, view: &crate::TerminalViewState) -> Result<(), TerminalHostError>;
}

/// Explicit run request over caller-owned ports.
pub struct LiveTerminalRequest<'config, 'host, 'connection, Host, Connection> {
    /// Validated terminal configuration.
    pub config: &'config crate::ValidatedTerminalConfig,
    /// Explicit connection implementation.
    pub connection: &'connection mut Connection,
    /// Terminal input and rendering shell.
    pub host: &'host mut Host,
    /// Maximum event-loop iterations.
    pub maximum_iterations: u64,
}

/// Runs one bounded terminal attachment. This function does not enter raw mode.
///
/// The caller must use [`crate::TerminalLifecycleGuard`] around this function and
/// pass the resulting cleanup observation to receipt projection.
///
/// # Errors
///
/// Returns [`TerminalHostError`] for invalid limits, connection failure, rendering failure,
/// or when the finite iteration bound expires.
#[allow(
    clippy::needless_pass_by_value,
    reason = "the request is the visible composition boundary for borrowed host capabilities"
)]
pub fn run_attached_terminal<Host, Connection>(
    request: LiveTerminalRequest<'_, '_, '_, Host, Connection>,
) -> Result<(crate::TerminalClientState, crate::TerminalExit), TerminalHostError>
where
    Host: TerminalHostPort,
    Connection: crate::TerminalConnectionPort,
{
    let activation = crate::preflight_live_terminal_activation()
        .map_err(|_error| TerminalHostError::Activation)?;
    run_attached_terminal_with_activation(request, &activation)
}

/// Runs one bounded terminal attachment after an explicit activation decision.
///
/// No host or connection port is called when activation is blocked.
///
/// # Errors
///
/// Returns [`TerminalHostError`] for blocked activation, invalid limits, connection failure,
/// rendering failure, or when the finite iteration bound expires.
#[allow(
    clippy::needless_pass_by_value,
    reason = "the request is the visible composition boundary for borrowed host capabilities"
)]
pub fn run_attached_terminal_with_activation<Host, Connection>(
    request: LiveTerminalRequest<'_, '_, '_, Host, Connection>,
    activation: &animus_runtime::composition_activation_graph::ActivationDecision,
) -> Result<(crate::TerminalClientState, crate::TerminalExit), TerminalHostError>
where
    Host: TerminalHostPort,
    Connection: crate::TerminalConnectionPort,
{
    let expected_activation = crate::preflight_live_terminal_activation()
        .map_err(|_error| TerminalHostError::Activation)?;
    if activation != &expected_activation || !activation.effects_admitted() {
        return Err(TerminalHostError::Activation);
    }
    if request.maximum_iterations == 0 {
        return Err(TerminalHostError::Bound);
    }
    let limits = limits_from_config(request.config);
    let viewport = request.host.viewport()?;
    let snapshot = request
        .connection
        .attach(
            request.config.root_id(),
            request.config.client_id(),
            request.config.max_event_batch(),
        )
        .map_err(TerminalHostError::Connection)?;
    let generation = snapshot.generation;
    let result = (|| {
        let mut state = crate::TerminalClientState::from_snapshot(&snapshot, viewport, limits)
            .map_err(TerminalHostError::Core)?;
        request.host.render(state.view())?;
        state.record_rendered().map_err(TerminalHostError::Core)?;

        for _iteration in 0..request.maximum_iterations {
            let input = request.host.poll_input(request.config.poll_interval_ms())?;
            let events = request
                .connection
                .events_after(
                    request.config.root_id(),
                    state.last_sequence(),
                    request.config.max_event_batch(),
                )
                .map_err(TerminalHostError::Connection)?;
            let plan = crate::reduce_iteration(
                &mut state,
                &crate::TerminalIteration {
                    input,
                    session_events: events.as_slice(),
                    cancelled: request.host.cancelled(),
                    signalled: request.host.signalled(),
                },
                limits,
            )
            .map_err(TerminalHostError::Core)?;
            if let Some(command) = &plan.command {
                match command {
                    crate::ClientCommand::Session {
                        action: crate::SessionAction::RefreshSnapshot,
                        ..
                    } => {
                        let refreshed = request
                            .connection
                            .refresh(
                                request.config.root_id(),
                                request.config.client_id(),
                                request.config.max_event_batch(),
                            )
                            .map_err(TerminalHostError::Connection)?;
                        state
                            .apply_refresh(&refreshed, request.config.max_event_batch())
                            .map_err(TerminalHostError::Core)?;
                    }
                    crate::ClientCommand::Session {
                        action: crate::SessionAction::Detach,
                        ..
                    } => {}
                    crate::ClientCommand::Session {
                        action: crate::SessionAction::Stop,
                        ..
                    } => request
                        .connection
                        .stop(request.config.root_id(), generation)
                        .map_err(TerminalHostError::Connection)?,
                    _other => {
                        request
                            .connection
                            .route(request.config.root_id(), generation, command)
                            .map_err(TerminalHostError::Connection)?;
                    }
                }
            }
            if plan.render {
                request.host.render(state.view())?;
                state.record_rendered().map_err(TerminalHostError::Core)?;
            }
            if let Some(exit) = plan.exit {
                return Ok((state, exit));
            }
        }
        Err(TerminalHostError::IterationLimit)
    })();
    request
        .connection
        .detach(request.config.root_id(), request.config.client_id())
        .map_err(TerminalHostError::Connection)?;
    result
}

fn limits_from_config(config: &crate::ValidatedTerminalConfig) -> crate::TerminalLimits {
    crate::TerminalLimits {
        event_batch: config.max_event_batch(),
        prompt_bytes: config.max_prompt_bytes(),
        width: config.max_width(),
        height: config.max_height(),
    }
}

/// Bounded live-terminal host failure.
#[derive(Debug)]
pub enum TerminalHostError {
    /// The selected composition failed activation preflight.
    Activation,
    /// A configured loop limit was zero or exceeded.
    Bound,
    /// Pure client reduction failed.
    Core(crate::TerminalCoreError),
    /// Selected daemon or direct connection failed.
    Connection(crate::TerminalConnectionError),
    /// Terminal input, dimensions, or backend I/O failed.
    Terminal,
    /// The finite iteration count ended without an exit observation.
    IterationLimit,
}

impl core::fmt::Display for TerminalHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live terminal host failed: {self:?}")
    }
}

impl std::error::Error for TerminalHostError {}

/// Real Crossterm event and Ratatui backend shell.
#[cfg(feature = "live-terminal")]
pub struct CrosstermTerminalHost<Backend: ratatui::backend::Backend> {
    terminal: ratatui::Terminal<Backend>,
    cancelled: bool,
    signalled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(feature = "live-terminal")]
impl<Backend: ratatui::backend::Backend> CrosstermTerminalHost<Backend> {
    /// Builds a terminal host from an already-created backend and signal flag.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalHostError`] when Ratatui cannot create its terminal state.
    pub fn new(
        backend: Backend,
        signalled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<Self, TerminalHostError> {
        let terminal =
            ratatui::Terminal::new(backend).map_err(|_error| TerminalHostError::Terminal)?;
        Ok(Self {
            terminal,
            cancelled: false,
            signalled,
        })
    }
}

#[cfg(feature = "live-terminal")]
impl<Backend: ratatui::backend::Backend> TerminalHostPort for CrosstermTerminalHost<Backend> {
    fn viewport(&mut self) -> Result<(u16, u16), TerminalHostError> {
        let area = self
            .terminal
            .size()
            .map_err(|_error| TerminalHostError::Terminal)?;
        Ok((area.width, area.height))
    }

    fn poll_input(
        &mut self,
        timeout_ms: u64,
    ) -> Result<Option<crate::TerminalInput>, TerminalHostError> {
        let ready = crossterm::event::poll(std::time::Duration::from_millis(timeout_ms))
            .map_err(|_error| TerminalHostError::Terminal)?;
        if !ready {
            return Ok(None);
        }
        let event = crossterm::event::read().map_err(|_error| TerminalHostError::Terminal)?;
        Ok(map_crossterm_event(event, &mut self.cancelled))
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn signalled(&self) -> bool {
        self.signalled.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn render(&mut self, view: &crate::TerminalViewState) -> Result<(), TerminalHostError> {
        crate::render_terminal_view(&mut self.terminal, view)
            .map_err(|_error| TerminalHostError::Terminal)
    }
}

#[cfg(feature = "live-terminal")]
fn map_crossterm_event(
    event: crossterm::event::Event,
    cancelled: &mut bool,
) -> Option<crate::TerminalInput> {
    match event {
        crossterm::event::Event::Resize(width, height) => {
            Some(crate::TerminalInput::Resize { width, height })
        }
        crossterm::event::Event::Key(key) if key.is_press() => match key.code {
            crossterm::event::KeyCode::Char('c')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                *cancelled = true;
                None
            }
            crossterm::event::KeyCode::Char('q')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(crate::TerminalInput::Quit)
            }
            crossterm::event::KeyCode::Char(character)
                if key.modifiers.is_empty()
                    || key.modifiers == crossterm::event::KeyModifiers::SHIFT =>
            {
                Some(crate::TerminalInput::Character(character))
            }
            crossterm::event::KeyCode::Backspace => Some(crate::TerminalInput::Backspace),
            crossterm::event::KeyCode::Enter => Some(crate::TerminalInput::Submit),
            crossterm::event::KeyCode::Esc => Some(crate::TerminalInput::Cancel),
            crossterm::event::KeyCode::Left => Some(crate::TerminalInput::Left),
            crossterm::event::KeyCode::Right => Some(crate::TerminalInput::Right),
            crossterm::event::KeyCode::Home => Some(crate::TerminalInput::Home),
            crossterm::event::KeyCode::End => Some(crate::TerminalInput::End),
            _other => None,
        },
        _other => None,
    }
}
