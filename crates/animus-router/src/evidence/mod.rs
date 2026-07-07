//! Router-receipt evidence assembly, validation, and leak screening.
//!
//! Keep this in sync with `src/evidence.rs` so builds that resolve the
//! directory-style module entry point still expose the full evidence surface.

mod record;
mod screen;

pub use record::{DurationBucket, QuorumClass, Receipt, ReceiptDraft, Redaction};
pub use screen::{ReceiptLeak, validate_receipt_text};
