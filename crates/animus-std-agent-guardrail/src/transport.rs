use std::ffi::OsString;
use std::io::{Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, OwnedFd};
use std::path::Path;
use std::process::{Command, ExitStatus};

use crate::core::executable_identity;
use crate::model::{
    BUBBLEWRAP_COHORT, ExecutableFacts, ExitClass, GuardrailError, GuardrailErrorKind,
    GuardrailPlan, PlanOperation,
};

const FIELD_TRANSPORT: &str = "sealed-transport";
const FIELD_EXECUTABLE: &str = "executable";
const FIELD_LAUNCH: &str = "launch";
const MEMFD_NAME: &str = "animus-agent-guardrail-options";
const BWRAP_VERSION_ARGUMENT: &str = "--version";
const BWRAP_ARGS_ARGUMENT: &str = "--args";
const COMMAND_SEPARATOR: &str = "--";
const EXPECTED_VERSION: &str = "bubblewrap 0.11.2";
const NUL_BYTE: u8 = 0;
const STANDARD_ERROR_FD: i32 = 2;
const LINUX_CLOSE_ON_EXEC_FLAG: u32 = 0o2_000_000;

/// Final guarded process observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LaunchObservation {
    /// Child exit class.
    pub exit: ExitClass,
    /// Process exit code when available.
    pub exit_code: Option<i32>,
    /// Number of finalized options carried through the sealed descriptor.
    pub option_count: usize,
}

/// Converts a typed plan to finalized Bubblewrap options.
///
/// Host source paths and environment values remain ephemeral and are never receipt fields.
#[must_use]
pub fn finalize_options(plan: &GuardrailPlan) -> Vec<OsString> {
    let mut options = vec![
        OsString::from("--die-with-parent"),
        OsString::from("--new-session"),
        OsString::from("--unshare-all"),
    ];
    for operation in &plan.operations {
        match operation {
            PlanOperation::PrivateRoot => {
                options.push(OsString::from("--tmpfs"));
                options.push(OsString::from("/"));
            }
            PlanOperation::ReadOnlyBind { source, target } => {
                options.push(OsString::from("--ro-bind"));
                options.push(source.as_os_str().to_owned());
                options.push(target.as_os_str().to_owned());
            }
            PlanOperation::WritableBind { source, target } => {
                options.push(OsString::from("--bind"));
                options.push(source.as_os_str().to_owned());
                options.push(target.as_os_str().to_owned());
            }
            PlanOperation::Proc => {
                options.push(OsString::from("--proc"));
                options.push(OsString::from("/proc"));
            }
            PlanOperation::PrivateTmp => {
                options.push(OsString::from("--tmpfs"));
                options.push(OsString::from("/tmp"));
            }
            PlanOperation::PrivateDev => {
                options.push(OsString::from("--tmpfs"));
                options.push(OsString::from("/dev"));
            }
            PlanOperation::ClearEnvironment => options.push(OsString::from("--clearenv")),
            PlanOperation::SetEnvironment { key, value } => {
                options.push(OsString::from("--setenv"));
                options.push(OsString::from(key));
                options.push(OsString::from(value));
            }
            PlanOperation::ShareNetwork => options.push(OsString::from("--share-net")),
            PlanOperation::LockRoot => {
                options.push(OsString::from("--remount-ro"));
                options.push(OsString::from("/"));
            }
        }
    }
    options
}

/// Encodes finalized options as one NUL-delimited byte vector.
///
/// # Errors
///
/// Returns [`GuardrailError`] if an option contains NUL or attempts to inject parser controls.
pub fn encode_options(options: &[OsString]) -> Result<Vec<u8>, GuardrailError> {
    let mut bytes = Vec::new();
    for option in options {
        let value = option.to_string_lossy();
        if value.contains(char::from(NUL_BYTE))
            || value == BWRAP_ARGS_ARGUMENT
            || value == COMMAND_SEPARATOR
        {
            return Err(GuardrailError::new(
                GuardrailErrorKind::Transport,
                FIELD_TRANSPORT,
            ));
        }
        bytes.extend_from_slice(value.as_bytes());
        bytes.push(NUL_BYTE);
    }
    Ok(bytes)
}

/// Creates, writes, rewinds, and seals the anonymous option descriptor.
///
/// # Errors
///
/// Returns [`GuardrailError`] when memfd creation, writing, rewind, or sealing fails.
#[cfg(target_os = "linux")]
pub fn prepare_sealed_options(options: &[OsString]) -> Result<std::fs::File, GuardrailError> {
    use rustix::fs::{MemfdFlags, SealFlags, fcntl_add_seals, fcntl_get_seals, memfd_create};

    let payload = encode_options(options)?;
    let fd: OwnedFd = memfd_create(MEMFD_NAME, MemfdFlags::ALLOW_SEALING)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    let mut file = std::fs::File::from(fd);
    file.write_all(payload.as_slice())
        .and_then(|()| file.flush())
        .and_then(|()| file.seek(SeekFrom::Start(0)).map(|_offset| ()))
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    let required = SealFlags::SHRINK | SealFlags::GROW | SealFlags::WRITE | SealFlags::SEAL;
    fcntl_add_seals(&file, required)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    let observed = fcntl_get_seals(&file)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    if !observed.contains(required) {
        return Err(GuardrailError::new(
            GuardrailErrorKind::Transport,
            FIELD_TRANSPORT,
        ));
    }
    Ok(file)
}

/// Non-Linux sealed transport is explicitly unavailable.
#[cfg(not(target_os = "linux"))]
pub fn prepare_sealed_options(_options: &[OsString]) -> Result<std::fs::File, GuardrailError> {
    Err(GuardrailError::new(
        GuardrailErrorKind::Transport,
        FIELD_TRANSPORT,
    ))
}

/// Observes exact executable bytes and the first-tranche Bubblewrap cohort.
///
/// # Errors
///
/// Returns [`GuardrailError`] for a non-absolute or unreadable executable, probe failure, or an
/// incompatible version/`--args` cohort.
pub fn observe_executable(path: &Path) -> Result<ExecutableFacts, GuardrailError> {
    if !path.is_absolute() {
        return Err(GuardrailError::new(
            GuardrailErrorKind::Executable,
            FIELD_EXECUTABLE,
        ));
    }
    let bytes = std::fs::read(path)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Executable, FIELD_EXECUTABLE))?;
    let output = Command::new(path)
        .arg(BWRAP_VERSION_ARGUMENT)
        .env_clear()
        .output()
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Executable, FIELD_EXECUTABLE))?;
    let version = String::from_utf8(output.stdout)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Executable, FIELD_EXECUTABLE))?;
    if !output.status.success() || version.trim() != EXPECTED_VERSION {
        return Err(GuardrailError::new(
            GuardrailErrorKind::Executable,
            FIELD_EXECUTABLE,
        ));
    }
    Ok(ExecutableFacts {
        path: path.to_path_buf(),
        identity: executable_identity(bytes.as_slice()),
        cohort: String::from(BUBBLEWRAP_COHORT),
        args_supported: true,
    })
}

/// Launches only through `bwrap --args FD -- COMMAND`; there is no argv or unguarded fallback.
///
/// # Errors
///
/// Returns [`GuardrailError`] before child execution for invalid command input, sealed transport
/// failure, or spawn/wait failure.
#[cfg(target_os = "linux")]
pub fn launch(
    executable: &ExecutableFacts,
    plan: &GuardrailPlan,
    command: &[OsString],
) -> Result<LaunchObservation, GuardrailError> {
    if command.is_empty() || !executable.path.is_absolute() {
        return Err(GuardrailError::new(
            GuardrailErrorKind::Launch,
            FIELD_LAUNCH,
        ));
    }
    validate_inherited_descriptors()?;
    let options = finalize_options(plan);
    let descriptor = prepare_sealed_options(options.as_slice())?;
    let descriptor_number = descriptor.as_raw_fd().to_string();
    let status = Command::new(executable.path.as_path())
        .arg(BWRAP_ARGS_ARGUMENT)
        .arg(descriptor_number)
        .arg(COMMAND_SEPARATOR)
        .args(command)
        .env_clear()
        .status()
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Launch, FIELD_LAUNCH))?;
    Ok(LaunchObservation {
        exit: classify_exit(status),
        exit_code: status.code(),
        option_count: options.len(),
    })
}

/// Non-Linux launch is explicitly unavailable.
#[cfg(not(target_os = "linux"))]
pub fn launch(
    _executable: &ExecutableFacts,
    _plan: &GuardrailPlan,
    _command: &[OsString],
) -> Result<LaunchObservation, GuardrailError> {
    Err(GuardrailError::new(
        GuardrailErrorKind::Launch,
        FIELD_LAUNCH,
    ))
}

#[cfg(target_os = "linux")]
fn validate_inherited_descriptors() -> Result<(), GuardrailError> {
    let descriptor_directory = format!("/proc/{}/fd", std::process::id());
    let entries = std::fs::read_dir(descriptor_directory.as_str())
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    for entry in entries {
        let entry = entry.map_err(|_error| {
            GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT)
        })?;
        let Some(descriptor) = entry
            .file_name()
            .to_str()
            .and_then(|value| value.parse::<i32>().ok())
        else {
            return Err(GuardrailError::new(
                GuardrailErrorKind::Transport,
                FIELD_TRANSPORT,
            ));
        };
        if descriptor <= STANDARD_ERROR_FD {
            continue;
        }
        let target = std::fs::read_link(entry.path()).map_err(|_error| {
            GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT)
        })?;
        if target == std::path::Path::new(descriptor_directory.as_str())
            || descriptor_is_close_on_exec(descriptor)?
        {
            continue;
        }
        return Err(GuardrailError::new(
            GuardrailErrorKind::Transport,
            FIELD_TRANSPORT,
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn descriptor_is_close_on_exec(descriptor: i32) -> Result<bool, GuardrailError> {
    let info = std::fs::read_to_string(format!("/proc/self/fdinfo/{descriptor}"))
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    let flags = info
        .lines()
        .find_map(|line| line.strip_prefix("flags:\t"))
        .and_then(|value| u32::from_str_radix(value, 8).ok())
        .ok_or_else(|| GuardrailError::new(GuardrailErrorKind::Transport, FIELD_TRANSPORT))?;
    Ok(flags & LINUX_CLOSE_ON_EXEC_FLAG == LINUX_CLOSE_ON_EXEC_FLAG)
}

fn classify_exit(status: ExitStatus) -> ExitClass {
    if status.success() {
        ExitClass::Success
    } else if status.code().is_some() {
        ExitClass::NonZero
    } else {
        ExitClass::Signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_encoding_is_nul_delimited_and_rejects_parser_controls() {
        let encoded = encode_options(&[OsString::from("--ro-bind"), OsString::from("/source")])
            .expect("encode");
        assert_eq!(encoded, b"--ro-bind\0/source\0");
        assert!(encode_options(&[OsString::from("--args")]).is_err());
        assert!(encode_options(&[OsString::from("--")]).is_err());
        assert!(encode_options(&[OsString::from("bad\0value")]).is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn option_descriptor_is_rewound_and_sealed() {
        use std::io::Read;

        let mut file = prepare_sealed_options(&[OsString::from("--clearenv")]).expect("sealed");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .expect("read sealed descriptor");
        assert_eq!(bytes, b"--clearenv\0");
        assert!(file.write_all(b"mutation").is_err());
    }
}
