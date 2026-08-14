//! Local Unix transport framing and endpoint tests.

use animus_local_daemon::{
    LocalClient, LocalEndpoint, LocalListener, TransportError, TransportFrame,
};
use std::time::Duration;

const MAX_FRAME_BYTES: u32 = 4096;
const DEADLINE_MS: u64 = 1000;

#[test]
fn explicit_unix_endpoint_authenticates_peer_and_round_trips_frame() {
    let temporary = tempfile::tempdir().expect("temporary root");
    let endpoint = LocalEndpoint::new(temporary.path(), "daemon.sock").expect("endpoint");
    let endpoint_identity = String::from(endpoint.identity());
    let listener = LocalListener::bind(endpoint.clone()).expect("listener");
    let worker = std::thread::spawn(move || {
        let (mut server, peer) = listener.accept().expect("accept");
        assert!(peer.process_id > 0);
        server
            .set_deadline(Duration::from_millis(DEADLINE_MS))
            .expect("server deadline");
        let frame: TransportFrame<String> = server.read_frame(MAX_FRAME_BYTES).expect("read");
        assert_eq!(frame.payload, "snapshot");
        server.write_frame(&frame, MAX_FRAME_BYTES).expect("write");
        listener.close().expect("close listener");
    });
    let mut client = LocalClient::connect(&endpoint).expect("client");
    client
        .set_deadline(Duration::from_millis(DEADLINE_MS))
        .expect("client deadline");
    let frame = TransportFrame {
        version: 1,
        endpoint_identity,
        generation: 1,
        correlation_id: String::from("correlation-a"),
        payload: String::from("snapshot"),
    };
    client.write_frame(&frame, MAX_FRAME_BYTES).expect("send");
    assert_eq!(
        client
            .read_frame::<String>(MAX_FRAME_BYTES)
            .expect("response"),
        frame
    );
    drop(client);
    worker.join().expect("transport worker");
    assert!(!endpoint.path().exists());
}

#[test]
#[cfg(unix)]
fn ambient_missing_existing_symlink_and_overbound_frames_fail_closed() {
    use std::os::unix::fs::symlink;

    let temporary = tempfile::tempdir().expect("temporary root");
    assert!(matches!(
        LocalEndpoint::new(temporary.path(), "../daemon.sock"),
        Err(TransportError::InvalidEndpoint)
    ));
    let endpoint = LocalEndpoint::new(temporary.path(), "daemon.sock").expect("endpoint");
    assert!(matches!(
        LocalClient::connect(&endpoint),
        Err(TransportError::Io)
    ));
    std::fs::write(temporary.path().join("target"), b"not a socket").expect("target");
    symlink("target", endpoint.path()).expect("symlink");
    assert!(matches!(
        LocalClient::connect(&endpoint),
        Err(TransportError::InvalidEndpoint)
    ));
}
