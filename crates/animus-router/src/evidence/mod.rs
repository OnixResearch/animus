//! Router-receipt evidence assembly, validation, and leak screening.
//!
//! Keep this in sync with `src/evidence.rs` so builds that resolve the
//! directory-style module entry point still expose the full evidence surface.

mod label;
mod record;
mod screen;

pub use label::{DurationBucket, QuorumClass, Redaction};
pub use record::{Receipt, ReceiptDraft};
pub use screen::{ReceiptLeak, validate_receipt_text};
