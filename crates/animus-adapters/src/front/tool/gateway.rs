include!("gateway/delivery_types.rs");
include!("gateway/delivery_request.rs");
include!("gateway/delivery_policy.rs");
include!("gateway/delivery_outbox.rs");
include!("gateway/delivery_retry_receipts.rs");
include!("gateway/delivery_errors.rs");

#[cfg(test)]
mod tests {
    include!("gateway/tests/tests001.rs");
    include!("gateway/tests/tests002.rs");
}
