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

use core::borrow::Borrow;
use core::future::Future;
use core::mem::MaybeUninit;

use num_derive::FromPrimitive;

use crate::error::*;
use crate::respond::ExchangeHandler;
use crate::tlv::{FromTLV, ToTLV};
use crate::transport::exchange::{Exchange, MessageMeta};
use crate::utils::init::InitMaybeUninit;
use crate::utils::storage::{ReadBuf, WriteBuf};

use case::Case;
use pake::Pake;

pub mod busy;
pub mod case;
pub mod crypto;
pub mod pake;

/* Interaction Model ID as per the Matter Spec */
pub const PROTO_ID_SECURE_CHANNEL: u16 = 0x00;

#[derive(FromPrimitive, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpCode {
    MsgCounterSyncReq = 0x00,
    MsgCounterSyncResp = 0x01,
    MRPStandAloneAck = 0x10,
    PBKDFParamRequest = 0x20,
    PBKDFParamResponse = 0x21,
    PASEPake1 = 0x22,
    PASEPake2 = 0x23,
    PASEPake3 = 0x24,
    CASESigma1 = 0x30,
    CASESigma2 = 0x31,
    CASESigma3 = 0x32,
    CASESigma2Resume = 0x33,
    StatusReport = 0x40,
}

impl OpCode {
    pub fn meta(&self) -> MessageMeta {
        MessageMeta {
            proto_id: PROTO_ID_SECURE_CHANNEL,
            proto_opcode: *self as u8,
            reliable: !matches!(self, Self::MRPStandAloneAck),
        }
    }

    pub fn is_tlv(&self) -> bool {
        !matches!(
            self,
            Self::MRPStandAloneAck
                | Self::StatusReport
                | Self::MsgCounterSyncReq
                | Self::MsgCounterSyncResp
        )
    }
}

impl From<OpCode> for MessageMeta {
    fn from(op: OpCode) -> Self {
        op.meta()
    }
}

/// Secure Channel Status Report Protocol Specific codes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCStatusCodes {
    /// Indication that the last session establishment message was successfully processed.
    SessionEstablishmentSuccess = 0,
    /// Failure to find a common set of shared roots.
    NoSharedTrustRoots = 1,
    /// Generic failure during session establishment.
    InvalidParameter = 2,
    /// Indication that the sender will close the current session.
    CloseSession = 3,
    /// Indication that the sender cannot currently fulfill the request.
    Busy = 4,
    /// Required CAT Mismatch during session establishment at the Sigma2 Validation step.
    RequiredCatMismatch = 5,
    // todo Where is this error defined? This does not seem to exist in the Matter spec.
    SessionNotFound = 6,
}

impl SCStatusCodes {
    pub fn reliable(&self) -> bool {
        // CloseSession and Busy are sent without the R flag raised
        !matches!(self, SCStatusCodes::CloseSession | SCStatusCodes::Busy)
    }

    pub fn as_report<'a>(&self, payload: &'a [u8]) -> StatusReport<'a> {
        let general_code = match self {
            SCStatusCodes::SessionEstablishmentSuccess => GeneralCode::Success,
            SCStatusCodes::CloseSession => GeneralCode::Success,
            SCStatusCodes::Busy => GeneralCode::Busy,
            SCStatusCodes::InvalidParameter
            | SCStatusCodes::NoSharedTrustRoots
            | SCStatusCodes::RequiredCatMismatch
            | SCStatusCodes::SessionNotFound => GeneralCode::Failure,
        };

        StatusReport {
            general_code,
            proto_id: PROTO_ID_SECURE_CHANNEL as u32,
            proto_code: *self as u16,
            proto_data: payload,
        }
    }
}

pub async fn complete_with_status(
    exchange: &mut Exchange<'_>,
    status_code: SCStatusCodes,
    payload: &[u8],
) -> Result<(), Error> {
    exchange
        .send_with(|_, wb| sc_write(wb, status_code, payload))
        .await
}

pub fn sc_write(
    wb: &mut WriteBuf,
    status_code: SCStatusCodes,
    payload: &[u8],
) -> Result<Option<MessageMeta>, Error> {
    status_code.as_report(payload).write(wb)?;

    Ok(Some(
        OpCode::StatusReport.meta().reliable(status_code.reliable()),
    ))
}

/// General status codes conveyed in the GeneralCode field of a Status Report message.
/// These are uniform codes that convey both success and failures.
#[allow(dead_code)]
#[derive(FromPrimitive, PartialEq, Eq, Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GeneralCode {
    /// Operation completed successfully.
    Success = 0,
    /// Generic failure, additional details may be included in the _protocol specific status_.
    Failure = 1,
    /// Operation was rejected by the system because the system is in an invalid state.
    BadPrecondition = 2,
    /// A value was out of a required range.
    OutOfRange = 3,
    /// A request was unrecognized or malformed.
    BadRequest = 4,
    /// An unrecognized or unsupported request was received.
    Unsupported = 5,
    /// A request was not expected at this time.
    Unexpected = 6,
    /// Insufficient resources to process the given request.
    ResourceExhausted = 7,
    /// Device is busy and cannot handle this request at this time.
    Busy = 8,
    /// A timeout occurred.
    Timeout = 9,
    ///  Context-specific signal to proceed.
    Continue = 10,
    /// Failure, may be due to a concurrency error.
    Aborted = 11,
    /// An invalid/unsupported argument was provided.
    InvalidArgument = 12,
    ///  Some requested entity was not found.
    NotFound = 13,
    ///  The sender attempted to create something that already exists.
    AlreadyExists = 14,
    ///  The sender does not have sufficient permissions to execute the requested operations.
    PermissionDenied = 15,
    /// Unrecoverable data loss or corruption has occurred.
    DataLoss = 16,
    /// Message size is larger than the recipient can handle.
    MessageTooLarge = 17,
}

/// Represents the session parameters
/// that might present in a "PBKDFParamRequest" or "CASE-Sigma1" message
#[derive(FromTLV, ToTLV, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[tlvargs(start = 1)]
struct SessionParameters {
    /// Session Idle Interval
    sii: Option<u32>,
    /// Session Active Interval
    sai: Option<u32>,
    /// Session Active Threshold
    sat: Option<u16>,
    /// Data Model Revision
    dm_revision: Option<u16>,
    /// Interaction Model Revision
    im_revision: Option<u16>,
    /// Specification Version
    spec_version: Option<u32>,
    /// Maximum number of paths per invoke
    max_paths_per_invoke: Option<u16>,
}

/// Represents a Status Report message, as per "Appendix D: Status Report Messages" of the Matter Spec.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusReport<'a> {
    pub general_code: GeneralCode,
    pub proto_id: u32,
    pub proto_code: u16,
    pub proto_data: &'a [u8],
}

impl<'a> StatusReport<'a> {
    pub fn read<T>(pb: &'a mut ReadBuf<T>) -> Result<Self, Error>
    where
        T: Borrow<[u8]>,
    {
        Ok(Self {
            general_code: num::FromPrimitive::from_u16(pb.le_u16()?)
                .ok_or(ErrorCode::InvalidOpcode)?,
            proto_id: pb.le_u32()?,
            proto_code: pb.le_u16()?,
            proto_data: pb.as_slice(),
        })
    }

    pub fn write(&self, wb: &mut WriteBuf) -> Result<(), Error> {
        wb.le_u16(self.general_code as u16)?;
        wb.le_u32(self.proto_id)?;
        wb.le_u16(self.proto_code)?;
        wb.copy_from_slice(self.proto_data)?;

        Ok(())
    }
}

/// Handle messages related to the Secure Channel
pub struct SecureChannel(());

impl SecureChannel {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(())
    }

    pub async fn handle(&self, exchange: &mut Exchange<'_>) -> Result<(), Error> {
        if exchange.rx().is_err() {
            exchange.recv_fetch().await?;
        }

        let meta = exchange.rx()?.meta();
        if meta.proto_id != PROTO_ID_SECURE_CHANNEL {
            Err(ErrorCode::InvalidProto)?;
        }

        match meta.opcode()? {
            OpCode::PBKDFParamRequest => {
                let mut pake = MaybeUninit::uninit(); // TODO LARGE BUFFER
                pake.init_with(Pake::init()).handle(exchange).await
            }
            OpCode::CASESigma1 => {
                let mut case = MaybeUninit::uninit(); // TODO LARGE BUFFER
                case.init_with(Case::init()).handle(exchange).await
            }
            opcode => {
                error!("Invalid opcode: {:?}", opcode);
                Err(ErrorCode::InvalidOpcode.into())
            }
        }
    }
}

impl Default for SecureChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeHandler for SecureChannel {
    fn handle(&self, exchange: &mut Exchange<'_>) -> impl Future<Output = Result<(), Error>> {
        SecureChannel::handle(self, exchange)
    }
}

/// Check that the opcode of the received message matches the expected one.
/// Logs an error if that's not the case, and if the opcode is `StatusReport`,
/// it also logs the details of the status report.
fn check_opcode(exchange: &Exchange<'_>, opcode: OpCode) -> Result<(), Error> {
    let meta = exchange.rx()?.meta();
    let their_opcode = meta.opcode::<OpCode>()?;

    if their_opcode == opcode {
        Ok(())
    } else {
        error!("Invalid opcode: {:?}, expected: {:?}", their_opcode, opcode);

        if matches!(their_opcode, OpCode::StatusReport) {
            let mut rb = ReadBuf::new(exchange.rx()?.payload());

            // Show the status code details in the log
            match StatusReport::read(&mut rb) {
                Ok(status_report) => error!("Status Report: {:?}", status_report),
                Err(e) => error!("Failed to parse Status Report: {:?}", e),
            }
        }

        Err(ErrorCode::Invalid.into())
    }
}
