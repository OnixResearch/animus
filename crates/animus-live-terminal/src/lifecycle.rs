//! Idempotent partial-start and full-start terminal lifecycle guard.

#[cfg(feature = "live-terminal")]
use std::io::Write;
#[cfg(feature = "live-terminal")]
use std::sync::{Arc, atomic::AtomicBool};

/// Shell-owned terminal operations. Tests can bind this port to a pseudo-terminal.
#[allow(
    clippy::missing_errors_doc,
    reason = "each lifecycle method returns its exact typed transition failure"
)]
pub trait TerminalLifecyclePort {
    /// Enables raw terminal input.
    fn enable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Enters the alternate screen.
    fn enter_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Hides the cursor.
    fn hide_cursor(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Verifies backend construction after terminal state changes.
    fn backend_ready(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Registers supported shutdown signals.
    fn register_signals(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Unregisters supported shutdown signals.
    fn unregister_signals(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Releases or clears the backend.
    fn release_backend(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Shows the cursor.
    fn show_cursor(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Leaves the alternate screen.
    fn leave_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure>;
    /// Disables raw terminal input.
    fn disable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure>;
}

/// One lifecycle operation failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalLifecycleFailure {
    /// Raw-mode transition failed.
    RawMode,
    /// Alternate-screen transition failed.
    AlternateScreen,
    /// Cursor transition failed.
    Cursor,
    /// Backend construction or release failed.
    Backend,
    /// Supported signal registration failed.
    Signal,
}

/// Start or cleanup failure with explicit cleanup observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalLifecycleError {
    /// Operation that first failed.
    pub failure: TerminalLifecycleFailure,
    /// Cleanup attempted for every completed transition.
    pub cleanup: crate::CleanupObservation,
}

impl core::fmt::Display for TerminalLifecycleError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "terminal lifecycle failed: {:?}", self.failure)
    }
}

impl std::error::Error for TerminalLifecycleError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompletedTransition {
    RawMode,
    AlternateScreen,
    Cursor,
    Backend,
    Signals,
}

/// Guard that restores every terminal transition completed during startup.
pub struct TerminalLifecycleGuard<'port, Port: TerminalLifecyclePort> {
    port: &'port mut Port,
    completed: Vec<CompletedTransition>,
    restored: bool,
}

impl<'port, Port: TerminalLifecyclePort> TerminalLifecycleGuard<'port, Port> {
    /// Runs ordered terminal startup and restores partial state on failure.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalLifecycleError`] with cleanup observations when any startup step fails.
    pub fn enter(port: &'port mut Port) -> Result<Self, TerminalLifecycleError> {
        let mut completed = Vec::with_capacity(5);
        apply_step(
            port.enable_raw_mode(),
            CompletedTransition::RawMode,
            &mut completed,
            port,
        )?;
        apply_step(
            port.enter_alternate_screen(),
            CompletedTransition::AlternateScreen,
            &mut completed,
            port,
        )?;
        apply_step(
            port.hide_cursor(),
            CompletedTransition::Cursor,
            &mut completed,
            port,
        )?;
        apply_step(
            port.backend_ready(),
            CompletedTransition::Backend,
            &mut completed,
            port,
        )?;
        apply_step(
            port.register_signals(),
            CompletedTransition::Signals,
            &mut completed,
            port,
        )?;
        Ok(Self {
            port,
            completed,
            restored: false,
        })
    }

    /// Returns mutable access to the terminal operation adapter while the guard is active.
    #[must_use]
    pub fn port(&mut self) -> &mut Port {
        self.port
    }

    /// Attempts every required inverse action exactly once.
    ///
    /// # Errors
    ///
    /// Returns the first cleanup failure and the complete cleanup observation.
    pub fn restore(&mut self) -> Result<crate::CleanupObservation, TerminalLifecycleError> {
        if self.restored {
            let count = u32::try_from(self.completed.len()).unwrap_or(u32::MAX);
            return Ok(crate::CleanupObservation {
                completed_transitions: count,
                attempted_actions: count,
                completed_actions: count,
                failed_actions: 0,
            });
        }
        self.restored = true;
        restore_completed(self.port, self.completed.as_slice())
    }
}

impl<Port: TerminalLifecyclePort> Drop for TerminalLifecycleGuard<'_, Port> {
    fn drop(&mut self) {
        if !self.restored {
            self.restored = true;
            let _cleanup = restore_completed(self.port, self.completed.as_slice());
        }
    }
}

fn apply_step<Port: TerminalLifecyclePort>(
    result: Result<(), TerminalLifecycleFailure>,
    transition: CompletedTransition,
    completed: &mut Vec<CompletedTransition>,
    port: &mut Port,
) -> Result<(), TerminalLifecycleError> {
    match result {
        Ok(()) => {
            completed.push(transition);
            Ok(())
        }
        Err(failure) => {
            let cleanup = match restore_completed(port, completed.as_slice()) {
                Ok(observation) => observation,
                Err(error) => error.cleanup,
            };
            Err(TerminalLifecycleError { failure, cleanup })
        }
    }
}

fn restore_completed<Port: TerminalLifecyclePort>(
    port: &mut Port,
    completed: &[CompletedTransition],
) -> Result<crate::CleanupObservation, TerminalLifecycleError> {
    let completed_transitions = u32::try_from(completed.len()).unwrap_or(u32::MAX);
    let mut observation = crate::CleanupObservation {
        completed_transitions,
        attempted_actions: 0,
        completed_actions: 0,
        failed_actions: 0,
    };
    let mut first_failure = None;
    for transition in completed.iter().rev() {
        observation.attempted_actions = observation.attempted_actions.saturating_add(1);
        let result = match transition {
            CompletedTransition::Signals => port.unregister_signals(),
            CompletedTransition::Backend => port.release_backend(),
            CompletedTransition::Cursor => port.show_cursor(),
            CompletedTransition::AlternateScreen => port.leave_alternate_screen(),
            CompletedTransition::RawMode => port.disable_raw_mode(),
        };
        match result {
            Ok(()) => {
                observation.completed_actions = observation.completed_actions.saturating_add(1);
            }
            Err(failure) => {
                observation.failed_actions = observation.failed_actions.saturating_add(1);
                first_failure.get_or_insert(failure);
            }
        }
    }
    if let Some(failure) = first_failure {
        Err(TerminalLifecycleError {
            failure,
            cleanup: observation,
        })
    } else {
        Ok(observation)
    }
}

/// Real Crossterm lifecycle adapter for the controlling terminal.
#[cfg(feature = "live-terminal")]
pub struct CrosstermTerminalLifecycle<Writer: Write> {
    writer: Writer,
    signal_flag: Arc<AtomicBool>,
    signal_ids: Vec<signal_hook::SigId>,
}

#[cfg(feature = "live-terminal")]
impl<Writer: Write> CrosstermTerminalLifecycle<Writer> {
    /// Builds an inactive terminal lifecycle adapter around an explicit writer.
    #[must_use]
    pub fn new(writer: Writer) -> Self {
        Self {
            writer,
            signal_flag: Arc::new(AtomicBool::new(false)),
            signal_ids: Vec::new(),
        }
    }

    /// Returns the supported-signal shutdown flag.
    #[must_use]
    pub fn signal_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.signal_flag)
    }

    /// Returns mutable access to the terminal writer for Ratatui construction.
    #[must_use]
    pub fn writer_mut(&mut self) -> &mut Writer {
        &mut self.writer
    }
}

#[cfg(feature = "live-terminal")]
impl<Writer: Write> TerminalLifecyclePort for CrosstermTerminalLifecycle<Writer> {
    fn enable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::terminal::enable_raw_mode().map_err(|_error| TerminalLifecycleFailure::RawMode)
    }

    fn enter_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::execute!(self.writer, crossterm::terminal::EnterAlternateScreen)
            .map_err(|_error| TerminalLifecycleFailure::AlternateScreen)
    }

    fn hide_cursor(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::execute!(self.writer, crossterm::cursor::Hide)
            .map_err(|_error| TerminalLifecycleFailure::Cursor)
    }

    fn backend_ready(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::terminal::size()
            .map(|_size| ())
            .map_err(|_error| TerminalLifecycleFailure::Backend)
    }

    fn register_signals(&mut self) -> Result<(), TerminalLifecycleFailure> {
        for signal in signal_hook::consts::TERM_SIGNALS {
            let id = signal_hook::flag::register(*signal, Arc::clone(&self.signal_flag))
                .map_err(|_error| TerminalLifecycleFailure::Signal)?;
            self.signal_ids.push(id);
        }
        Ok(())
    }

    fn unregister_signals(&mut self) -> Result<(), TerminalLifecycleFailure> {
        let mut complete = true;
        for id in self.signal_ids.drain(..) {
            complete &= signal_hook::low_level::unregister(id);
        }
        if complete {
            Ok(())
        } else {
            Err(TerminalLifecycleFailure::Signal)
        }
    }

    fn release_backend(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.writer
            .flush()
            .map_err(|_error| TerminalLifecycleFailure::Backend)
    }

    fn show_cursor(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::execute!(self.writer, crossterm::cursor::Show)
            .map_err(|_error| TerminalLifecycleFailure::Cursor)
    }

    fn leave_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::execute!(self.writer, crossterm::terminal::LeaveAlternateScreen)
            .map_err(|_error| TerminalLifecycleFailure::AlternateScreen)
    }

    fn disable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure> {
        crossterm::terminal::disable_raw_mode().map_err(|_error| TerminalLifecycleFailure::RawMode)
    }
}
