//! Private TLS controls. Synthetic trust and local routing never enter public configuration.

mod server;
use std::io::Read;

const HOST: &str = "api.openai.com";
const WRONG_HOST: &str = "wrong.invalid";
const TIMEOUT_MS: u64 = 500;
const SERVER_LIMIT_MS: u64 = 3000;
const SLACK_MS: u64 = 1500;
const POLL_MS: u64 = 20;
const DRIP_MS: u64 = 50;
const RESPONSE_BYTES: u64 = 4096;
const BODY: &[u8] = b"{}";
const COMMENT: &[u8] = b": keepalive\n\n";
const SUCCESS: u16 = 200;

#[derive(Clone, Copy)]
enum Path {
    Text,
    Stream,
}
#[derive(Clone, Copy, PartialEq)]
enum Case {
    Complete,
    Untrusted,
    WrongName,
    HandshakeStall,
    ActiveBody,
}

fn observe(path: Path, case: Case) {
    let listener =
        std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).expect("listener");
    let address = listener.local_addr().expect("address");
    assert!(address.ip().is_loopback());
    let name = if case == Case::WrongName {
        WRONG_HOST
    } else {
        HOST
    };
    let certificate =
        rcgen::generate_simple_self_signed(vec![name.to_owned()]).expect("synthetic certificate");
    let timeout = std::time::Duration::from_millis(TIMEOUT_MS);
    let config = crate::OpenAiHostConfig::new(
        crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
        crate::OFFICIAL_OPENAI_RESPONSES_URL,
        timeout,
        RESPONSE_BYTES,
    )
    .expect("admitted config");
    let mut builder = super::ReqwestOpenAiHost::client_builder(&config)
        .tls_built_in_root_certs(false)
        .resolve(HOST, address);
    if case != Case::Untrusted {
        builder = builder.add_root_certificate(
            reqwest::Certificate::from_der(certificate.cert.der()).expect("root"),
        );
    }
    let host = super::ReqwestOpenAiHost {
        config,
        client: builder.build().expect("client"),
    };
    let endpoint = format!("https://{HOST}:{}/v1/responses", address.port());
    let request = super::WireRequest {
        endpoint_url: &endpoint,
        body: BODY.to_vec(),
        max_response_bytes: RESPONSE_BYTES,
    };
    let key = crate::OpenAiApiKey::new("fixture", "synthetic-only".to_owned()).expect("key");
    let (done, receiver) = std::sync::mpsc::sync_channel(1);
    std::thread::scope(|scope| {
        let worker = scope.spawn(move || server::serve(&listener, &certificate, case, &receiver));
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
        let _sent = done.send(());
        let observed_request = worker.join().expect("joined server");
        match case {
            Case::Complete => {
                assert!(observed_request);
                assert_eq!(result.expect("accepted TLS"), BODY);
            }
            Case::Untrusted | Case::WrongName => {
                assert!(!observed_request);
                assert_eq!(result, Err(crate::TransportErrorClass::Unavailable));
            }
            Case::HandshakeStall | Case::ActiveBody => {
                assert_eq!(observed_request, case == Case::ActiveBody);
                let expected = if case == Case::HandshakeStall {
                    crate::TransportErrorClass::Timeout
                } else {
                    crate::TransportErrorClass::ResponseRead
                };
                assert_eq!(result, Err(expected));
                assert!(elapsed >= timeout);
                assert!(
                    elapsed
                        < timeout
                            .checked_add(std::time::Duration::from_millis(SLACK_MS))
                            .expect("slack"),
                    "elapsed={elapsed:?}"
                );
            }
        }
    });
}

#[test]
fn text_accepts_synthetic_trust() {
    observe(Path::Text, Case::Complete);
}
#[test]
fn stream_accepts_synthetic_trust() {
    observe(Path::Stream, Case::Complete);
}
#[test]
fn text_rejects_untrusted_certificate() {
    observe(Path::Text, Case::Untrusted);
}
#[test]
fn stream_rejects_untrusted_certificate() {
    observe(Path::Stream, Case::Untrusted);
}
#[test]
fn text_rejects_wrong_hostname() {
    observe(Path::Text, Case::WrongName);
}
#[test]
fn stream_rejects_wrong_hostname() {
    observe(Path::Stream, Case::WrongName);
}
#[test]
fn text_bounds_handshake_stall() {
    observe(Path::Text, Case::HandshakeStall);
}
#[test]
fn stream_bounds_handshake_stall() {
    observe(Path::Stream, Case::HandshakeStall);
}
#[test]
fn text_bounds_active_tls_body() {
    observe(Path::Text, Case::ActiveBody);
}
#[test]
fn stream_bounds_active_tls_body() {
    observe(Path::Stream, Case::ActiveBody);
}
