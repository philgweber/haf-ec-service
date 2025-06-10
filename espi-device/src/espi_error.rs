use core::fmt;

/// Error types for eSPI operations
#[derive(Debug)]
pub enum EspiError {
    /// CRC error detected
    CrcError,
    /// Command or response timeout
    Timeout,
    /// Channel not supported or not enabled
    ChannelNotAvailable,
    /// Device reported a fatal error
    FatalError,
    /// Device reported a non-fatal error
    NonFatalError,
    /// Protocol violation
    ProtocolError,
    /// Invalid parameters provided
    InvalidParameters,
    /// Feature not supported by device
    Unsupported,
    /// Malformed packet received
    MalformedPacket,
    /// Device busy or buffer full
    DeviceBusy,
    /// Platform-specific error
    PlatformError(&'static str),
    /// Other unspecified error
    Other(&'static str),
}

impl fmt::Display for EspiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CrcError => write!(f, "CRC error detected"),
            Self::Timeout => write!(f, "Command or response timeout"),
            Self::ChannelNotAvailable => write!(f, "Channel not supported or not enabled"),
            Self::FatalError => write!(f, "Device reported a fatal error"),
            Self::NonFatalError => write!(f, "Device reported a non-fatal error"),
            Self::ProtocolError => write!(f, "Protocol violation"),
            Self::InvalidParameters => write!(f, "Invalid parameters provided"),
            Self::Unsupported => write!(f, "Feature not supported by device"),
            Self::MalformedPacket => write!(f, "Malformed packet received"),
            Self::DeviceBusy => write!(f, "Device busy or buffer full"),
            Self::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            Self::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}
