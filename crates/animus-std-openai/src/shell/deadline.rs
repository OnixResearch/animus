//! Offline transport controls. Only this private fixture admits a loopback wire URL.

use std::io::{Read, Write};

const TIMEOUT_MS: u64 = 200;
const DRIP_MS: u64 = 50;
const SERVER_LIMIT_MS: u64 = 2000;
const SCHEDULING_SLACK_MS: u64 = 1000;
const RESPONSE_BYTES: u64 = 4096;
const REQUEST_BYTES: usize = 4096;
const SUCCESS: u16 = 200;
const COMPLETE: &[u8] = b"{}";
const COMMENT: &[u8] = b": keepalive\n\n";

#[derive(Clone, Copy)]
enum Path {
    Text,
    Stream,
}

fn reply(mut stream: std::net::TcpStream, active: bool, done: &std::sync::mpsc::Receiver<()>) {
    let limit = std::time::Duration::from_millis(SERVER_LIMIT_MS);
    stream.set_read_timeout(Some(limit)).expect("read bound");
    stream.set_write_timeout(Some(limit)).expect("write bound");
    let mut request = Vec::new();
    while !request.ends_with(b"\r\n\r\n") {
        assert!(request.len() < REQUEST_BYTES);
        let mut byte = [0];
        stream.read_exact(&mut byte).expect("request header");
        request.push(byte[0]);
    }
    let mut body = [0; COMPLETE.len()];
    stream.read_exact(&mut body).expect("request body");
    assert_eq!(body, COMPLETE);
    let content_bytes = if active {
        RESPONSE_BYTES
    } else {
        u64::try_from(COMPLETE.len()).expect("length")
    };
    write!(
        stream,
        "HTTP/1.1 {SUCCESS} OK\r\nContent-Length: {content_bytes}\r\nConnection: close\r\n\r\n"
    )
    .expect("headers");
    if !active {
        stream.write_all(COMPLETE).expect("complete response");
        return;
    }
    let start = std::time::Instant::now();
    let mut sent_bytes = 0_u64;
    while start.elapsed() < limit {
        if !matches!(done.try_recv(), Err(std::sync::mpsc::TryRecvError::Empty)) {
            break;
        }
        if let Err(error) = stream.write_all(COMMENT) {
            assert!(matches!(
                error.kind(),
                std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset
            ));
            break;
        }
        sent_bytes = sent_bytes
            .checked_add(u64::try_from(COMMENT.len()).expect("comment length"))
            .expect("byte count");
        assert!(sent_bytes < RESPONSE_BYTES);
        std::thread::sleep(std::time::Duration::from_millis(DRIP_MS));
    }
    assert!(sent_bytes > 0);
}

fn accept(listener: &std::net::TcpListener) -> (std::net::TcpStream, std::net::SocketAddr) {
    listener.set_nonblocking(true).expect("accept mode");
    let started = std::time::Instant::now();
    loop {
        match listener.accept() {
            Ok(connection) => return connection,
            Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::WouldBlock),
        }
        assert!(started.elapsed() < std::time::Duration::from_millis(SERVER_LIMIT_MS));
        std::thread::sleep(std::time::Duration::from_millis(DRIP_MS));
    }
}

fn observe(path: Path, active: bool) {
    let listener =
        std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).expect("private listener");
    let address = listener.local_addr().expect("address");
    assert!(address.ip().is_loopback());
    let endpoint = format!("http://{address}/v1/responses");
    let timeout = std::time::Duration::from_millis(TIMEOUT_MS);
    let config = crate::OpenAiHostConfig::new(
        crate::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
        crate::LOCAL_DSPARK_RESPONSES_URL,
        timeout,
        RESPONSE_BYTES,
    )
    .expect("config");
    let host = super::ReqwestOpenAiHost::new(config).expect("host");
    let key =
        crate::OpenAiApiKey::new("fixture", "synthetic-only".to_owned()).expect("synthetic key");
    let request = super::WireRequest {
        endpoint_url: &endpoint,
        body: COMPLETE.to_vec(),
        max_response_bytes: RESPONSE_BYTES,
    };
    let (done, receiver) = std::sync::mpsc::channel();
    std::thread::scope(|scope| {
        let server = scope.spawn(move || {
            let (stream, peer) = accept(&listener);
            assert!(peer.ip().is_loopback());
            reply(stream, active, &receiver);
        });
        let started = std::time::Instant::now();
        let result = match path {
            Path::Text => super::Transport::send(&host, &request, &key).map(|response| {
                assert_eq!(response.status, SUCCESS);
                response.body
            }),
            Path::Stream => super::StreamingTransport::open_stream(&host, &request, &key).and_then(
                |mut response| {
                    assert_eq!(response.status, SUCCESS);
                    let mut bytes = Vec::new();
                    response
                        .body
                        .read_to_end(&mut bytes)
                        .map_err(|_| crate::TransportErrorClass::ResponseRead)?;
                    Ok(bytes)
                },
            ),
        };
        let elapsed = started.elapsed();
        let _notification = done.send(());
        server.join().expect("server stopped");
        if active {
            assert_eq!(result, Err(crate::TransportErrorClass::ResponseRead));
            assert!(elapsed >= timeout);
            let latest = timeout
                .checked_add(std::time::Duration::from_millis(SCHEDULING_SLACK_MS))
                .expect("deadline");
            assert!(
                elapsed < latest,
                "configured={timeout:?}, observed={elapsed:?}"
            );
        } else {
            assert_eq!(result.expect("successful response"), COMPLETE);
        }
    });
}

#[test]
fn text_complete_body_succeeds() {
    observe(Path::Text, false);
}
#[test]
fn stream_complete_body_succeeds() {
    observe(Path::Stream, false);
}
#[test]
fn text_activity_cannot_renew_deadline() {
    observe(Path::Text, true);
}
#[test]
fn stream_activity_cannot_renew_deadline() {
    observe(Path::Stream, true);
}
