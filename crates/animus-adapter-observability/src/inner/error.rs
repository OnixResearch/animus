/// Observability contract validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Metric, target, label, or sink name was empty.
    EmptyName,
    /// A label collection was missing or exceeded bounded cardinality.
    InvalidLabelCount,
    /// Safe receipt metadata validation rejected the record.
    UnsafeMetadata(animus_adapter_safe::Error),
    /// Message label exceeded the bounded log-message contract.
    UnboundedMessage,
    /// The sink is known but unavailable.
    SinkUnavailable,
    /// The sink or collector is closed.
    CollectorClosed,
}

impl core::fmt::Display for Error {
    // Animus implements the non-const core::fmt::Display trait; its method cannot be const.
    #[cfg_attr(dylint_lib = "octet", allow(tigerstyle::missing_const_fn))]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("observability name must not be empty"),
            Self::InvalidLabelCount => {
                formatter.write_str("observability labels are out of bounds")
            }
            Self::UnsafeMetadata(error) => write!(formatter, "{error}"),
            Self::UnboundedMessage => formatter.write_str("observability message is unbounded"),
            Self::SinkUnavailable => formatter.write_str("observability sink is unavailable"),
            Self::CollectorClosed => formatter.write_str("observability collector is closed"),
        }
    }
}
