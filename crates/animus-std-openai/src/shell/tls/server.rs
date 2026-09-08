//! Finite local TLS peer. Certificate failures must precede HTTP request reads.

use std::io::{Read, Write};
const HEADER_BYTES: usize = 4096;
const HANDSHAKE_BYTES: usize = 1;

pub(super) fn serve(
    listener: &std::net::TcpListener,
    certificate: &rcgen::CertifiedKey,
    case: super::Case,
    done: &std::sync::mpsc::Receiver<()>,
) -> bool {
    let mut socket = accept(listener);
    let limit = std::time::Duration::from_millis(super::SERVER_LIMIT_MS);
    socket.set_read_timeout(Some(limit)).expect("read timeout");
    socket
        .set_write_timeout(Some(limit))
        .expect("write timeout");
    if case == super::Case::HandshakeStall {
        let mut byte = [0; HANDSHAKE_BYTES];
        socket.read_exact(&mut byte).expect("client started TLS");
        assert_ne!(byte, [0; HANDSHAKE_BYTES]);
        done.recv_timeout(limit)
            .expect("client deadline notification");
        return false;
    }
    let key = rustls::pki_types::PrivatePkcs8KeyDer::from(certificate.key_pair.serialize_der());
    let config = rustls::ServerConfig::builder_with_provider(std::sync::Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("TLS versions")
    .with_no_client_auth()
    .with_single_cert(vec![certificate.cert.der().clone()], key.into())
    .expect("TLS server");
    let mut connection =
        rustls::ServerConnection::new(std::sync::Arc::new(config)).expect("connection");
    while connection.is_handshaking() {
        if let Err(error) = connection.complete_io(&mut socket) {
            assert!(
                case == super::Case::Untrusted || case == super::Case::WrongName,
                "unexpected handshake error: {error}"
            );
            return false;
        }
    }
    assert!(
        case != super::Case::Untrusted && case != super::Case::WrongName,
        "invalid certificate accepted"
    );
    let mut stream = rustls::StreamOwned::new(connection, socket);
    let mut request = Vec::new();
    while !request.ends_with(b"\r\n\r\n") {
        assert!(request.len() < HEADER_BYTES);
        let mut byte = [0];
        stream.read_exact(&mut byte).expect("HTTP header");
        request.push(byte[0]);
    }
    assert!(request.starts_with(b"POST /v1/responses HTTP/1.1\r\n"));
    let mut body = [0; super::BODY.len()];
    stream.read_exact(&mut body).expect("HTTP body");
    assert_eq!(body, super::BODY);
    let length = if case == super::Case::ActiveBody {
        super::RESPONSE_BYTES
    } else {
        u64::try_from(body.len()).expect("body length")
    };
    write!(
        stream,
        "HTTP/1.1 {} OK\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n",
        super::SUCCESS
    )
    .expect("headers");
    if case != super::Case::ActiveBody {
        stream.write_all(super::BODY).expect("response body");
        stream.flush().expect("response flush");
        return true;
    }
    let started = std::time::Instant::now();
    let mut sent_bytes = 0_u64;
    while started.elapsed() < limit {
        if !matches!(done.try_recv(), Err(std::sync::mpsc::TryRecvError::Empty)) {
            break;
        }
        if stream
            .write_all(super::COMMENT)
            .and_then(|()| stream.flush())
            .is_err()
        {
            break;
        }
        sent_bytes = sent_bytes
            .checked_add(u64::try_from(super::COMMENT.len()).expect("comment length"))
            .expect("sent count");
        assert!(sent_bytes < super::RESPONSE_BYTES);
        std::thread::sleep(std::time::Duration::from_millis(super::DRIP_MS));
    }
    assert!(sent_bytes > 0);
    true
}

fn accept(listener: &std::net::TcpListener) -> std::net::TcpStream {
    listener.set_nonblocking(true).expect("listener mode");
    let started = std::time::Instant::now();
    loop {
        match listener.accept() {
            Ok((stream, peer)) => {
                assert!(peer.ip().is_loopback());
                return stream;
            }
            Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::WouldBlock),
        }
        assert!(started.elapsed() < std::time::Duration::from_millis(super::SERVER_LIMIT_MS));
        std::thread::sleep(std::time::Duration::from_millis(super::POLL_MS));
    }
}
