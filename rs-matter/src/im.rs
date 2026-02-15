/*
 *
 *    Copyright (c) 2020-2022 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

//! This module contains the TLV-serde types as defined by the Interaction Model.
//!
//! Additionally, it contains a very simple IM implementation - busy - which always
//! returns a busy status code, to all incoming IM requests.

use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::error::{Error, ErrorCode};
use crate::tlv::{FromTLV, TLVElement, TLVTag, TLVWrite, ToTLV, TLV};
use crate::transport::exchange::MessageMeta;

pub use attr::*;
pub use invoke::*;
pub use status::*;
pub use timed::*;

pub mod busy;

mod attr;
mod invoke;
mod status;
mod timed;

/// Interaction Model ID as per the Matter Core spec
pub const PROTO_ID_INTERACTION_MODEL: u16 = 0x01;

/// Global status codes for the Interaction Model.
/// These MAY be used by interaction model processing of actions and as common status codes for cluster specifications.
/// All values not defined here SHALL be reserved (per general conventions).
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IMStatusCode {
    /// Operation was successful.
    Success = 0x00,
    /// Operation was not successful.
    Failure = 0x01,
    /// Subscription ID is not active.
    InvalidSubscription = 0x7d,
    /// The sender of the action or command does not have authorization or access.
    UnsupportedAccess = 0x7e,
    /// The endpoint indicated is unsupported on the node.
    UnsupportedEndpoint = 0x7f,
    /// The action is malformed, has missing fields, or fields with invalid values.
    /// Action not carried out.
    InvalidAction = 0x80,
    /// The indicated command ID is not supported on the cluster instance.
    /// Command not carried out.
    UnsupportedCommand = 0x81,
    /// The cluster command is malformed, has missing fields, or fields with invalid values.
    /// Command not carried out.
    InvalidCommand = 0x85,
    /// The indicated attribute ID, field ID or list entry does not exist for an attribute path.
    UnsupportedAttribute = 0x86,
    /// Out of range error or set to a reserved value.
    /// Attribute keeps its old value.
    /// Note that an attribute value may be out of range if an attribute is related to another, e.g. with minimum and maximum attributes.
    ConstraintError = 0x87,
    /// Attempt to write a read-only attribute.
    UnsupportedWrite = 0x88,
    /// An action or operation failed due to insufficient available resources.
    ResourceExhausted = 0x89,
    /// The indicated data field or entry could not be found.
    NotFound = 0x8b,
    /// Reports cannot be issued for this attribute.
    UnreportableAttribute = 0x8c,
    /// The data type indicated is undefined or invalid for the indicated data field.
    /// Command or action not carried out.
    InvalidDataType = 0x8d,
    /// Attempt to read a write-only attribute.
    UnsupportedRead = 0x8f,
    /// Cluster instance data version did not match request path
    DataVersionMismatch = 0x92,
    /// The transaction was aborted due to time being exceeded.
    Timeout = 0x94,
    /// The node ID indicated is not supported on the node.
    UnsupportedNode = 0x9b,
    /// The receiver is busy processing another action that prevents the execution of the incoming action.
    Busy = 0x9c,
    /// The access to the action or command by the sender is permitted by the ACL but restricted by the ARL.
    AccessRestricted = 0x9d,
    /// The cluster indicated is not supported on the endpoint.
    UnsupportedCluster = 0xc3,
    /// Used by proxies to convey to clients the lack of an upstream subscription to a source.
    NoUpstreamSubscription = 0xc5,
    /// A Untimed Write or Untimed Invoke interaction was used for an attribute or command that requires a Timed Write or Timed Invoke.
    NeedsTimedInteraction = 0xc6,
    /// The indicated event ID is not supported on the cluster instance.
    UnsupportedEvent = 0xc7,
    /// The receiver has insufficient resources to support the specified number of paths in the request
    PathsExhausted = 0xc8,
    /// A request with TimedRequest field set to TRUE was issued outside a Timed transaction or a request with TimedRequest set to FALSE was issued inside a Timed transaction.
    TimedRequestMisMatch = 0xc9,
    /// A request requiring a Fail-safe context was invoked without the Fail-Safe context.
    FailSafeRequired = 0xca,
    /// The received request cannot be handled due to the current operational state of the device
    InvalidInState = 0xcb,
    /// A CommandDataIB is missing a response in the InvokeResponses of an Invoke Response action.
    NoCommandResponse = 0xcc,
    /// The node requires updated TC acceptance.
    /// The user MAY be directed to visit the EnhancedSetupFlowMaintenanceUrl to complete this.
    TermsAndConditionsChanged = 0xcd,
    /// The node requires the user to visit the EnhancedSetupFlowMaintenanceUrl for instructions on further action.
    MaintenanceRequired = 0xce,
}

impl Into<ErrorCode> for IMStatusCode {
    fn into(self) -> ErrorCode {
        match self {
            IMStatusCode::Success => todo!(),
            IMStatusCode::Failure => todo!(),
            IMStatusCode::InvalidSubscription => todo!(),
            IMStatusCode::UnsupportedAccess => todo!(),
            IMStatusCode::UnsupportedEndpoint => todo!(),
            IMStatusCode::InvalidAction => todo!(),
            IMStatusCode::UnsupportedCommand => todo!(),
            IMStatusCode::InvalidCommand => todo!(),
            IMStatusCode::UnsupportedAttribute => todo!(),
            IMStatusCode::ConstraintError => todo!(),
            IMStatusCode::UnsupportedWrite => todo!(),
            IMStatusCode::ResourceExhausted => todo!(),
            IMStatusCode::NotFound => todo!(),
            IMStatusCode::UnreportableAttribute => todo!(),
            IMStatusCode::InvalidDataType => todo!(),
            IMStatusCode::UnsupportedRead => todo!(),
            IMStatusCode::DataVersionMismatch => todo!(),
            IMStatusCode::Timeout => todo!(),
            IMStatusCode::UnsupportedNode => todo!(),
            IMStatusCode::Busy => todo!(),
            IMStatusCode::AccessRestricted => todo!(),
            IMStatusCode::UnsupportedCluster => todo!(),
            IMStatusCode::NoUpstreamSubscription => todo!(),
            IMStatusCode::NeedsTimedInteraction => todo!(),
            IMStatusCode::UnsupportedEvent => todo!(),
            IMStatusCode::PathsExhausted => todo!(),
            IMStatusCode::TimedRequestMisMatch => todo!(),
            IMStatusCode::FailSafeRequired => todo!(),
            IMStatusCode::InvalidInState => todo!(),
            IMStatusCode::NoCommandResponse => todo!(),
            IMStatusCode::TermsAndConditionsChanged => todo!(),
            IMStatusCode::MaintenanceRequired => todo!(),
        }
    }
}

impl From<ErrorCode> for IMStatusCode {
    fn from(e: ErrorCode) -> Self {
        match e {
            ErrorCode::EndpointNotFound => IMStatusCode::UnsupportedEndpoint,
            ErrorCode::ClusterNotFound => IMStatusCode::UnsupportedCluster,
            ErrorCode::AttributeNotFound => IMStatusCode::UnsupportedAttribute,
            ErrorCode::CommandNotFound => IMStatusCode::UnsupportedCommand,
            ErrorCode::InvalidAction => IMStatusCode::InvalidAction,
            ErrorCode::InvalidCommand => IMStatusCode::InvalidCommand,
            ErrorCode::InvalidDataType => IMStatusCode::InvalidDataType,
            ErrorCode::UnsupportedAccess => IMStatusCode::UnsupportedAccess,
            ErrorCode::Busy => IMStatusCode::Busy,
            ErrorCode::DataVersionMismatch => IMStatusCode::DataVersionMismatch,
            ErrorCode::ResourceExhausted => IMStatusCode::ResourceExhausted,
            ErrorCode::FailSafeRequired => IMStatusCode::FailSafeRequired,
            ErrorCode::ConstraintError => IMStatusCode::ConstraintError,
            ErrorCode::Failure => IMStatusCode::Failure,
            _ => IMStatusCode::Failure,
        }
    }
}

impl From<Error> for IMStatusCode {
    fn from(value: Error) -> Self {
        Self::from(value.code())
    }
}

impl FromTLV<'_> for IMStatusCode {
    fn from_tlv(t: &TLVElement) -> Result<Self, Error> {
        FromPrimitive::from_u16(t.u16()?).ok_or_else(|| ErrorCode::Invalid.into())
    }
}

impl ToTLV for IMStatusCode {
    fn to_tlv<W: TLVWrite>(&self, tag: &TLVTag, mut tw: W) -> Result<(), Error> {
        tw.u16(tag, *self as _)
    }

    fn tlv_iter(&self, tag: TLVTag) -> impl Iterator<Item = Result<TLV<'_>, Error>> {
        TLV::u16(tag, *self as _).into_tlv_iter()
    }
}

/// An enumeration of all possible opcodes used in the Interaction Model.
#[derive(FromPrimitive, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpCode {
    Reserved = 0,
    StatusResponse = 1,
    ReadRequest = 2,
    SubscribeRequest = 3,
    SubscribeResponse = 4,
    ReportData = 5,
    WriteRequest = 6,
    WriteResponse = 7,
    InvokeRequest = 8,
    InvokeResponse = 9,
    TimedRequest = 10,
}

impl OpCode {
    /// Return the opcode as a `MessageMeta` structure, which contains
    /// the protocol ID, opcode, and reliability information.
    ///
    /// Reliability is set to `true` as all IM messages are reliable.
    pub const fn meta(&self) -> MessageMeta {
        MessageMeta {
            proto_id: PROTO_ID_INTERACTION_MODEL,
            proto_opcode: *self as u8,
            reliable: true,
        }
    }

    /// Return `true` if the opcode payload is in TLV format.
    ///
    /// Currently, the payload of all IM opcodes except `Reserved` is in TLV format.
    pub const fn is_tlv(&self) -> bool {
        !matches!(self, Self::Reserved)
    }
}

impl From<OpCode> for MessageMeta {
    fn from(opcode: OpCode) -> Self {
        opcode.meta()
    }
}

// Type aliases for first-class matter types
pub type EndptId = u16;
pub type ClusterId = u32;
pub type AttrId = u32;
pub type CmdId = u32;
pub type ActionId = u8;
pub type AttributeId = u32;
pub type ClusterStatus = u8;
pub type CommandRef = u16;
pub type CompressedFabricId = u64;
pub type DataVersion = u32;
pub type DeviceTypeId = u32;
pub type ElapsedS = u32;
pub type EventId = u32;
pub type EventNumber = u64;
pub type FabricId = u64;
pub type FabricIndex = u8;
pub type FieldId = u32;
pub type ListIndex = u16;
pub type LocalizedStringIdentifier = u16;
pub type TransactionId = u32;
pub type KeysetId = u16;
pub type InteractionModelRevision = u8;
pub type SubscriptionId = u32;
pub type SceneId = u8;
pub type Percent = u8;
pub type Percent100ths = u16;
pub type EnergyMilliWh = i64;
pub type EnergyMilliVAh = i64;
pub type EnergyMilliVARh = i64;
pub type AmperageMilliA = i64;
pub type PowerMilliW = i64;
pub type PowerMilliVA = i64;
pub type PowerMilliVAR = i64;
pub type VoltageMilliV = i64;
pub type Money = i64;

/// A generic (possibly a wildcard) path with endpoint, clusters, and a leaf
///
/// The leaf could be a command, an attribute, or an event
///
/// Note that this type does not implement `FromTLV` / `ToTLV` because it does not correspond
/// to a specific TLV structure in the Interaction Model.
///
/// Note also that it only captures a _subset_ of the fields of `AttrPath`, and as such, it should be used with care!
///
/// Look at `AttrPath`, `CmdPath`, and `EventPath` for specific TLV structures, which
/// can be turned into `GenericPath` using their `to_gp()` method.
#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GenericPath {
    /// The endpoint ID, if specified, otherwise `None` for wildcard
    pub endpoint: Option<EndptId>,
    /// The cluster ID, if specified, otherwise `None` for wildcard
    pub cluster: Option<ClusterId>,
    /// The leaf ID, if specified, otherwise `None` for wildcard
    pub leaf: Option<u32>,
}

impl GenericPath {
    /// Create a new `GenericPath` with the given endpoint, cluster, and leaf.
    pub const fn new(
        endpoint: Option<EndptId>,
        cluster: Option<ClusterId>,
        leaf: Option<u32>,
    ) -> Self {
        Self {
            endpoint,
            cluster,
            leaf,
        }
    }

    /// Return Ok, if the path is non wildcard, otherwise returns an error
    pub fn not_wildcard(&self) -> Result<(EndptId, ClusterId, u32), Error> {
        match *self {
            GenericPath {
                endpoint: Some(e),
                cluster: Some(c),
                leaf: Some(l),
            } => Ok((e, c, l)),
            _ => Err(ErrorCode::Invalid.into()),
        }
    }

    /// Return true, if the path is wildcard
    pub const fn is_wildcard(&self) -> bool {
        !matches!(
            *self,
            GenericPath {
                endpoint: Some(_),
                cluster: Some(_),
                leaf: Some(_),
            }
        )
    }
}
