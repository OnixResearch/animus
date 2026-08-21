# Animus local daemon

This non-default AGPL package supplies a local Unix supervisor and worker host.

The supervisor owns one explicit local endpoint, routing, registry state, health checks, and worker lifecycle. Workers own session execution. The supervisor routes bounded private events without decoding or retaining their payloads.

The package does not supply remote transport, ambient endpoint discovery, a security sandbox, high availability, service-manager integration, or release eligibility.
