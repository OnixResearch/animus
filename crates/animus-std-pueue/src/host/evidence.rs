use animus_adapters::{
    CapabilityClass, OutcomeKind, Reason, Receipt, ReceiptDraft, RedactionMarker, SafeHash,
    ServiceKind, ServiceOutcome,
};
use animus_adapters::{QueueServiceRequest, QueueServiceResponse};

const COMPLETED_PAYLOAD_COUNT: u32 = 1;

pub(crate) fn completed(
    request: &QueueServiceRequest,
    response: QueueServiceResponse,
) -> ServiceOutcome<QueueServiceResponse> {
    let output = format!("{response:?}");
    let receipt = receipt(
        request,
        OutcomeKind::Completed,
        Reason::Completed,
        Some(output.as_bytes()),
    );
    ServiceOutcome::completed(response, receipt)
}

pub(crate) fn denied(request: &QueueServiceRequest) -> ServiceOutcome<QueueServiceResponse> {
    ServiceOutcome::denied(
        None,
        receipt(request, OutcomeKind::Denied, Reason::PolicyDenied, None),
    )
}

pub(crate) fn unavailable(request: &QueueServiceRequest) -> ServiceOutcome<QueueServiceResponse> {
    ServiceOutcome::unavailable(receipt(
        request,
        OutcomeKind::Unavailable,
        Reason::HostUnavailable,
        None,
    ))
}

pub(crate) fn cancelled(request: &QueueServiceRequest) -> ServiceOutcome<QueueServiceResponse> {
    ServiceOutcome::cancelled(receipt(
        request,
        OutcomeKind::Cancelled,
        Reason::Cancelled,
        None,
    ))
}

pub(crate) fn timed_out(request: &QueueServiceRequest) -> ServiceOutcome<QueueServiceResponse> {
    ServiceOutcome::timed_out(receipt(
        request,
        OutcomeKind::TimedOut,
        Reason::DeadlineExceeded,
        None,
    ))
}

pub(crate) fn malformed(request: &QueueServiceRequest) -> ServiceOutcome<QueueServiceResponse> {
    ServiceOutcome::malformed_output(receipt(
        request,
        OutcomeKind::MalformedOutput,
        Reason::InvalidBoundaryPayload,
        None,
    ))
}

fn receipt(
    request: &QueueServiceRequest,
    outcome: OutcomeKind,
    reason: Reason,
    output: Option<&[u8]>,
) -> Receipt {
    let input = format!("{request:?}");
    let draft = ReceiptDraft::new(
        ServiceKind::HostEffect,
        CapabilityClass::HostEffect,
        outcome,
        reason,
    )
    .with_input_hash(SafeHash::from_blake3_bytes(input.as_bytes()));
    let draft = match output {
        Some(bytes) => draft
            .with_output_hash(SafeHash::from_blake3_bytes(bytes))
            .with_payload_count(COMPLETED_PAYLOAD_COUNT)
            .with_redaction(RedactionMarker::PayloadHashOnly),
        None => draft,
    };
    Receipt::new(draft)
}
