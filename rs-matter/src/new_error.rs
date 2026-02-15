// -----

use crate::{im::IMStatusCode, sc::SCStatusCodes};

pub enum HolisticError {
    SdkCoreError(SdkCoreError),
    IMGlobalStatus(IMStatusCode),
    SCStatus(SCStatusCodes)
}

impl From<SdkCoreError> for HolisticError {
    fn from(value: SdkCoreError) -> Self {
        Self::SdkCoreError(value)
    }
}

// This can be implemented as impl Into<SdkCoreError> for HolisticError
impl From<HolisticError> for SdkCoreError {
    fn from(value: HolisticError) -> Self {
        match value {
            HolisticError::SdkCoreError(sdk_core_error) => sdk_core_error,
            HolisticError::IMGlobalStatus(im_status_code) => {
                match im_status_code {
                    IMStatusCode::Success => SdkCoreError::Success,
                    IMStatusCode::Busy => SdkCoreError::Busy,
                    _ => SdkCoreError::Failure,
                }
            },
            HolisticError::SCStatus(sc_status_codes) => {
                match sc_status_codes {
                    SCStatusCodes::SessionEstablishmentSuccess
                    | SCStatusCodes::CloseSession => SdkCoreError::Success,
                    SCStatusCodes::Busy => SdkCoreError::Busy,
                    _ => SdkCoreError::Failure,
                }
            },
        }
    }
}

impl From<IMStatusCode> for HolisticError {
    fn from(value: IMStatusCode) -> Self {
        Self::IMGlobalStatus(value)
    }
}

impl From<SCStatusCodes> for HolisticError {
    fn from(value: SCStatusCodes) -> Self {
        Self::SCStatus(value)
    }
}



/// SDK core errors. Extracted from `CHIPError.h`
pub enum SdkCoreError {
    /// Added for completeness.
    Success = 0,
    Failure = 0xff,
    /// A message exceeds the sent limit.
    SendingBlocked = 0x01,
    /// A connection has been aborted.
    ConnectionAborted = 0x02,
    /// An unexpected state was encountered.
    IncorrectState = 0x03,
    /// A message is too long.
    MessageTooLong = 0x04,
    /// Recursion depth overflow
    RecursionDepthLimit = 0x05,
    /// The attempt to register an unsolicited message handler failed because the
    /// unsolicited message handler pool is full.
    TooManyUnsolicitedMessageHandlers = 0x06,
    /// The attempt to unregister an unsolicited message handler failed because
    /// the target handler was not found in the unsolicited message handler pool.
    NoUnsolicitedMessageHandler = 0x07,
    /// No callback has been registered for handling a connection.
    NoConnectionHandler = 0x08,
    /// The number of peer nodes exceeds the maximum limit of a local node.
    TooManyPeerNodes = 0x09,
    /// For use locally to mark conditions such as value found or end of iteration.
    Sentinel = 0x0a,
    /// The attempt to allocate a buffer or object failed due to a lack of memory.
    NoMemory = 0x0b,
    /// No callback has been registered for handling a message.
    NoMessageHandler = 0x0c,
    /// A message is incomplete.
    MessageIncomplete = 0x0d,
    /// The data is not aligned.
    DataNotAligned = 0x0e,
    /// The encryption key type is unknown.
    UnknownKeyType = 0x0f,
    /// The encryption key is not found.
    KeyNotFound = 0x10,
    /// The encryption type is incorrect for the specified key.
    WrongEncryptionType = 0x11,
    /// Invalid UTF8 string (contains some characters that are invalid)
    InvalidUtf8 = 0x12,
    /// The integrity check in the message does not match the expected integrity
    /// check.
    IntegrityCheckFailed = 0x13,
    /// Invalid signature.
    InvalidSignature = 0x14,
    /// Invalid TLV character string (e.g. null terminator)
    InvalidTlvCharString = 0x15,
    /// Subscription timeout caused by LIT ICD device inactive mode
    LitSubscribeInactiveTimeout = 0x16,
    /// A signature type is unsupported.
    UnsupportedSignatureType = 0x17,
    /// A message length is invalid.
    InvalidMessageLength = 0x18,
    /// A buffer is too small.
    BufferTooSmall = 0x19,
    /// A key id is duplicate.
    DuplicateKeyId = 0x1a,
    /// A key type does not match the expected key type.
    WrongKeyType = 0x1b,
    /// A requested object is uninitialized.
    Uninitialized = 0x1c,
    /// The IPK is invalid
    InvalidIpk = 0x1d,
    /// A string length is invalid.
    InvalidStringLength = 0x1e,
    /// A list length is invalid.
    InvalidListLength = 0x1f,
    /// The end of a TLV encoding,
    /// or the end of a TLV container element has been reached.
    EndOfTlv = 0x21,
    /// The TLV encoding ended prematurely.
    TlvUnderrun = 0x22,
    /// A TLV element is invalid.
    InvalidTlvElement = 0x23,
    /// A TLV tag is invalid.
    InvalidTlvTag = 0x24,
    /// An implicitly encoded TLV tag was encountered,
    /// but an implicit profile id has not been defined.
    UnknownImplicitTlvTag = 0x25,
    /// A TLV type is wrong.
    WrongTlvType = 0x26,
    /// A TLV container is unexpectedly open.
    TlvContainerOpen = 0x27,
    /// A message type is invalid.
    InvalidMessageType = 0x2a,
    /// An unexpected TLV element was encountered.
    UnexpectedTlvElement = 0x2b,
    /// A requested function or feature is not implemented.
    NotImplemented = 0x2d,
    /// An address is invalid.
    InvalidAddress = 0x2e,
    /// An argument is invalid.
    InvalidArgument = 0x2f,
    /// A TLV path list is invalid.
    InvalidPathList = 0x30,
    /// A TLV data list is invalid.
    InvalidDataList = 0x31,
    /// A request timed out.
    Timeout = 0x32,
    /// A device descriptor is invalid.
    InvalidDeviceDescriptor = 0x33,
    /// A PASE parameter is invalid.
    InvalidPaseParameter = 0x38,
    /// A use of session key is invalid.
    InvalidUseOfSessionKey = 0x3b,
    /// A connection is closed unexpectedly.
    ConnectionClosedUnexpectedly = 0x3c,
    /// A TLV element is missing.
    MissingTlvElement = 0x3d,
    /// Secure random data is not available.
    RandomDataUnavailable = 0x3e,
    /// A host/port list is empty.
    HostPortListEmpty = 0x41,
    /// A service manager is forced to reset.
    ForcedReset = 0x45,
    /// No endpoint is available.
    NoEndpoint = 0x46,
    /// A destination node id is invalid.
    InvalidDestinationNodeId = 0x47,
    /// The operation cannot be performed because the underlying object is not
    /// connected.
    NotConnected = 0x48,
    /// CA certificate is not found.
    CaCertNotFound = 0x4a,
    /// A certificate path length exceeds the constraint.
    CertPathLenConstraintExceeded = 0x4b,
    /// A certificate path is too long.
    CertPathTooLong = 0x4c,
    /// A requested certificate usage is not allowed.
    CertUsageNotAllowed = 0x4d,
    /// A certificate expired.
    CertExpired = 0x4e,
    /// A certificate is not valid yet.
    CertNotValidYet = 0x4f,
    /// A certificate format is unsupported.
    UnsupportedCertFormat = 0x50,
    /// An elliptic curve is unsupported.
    UnsupportedEllipticCurve = 0x51,
    /// A certificate is not found.
    CertNotFound = 0x53,
    /// A CASE parameter is invalid.
    InvalidCaseParameter = 0x54,
    /// A certificate load failed.
    CertLoadFailed = 0x56,
    /// A certificate is not trusted.
    CertNotTrusted = 0x57,
    /// A certificate subject/issuer distinguished name is wrong.
    WrongCertDn = 0x59,
    /// A node id is wrong.
    WrongNodeId = 0x5c,
    /// A retransmission table is already full.
    RetransTableFull = 0x64,
    /// A transaction is cancelled.
    TransactionCanceled = 0x68,
    /// A message was received as part of a subscription exchange that has a mis-matching subscription id.
    InvalidSubscription = 0x6b,
    /// A CHIP feature is unsupported.
    UnsupportedChipFeature = 0x6c,
    /// An unsolicited message with the originator bit clear.
    UnsolicitedMsgNoOriginator = 0x70,
    /// A fabric index is invalid.
    InvalidFabricIndex = 0x71,
    /// The attempt to allocate a connection object failed because too many
    /// connections exist.
    TooManyConnections = 0x72,
    /// The operation cancelled because a shut down was initiated
    ShutDown = 0x73,
    /// The operation has been cancelled, generally by calling a cancel/abort request.
    Cancelled = 0x74,
    /// A specified TLV tag was not found.
    TlvTagNotFound = 0x76,
    /// A secure session is needed to do work, but is missing/is not present.
    MissingSecureSession = 0x77,
    /// The CaseAdminSubject field is not valid in AddNOC command.
    InvalidAdminSubject = 0x78,
    /// Required privilege was insufficient during an operation.
    InsufficientPrivilege = 0x79,
    /// The message counter of the session is exhausted, the session should be closed.
    MessageCounterExhausted = 0x7d,
    /// The fabric with the given fabric id and root public key already exists.
    FabricExists = 0x7e,
    /// The endpoint with the given endpoint id already exists.
    EndpointExists = 0x7f,
    /// The wrong encryption type error received from a peer node.
    WrongEncryptionTypeFromPeer = 0x80,
    /// A key id is invalid.
    InvalidKeyId = 0x85,
    /// Time has invalid value.
    InvalidTime = 0x86,
    /// A mismatch in schema was encountered.
    SchemaMismatch = 0x8e,
    /// An integer does not have the kind of value we expect.
    InvalidIntegerValue = 0x8f,
    /// The request cannot be processed or fulfilled
    BadRequest = 0x92,
    /// The presented certificate was of the wrong type.
    WrongCertType = 0x9d,
    /// Persisted storage memory read/write failure.
    PersistedStorageFailed = 0x9f,
    /// The specific value is not found in the persisted storage.
    PersistedStorageValueNotFound = 0xa0,
    /// The fabric is deleted, and the corresponding IM resources are released
    ImFabricDeleted = 0xa1,
    /// The operation is still in progress
    InProgress = 0xa4,
    /// The CHIP message is not granted access for further processing.
    AccessDenied = 0xa5,
    /// Unknown resource ID
    UnknownResourceId = 0xa6,
    /// The conditional update of a trait instance path has failed
    /// because the local changes are based on an obsolete version of the
    /// data.
    VersionMismatch = 0xa7,
    /// Event ID matching the criteria was found
    EventIdFound = 0xab,
    /// Internal error
    Internal = 0xac,
    /// Open file failed
    OpenFailed = 0xad,
    /// Read from file failed
    ReadFailed = 0xae,
    /// Write to file failed
    WriteFailed = 0xaf,
    /// Decoding failed
    DecodeFailed = 0xb0,
    /// The application is not authorized to do DNS_SD lookups.
    DnsSdUnauthorized = 0xb3,
    /// The registered service name has collision on the LAN.
    MdnsCollision = 0xb4,
    /// The Attribute path IB is malformed: it does not contain
    /// the required path
    ImMalformedAttributePathIb = 0xb5,
    /// The Event Path IB is malformed: it does not contain
    /// the required elements
    ImMalformedEventPathIb = 0xb6,
    /// The Command Data IB is malformed: it does not contain
    /// the required elements
    ImMalformedCommandDataIb = 0xb9,
    /// The Event Data IB is malformed: it does not contain
    /// the required elements
    ImMalformedEventDataIb = 0xba,
    /// Caller is trying to add more invoke command paths
    /// than what the remote node reports supporting.
    MaximumPathsPerInvokeExceeded = 0xbb,
    /// Unable to find the peer node
    PeerNodeNotFound = 0xbc,
    /// Error in Hardware security module. Used for software fallback option.
    Hsm = 0xbd,
    /// The system's real time clock is not synchronized to an accurate time source.
    RealTimeNotSynced = 0xbf,
    /// An unexpected event was encountered.
    UnexpectedEvent = 0xc0,
    /// No endpoint pool entry is available.
    EndpointPoolFull = 0xc1,
    /// More inbound message data is pending than available buffer space available to copy it.
    InboundMessageTooBig = 0xc2,
    /// More outbound message data is pending than available buffer space available to copy it.
    OutboundMessageTooBig = 0xc3,
    /// The received message is a duplicate of a previously received message.
    DuplicateMessageReceived = 0xc4,
    /// The received public key doesn't match locally generated key.
    InvalidPublicKey = 0xc5,
    /// The fabric ID in ICA certificate doesn't match the one in NOC.
    FabricMismatchOnIca = 0xc6,
    /// The CASE session could not be established as peer's credentials do not have
    /// a common root of trust.
    NoSharedTrustedRoot = 0xc9,
    /// Indicates an IM status code was received. Usually accompanied with
    /// the actual IM status code.
    ImStatusCodeReceived = 0xca,
    /// The Data Version Filter IB is malformed: it does not contain
    /// the required elements
    ImMalformedDataVersionFilterIb = 0xd7,
    /// The item referenced in the function call was not found
    NotFound = 0xd8,
    /// The file identifier, encoded in the first few bytes of a processed file,
    /// has unexpected value.
    InvalidFileIdentifier = 0xda,
    /// The Resource is busy and cannot process the request. Trying again might work.
    Busy = 0xdb,
    /// The maximum retry limit has been exceeded.
    MaxRetryExceeded = 0xdc,
    /// The provider list has been exhausted.
    ProviderListExhausted = 0xdd,
    /// The scheme field contains an invalid prefix
    InvalidSchemePrefix = 0xdf,
    /// The URI separator is missing
    MissingUriSeparator = 0xe0,
    /// Callback function or callable object is not set
    HandlerNotSet = 0xe1,
}
// AVAILABLE: 0x20
// AVAILABLE: 0x28
// AVAILABLE: 0x29
// AVAILABLE: 0x2c
// AVAILABLE: 0x34
// AVAILABLE: 0x35
// AVAILABLE: 0x36
// AVAILABLE: 0x37
// AVAILABLE: 0x39
// AVAILABLE: 0x3a
// AVAILABLE: 0x3f
// AVAILABLE: 0x40
// AVAILABLE: 0x42
// AVAILABLE: 0x43
// AVAILABLE: 0x44
// AVAILABLE: 0x49
// AVAILABLE: 0x52
// AVAILABLE: 0x55
// AVAILABLE: 0x58
// AVAILABLE: 0x5a
// AVAILABLE: 0x5b
// AVAILABLE: 0x5d
// AVAILABLE: 0x5e
// AVAILABLE: 0x5f
// AVAILABLE: 0x60
// AVAILABLE: 0x61
// AVAILABLE: 0x62
// AVAILABLE: 0x63
// AVAILABLE: 0x65
// AVAILABLE: 0x66
// AVAILABLE: 0x67
// AVAILABLE: 0x69
// AVAILABLE: 0x6a
// AVAILABLE: 0x75
// AVAILABLE: 0x7a
// AVAILABLE: 0x7b
// AVAILABLE: 0x7c
// AVAILABLE: 0x81
// AVAILABLE: 0x82
// AVAILABLE: 0x83
// AVAILABLE: 0x84
// AVAILABLE: 0x87
// AVAILABLE: 0x88
// AVAILABLE: 0x89
// AVAILABLE: 0x8a
// AVAILABLE: 0x8b
// AVAILABLE: 0x8c
// AVAILABLE: 0x8d
// AVAILABLE: 0x90
// AVAILABLE: 0x91
// AVAILABLE: 0x93
// AVAILABLE: 0x94
// AVAILABLE: 0x95
// AVAILABLE: 0x96
// AVAILABLE: 0x97
// AVAILABLE: 0x98
// AVAILABLE: 0x99
// AVAILABLE: 0x9a
// AVAILABLE: 0x9b
// AVAILABLE: 0x9c
// AVAILABLE: 0x9e
// AVAILABLE: 0xa2
// AVAILABLE: 0xa3
// AVAILABLE: 0xa8
// AVAILABLE: 0xa9
// AVAILABLE: 0xaa
// AVAILABLE: 0xb1
// AVAILABLE: 0xb2
// AVAILABLE: 0xb7
// AVAILABLE: 0xb8
// AVAILABLE: 0xbe
// AVAILABLE: 0xc7
// AVAILABLE: 0xc8
// AVAILABLE: 0xcb
// AVAILABLE: 0xcc
// AVAILABLE: 0xcd
// AVAILABLE: 0xce
// AVAILABLE: 0xcf
// AVAILABLE: 0xd0
// AVAILABLE: 0xd1
// AVAILABLE: 0xd2
// AVAILABLE: 0xd3
// AVAILABLE: 0xd4
// AVAILABLE: 0xd5
// AVAILABLE: 0xd6
// AVAILABLE: 0xd9
// AVAILABLE: 0xde