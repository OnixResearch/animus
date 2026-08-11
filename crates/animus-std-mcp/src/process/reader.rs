use std::io::{BufRead, BufReader, Read};
use std::process::{ChildStderr, ChildStdout};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::{self, JoinHandle};

use super::{ProcessError, StderrObservation};

const PENDING_FRAME_CAPACITY: usize = 8;
const STDERR_CHUNK_BYTES: usize = 4096;
const STDERR_DOMAIN: &str = "animus.mcp-client-host.stderr.v1";
const STDOUT_THREAD_NAME: &str = "animus-mcp-stdout";
const STDERR_THREAD_NAME: &str = "animus-mcp-stderr";

pub(crate) enum LineEvent {
    Frame(Vec<u8>),
    Error(ProcessError),
    Eof,
}

pub(crate) struct ReaderWorkers {
    pub(crate) lines: Receiver<LineEvent>,
    pub(crate) stderr: Receiver<StderrObservation>,
    pub(crate) stdout_done: Receiver<()>,
    pub(crate) stderr_done: Receiver<()>,
    pub(crate) stdout_thread: JoinHandle<()>,
    pub(crate) stderr_thread: JoinHandle<()>,
}

pub(crate) fn start_readers(
    stdout: ChildStdout,
    stderr: ChildStderr,
    max_frame_bytes: usize,
    max_diagnostic_bytes: usize,
) -> Result<ReaderWorkers, ProcessError> {
    let (line_sender, lines) = sync_channel(PENDING_FRAME_CAPACITY);
    let (stderr_sender, stderr_receiver) = sync_channel(1);
    let (stdout_done_sender, stdout_done) = sync_channel(1);
    let (stderr_done_sender, stderr_done) = sync_channel(1);
    let stdout_thread = thread::Builder::new()
        .name(String::from(STDOUT_THREAD_NAME))
        .spawn(move || {
            read_lines(stdout, max_frame_bytes, &line_sender);
            let _sent = stdout_done_sender.send(());
        })
        .map_err(|_| ProcessError::ReaderLaunch)?;
    let stderr_thread = thread::Builder::new()
        .name(String::from(STDERR_THREAD_NAME))
        .spawn(move || {
            read_stderr(stderr, max_diagnostic_bytes, &stderr_sender);
            let _sent = stderr_done_sender.send(());
        })
        .map_err(|_| ProcessError::ReaderLaunch)?;
    Ok(ReaderWorkers {
        lines,
        stderr: stderr_receiver,
        stdout_done,
        stderr_done,
        stdout_thread,
        stderr_thread,
    })
}

fn read_lines(stdout: ChildStdout, max_frame_bytes: usize, sender: &SyncSender<LineEvent>) {
    let mut reader = BufReader::new(stdout);
    loop {
        match read_bounded_line(&mut reader, max_frame_bytes) {
            Ok(Some(frame)) => {
                if sender.send(LineEvent::Frame(frame)).is_err() {
                    return;
                }
            }
            Ok(None) => {
                let _sent = sender.send(LineEvent::Eof);
                return;
            }
            Err(error) => {
                let _sent = sender.send(LineEvent::Error(error));
                return;
            }
        }
    }
}

fn read_bounded_line(
    reader: &mut impl BufRead,
    max_frame_bytes: usize,
) -> Result<Option<Vec<u8>>, ProcessError> {
    let mut frame = Vec::new();
    loop {
        let buffer = reader.fill_buf().map_err(|_| ProcessError::Read)?;
        if buffer.is_empty() {
            if frame.is_empty() {
                return Ok(None);
            }
            return Err(ProcessError::MalformedFrame);
        }
        let newline = buffer.iter().position(|byte| *byte == b'\n');
        let take_count = newline.unwrap_or(buffer.len());
        if frame.len().saturating_add(take_count) > max_frame_bytes {
            return Err(ProcessError::OversizedFrame);
        }
        frame.extend_from_slice(&buffer[..take_count]);
        let consume_count = take_count.saturating_add(usize::from(newline.is_some()));
        reader.consume(consume_count);
        if newline.is_some() {
            if frame.last() == Some(&b'\r') {
                frame.pop();
            }
            if frame.is_empty() || frame.contains(&0) {
                return Err(ProcessError::MalformedFrame);
            }
            return Ok(Some(frame));
        }
    }
}

fn read_stderr(
    mut stderr: ChildStderr,
    max_diagnostic_bytes: usize,
    sender: &SyncSender<StderrObservation>,
) {
    let mut hasher = blake3::Hasher::new();
    hasher.update(STDERR_DOMAIN.as_bytes());
    let mut byte_count = 0_usize;
    let mut truncated = false;
    let mut buffer = [0_u8; STDERR_CHUNK_BYTES];
    loop {
        match stderr.read(&mut buffer) {
            Ok(0) => break,
            Ok(read_count) => {
                hasher.update(&buffer[..read_count]);
                byte_count = byte_count.saturating_add(read_count);
                if byte_count > max_diagnostic_bytes {
                    truncated = true;
                }
            }
            Err(_) => return,
        }
    }
    let observation = StderrObservation {
        identity: hasher.finalize().to_hex().to_string(),
        byte_count: byte_count.min(max_diagnostic_bytes),
        truncated,
    };
    let _sent = sender.send(observation);
}
