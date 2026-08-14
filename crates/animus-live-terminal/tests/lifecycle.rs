//! Lifecycle guard and pseudo-terminal restoration tests.

use animus_live_terminal::{
    TerminalLifecycleFailure, TerminalLifecycleGuard, TerminalLifecyclePort,
};
use std::os::fd::OwnedFd;

const ALT_ENTER: &[u8] = b"\x1b[?1049h";
const ALT_LEAVE: &[u8] = b"\x1b[?1049l";
const CURSOR_HIDE: &[u8] = b"\x1b[?25l";
const CURSOR_SHOW: &[u8] = b"\x1b[?25h";
const PTY_WIDTH: u16 = 80;
const PTY_HEIGHT: u16 = 24;
const OUTPUT_BUFFER_BYTES: usize = 128;
const EXPECTED_OUTPUT_BYTES: usize =
    ALT_ENTER.len() + ALT_LEAVE.len() + CURSOR_HIDE.len() + CURSOR_SHOW.len();
#[cfg(feature = "live-terminal")]
const SIGNAL_WAIT_ATTEMPTS: usize = 100;
#[cfg(feature = "live-terminal")]
const SIGNAL_WAIT_MILLISECONDS: u64 = 1;

#[test]
fn full_start_and_idempotent_restore_reverse_every_transition() {
    let mut port = MockPort::default();
    let mut guard = TerminalLifecycleGuard::enter(&mut port).expect("full start");
    let cleanup = guard.restore().expect("complete cleanup");
    let repeated = guard.restore().expect("idempotent cleanup");
    drop(guard);

    assert_eq!(cleanup.completed_transitions, 5);
    assert_eq!(cleanup.attempted_actions, 5);
    assert_eq!(cleanup.completed_actions, 5);
    assert_eq!(cleanup.failed_actions, 0);
    assert_eq!(cleanup, repeated);
    assert_eq!(
        port.calls,
        [
            "raw-on",
            "alternate-on",
            "cursor-hide",
            "backend-on",
            "signals-on",
            "signals-off",
            "backend-off",
            "cursor-show",
            "alternate-off",
            "raw-off",
        ]
    );
}

#[test]
fn partial_start_and_cleanup_failure_remain_explicit() {
    let mut partial = MockPort {
        startup_failure: Some(TerminalLifecycleFailure::Backend),
        ..MockPort::default()
    };
    let error = match TerminalLifecycleGuard::enter(&mut partial) {
        Ok(_guard) => panic!("backend failure must reject startup"),
        Err(error) => error,
    };
    assert_eq!(error.failure, TerminalLifecycleFailure::Backend);
    assert_eq!(error.cleanup.completed_transitions, 3);
    assert_eq!(error.cleanup.completed_actions, 3);
    assert_eq!(partial.calls.last(), Some(&"raw-off"));

    let mut cleanup_failure = MockPort {
        cleanup_failure: Some(TerminalLifecycleFailure::Cursor),
        ..MockPort::default()
    };
    let mut guard = TerminalLifecycleGuard::enter(&mut cleanup_failure).expect("startup");
    let error = guard
        .restore()
        .expect_err("cleanup failure must stay visible");
    assert_eq!(error.failure, TerminalLifecycleFailure::Cursor);
    assert_eq!(error.cleanup.attempted_actions, 5);
    assert_eq!(error.cleanup.completed_actions, 4);
    assert_eq!(error.cleanup.failed_actions, 1);
}

#[cfg(feature = "live-terminal")]
#[test]
fn supported_signal_sets_the_explicit_shutdown_observation() {
    let mut lifecycle = animus_live_terminal::CrosstermTerminalLifecycle::new(Vec::new());
    let signal_flag = lifecycle.signal_flag();
    lifecycle.register_signals().expect("register signals");
    nix::sys::signal::kill(nix::unistd::Pid::this(), nix::sys::signal::Signal::SIGTERM)
        .expect("deliver supported signal");
    for _attempt in 0..SIGNAL_WAIT_ATTEMPTS {
        if signal_flag.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(SIGNAL_WAIT_MILLISECONDS));
    }
    assert!(signal_flag.load(std::sync::atomic::Ordering::SeqCst));
    lifecycle.unregister_signals().expect("unregister signals");
}

#[test]
fn pseudo_terminal_raw_input_and_terminal_state_are_restored() {
    let size = nix::pty::Winsize {
        ws_row: PTY_HEIGHT,
        ws_col: PTY_WIDTH,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let pair = nix::pty::openpty(Some(&size), None).expect("pseudo terminal");
    let before = nix::sys::termios::tcgetattr(&pair.slave).expect("initial termios");
    let mut port = PseudoTerminalPort::new(pair.master, pair.slave);
    let mut guard = TerminalLifecycleGuard::enter(&mut port).expect("pseudo-terminal startup");
    guard.port().write_input(b"x").expect("write input");
    assert_eq!(guard.port().read_input().expect("read input"), b'x');
    let during = guard.port().termios().expect("raw termios");
    assert_ne!(before, during);
    let cleanup = guard.restore().expect("pseudo-terminal cleanup");
    drop(guard);

    assert_eq!(cleanup.failed_actions, 0);
    assert_eq!(port.termios().expect("restored termios"), before);
    let output = port.read_output().expect("terminal output");
    assert!(
        output
            .windows(ALT_ENTER.len())
            .any(|part| part == ALT_ENTER)
    );
    assert!(
        output
            .windows(ALT_LEAVE.len())
            .any(|part| part == ALT_LEAVE)
    );
    assert!(
        output
            .windows(CURSOR_HIDE.len())
            .any(|part| part == CURSOR_HIDE)
    );
    assert!(
        output
            .windows(CURSOR_SHOW.len())
            .any(|part| part == CURSOR_SHOW)
    );
}

#[derive(Default)]
struct MockPort {
    calls: Vec<&'static str>,
    startup_failure: Option<TerminalLifecycleFailure>,
    cleanup_failure: Option<TerminalLifecycleFailure>,
}

impl MockPort {
    fn startup(
        &mut self,
        call: &'static str,
        class: TerminalLifecycleFailure,
    ) -> Result<(), TerminalLifecycleFailure> {
        self.calls.push(call);
        if self.startup_failure == Some(class) {
            Err(class)
        } else {
            Ok(())
        }
    }

    fn cleanup(
        &mut self,
        call: &'static str,
        class: TerminalLifecycleFailure,
    ) -> Result<(), TerminalLifecycleFailure> {
        self.calls.push(call);
        if self.cleanup_failure == Some(class) {
            Err(class)
        } else {
            Ok(())
        }
    }
}

impl TerminalLifecyclePort for MockPort {
    fn enable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.startup("raw-on", TerminalLifecycleFailure::RawMode)
    }

    fn enter_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.startup("alternate-on", TerminalLifecycleFailure::AlternateScreen)
    }

    fn hide_cursor(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.startup("cursor-hide", TerminalLifecycleFailure::Cursor)
    }

    fn backend_ready(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.startup("backend-on", TerminalLifecycleFailure::Backend)
    }

    fn register_signals(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.startup("signals-on", TerminalLifecycleFailure::Signal)
    }

    fn unregister_signals(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.cleanup("signals-off", TerminalLifecycleFailure::Signal)
    }

    fn release_backend(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.cleanup("backend-off", TerminalLifecycleFailure::Backend)
    }

    fn show_cursor(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.cleanup("cursor-show", TerminalLifecycleFailure::Cursor)
    }

    fn leave_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.cleanup("alternate-off", TerminalLifecycleFailure::AlternateScreen)
    }

    fn disable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.cleanup("raw-off", TerminalLifecycleFailure::RawMode)
    }
}

struct PseudoTerminalPort {
    master: OwnedFd,
    slave: OwnedFd,
    original: Option<nix::sys::termios::Termios>,
}

impl PseudoTerminalPort {
    const fn new(master: OwnedFd, slave: OwnedFd) -> Self {
        Self {
            master,
            slave,
            original: None,
        }
    }

    fn write_input(&self, input: &[u8]) -> Result<(), TerminalLifecycleFailure> {
        nix::unistd::write(&self.master, input)
            .map(|_written| ())
            .map_err(|_error| TerminalLifecycleFailure::Backend)
    }

    fn read_input(&self) -> Result<u8, TerminalLifecycleFailure> {
        let mut input = [0_u8; 1];
        nix::unistd::read(&self.slave, &mut input)
            .map_err(|_error| TerminalLifecycleFailure::Backend)?;
        Ok(input[0])
    }

    fn termios(&self) -> Result<nix::sys::termios::Termios, TerminalLifecycleFailure> {
        nix::sys::termios::tcgetattr(&self.slave)
            .map_err(|_error| TerminalLifecycleFailure::RawMode)
    }

    fn read_output(&self) -> Result<Vec<u8>, TerminalLifecycleFailure> {
        let mut output = Vec::with_capacity(EXPECTED_OUTPUT_BYTES);
        while output.len() < EXPECTED_OUTPUT_BYTES {
            let mut chunk = [0_u8; OUTPUT_BUFFER_BYTES];
            let read = nix::unistd::read(&self.master, &mut chunk)
                .map_err(|_error| TerminalLifecycleFailure::Backend)?;
            if read == 0 {
                return Err(TerminalLifecycleFailure::Backend);
            }
            output.extend_from_slice(&chunk[..read]);
        }
        Ok(output)
    }

    fn write_escape(&self, sequence: &[u8]) -> Result<(), TerminalLifecycleFailure> {
        nix::unistd::write(&self.slave, sequence)
            .map(|_written| ())
            .map_err(|_error| TerminalLifecycleFailure::AlternateScreen)
    }
}

impl TerminalLifecyclePort for PseudoTerminalPort {
    fn enable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure> {
        let original = self.termios()?;
        let mut raw = original.clone();
        nix::sys::termios::cfmakeraw(&mut raw);
        nix::sys::termios::tcsetattr(&self.slave, nix::sys::termios::SetArg::TCSANOW, &raw)
            .map_err(|_error| TerminalLifecycleFailure::RawMode)?;
        self.original = Some(original);
        Ok(())
    }

    fn enter_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.write_escape(ALT_ENTER)
    }

    fn hide_cursor(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.write_escape(CURSOR_HIDE)
            .map_err(|_error| TerminalLifecycleFailure::Cursor)
    }

    fn backend_ready(&mut self) -> Result<(), TerminalLifecycleFailure> {
        Ok(())
    }

    fn register_signals(&mut self) -> Result<(), TerminalLifecycleFailure> {
        Ok(())
    }

    fn unregister_signals(&mut self) -> Result<(), TerminalLifecycleFailure> {
        Ok(())
    }

    fn release_backend(&mut self) -> Result<(), TerminalLifecycleFailure> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.write_escape(CURSOR_SHOW)
            .map_err(|_error| TerminalLifecycleFailure::Cursor)
    }

    fn leave_alternate_screen(&mut self) -> Result<(), TerminalLifecycleFailure> {
        self.write_escape(ALT_LEAVE)
    }

    fn disable_raw_mode(&mut self) -> Result<(), TerminalLifecycleFailure> {
        let original = self
            .original
            .as_ref()
            .ok_or(TerminalLifecycleFailure::RawMode)?;
        nix::sys::termios::tcsetattr(&self.slave, nix::sys::termios::SetArg::TCSANOW, original)
            .map_err(|_error| TerminalLifecycleFailure::RawMode)
    }
}
