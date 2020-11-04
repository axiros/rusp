pub use crate::usp_record::mod_Record::PayloadSecurity;
use std::collections::HashMap;
use structopt::*;

#[derive(Clone, Debug, PartialEq)]
pub enum OperateResponse {
    OutputArgs(HashMap<String, String>),
    CommandFailure(u32, String),
}

impl Default for OperateResponse {
    fn default() -> Self {
        Self::OutputArgs(HashMap::new())
    }
}

#[derive(StructOpt, Clone, Debug, PartialEq)]
pub enum NotifyType {
    /// USP OnBoardRequest notification
    OnBoardRequest {
        /// The OUI associated with the manufacturer of the device
        oui: String,

        /// The product class associated with the device
        product_class: String,

        /// The serial number of the device
        serial_number: String,

        /// A comma separated list of supported USP versions
        agent_supported_protocol_versions: String,
    },
    /// USP ValueChange notification
    ValueChange {
        /// The path of the changed parameter
        param_path: String,
        /// The new value of the changed parameter
        param_value: String,
    },
    /// USP Event notification
    Event {
        /// The path of the event
        obj_path: String,
        /// The name of the event
        event_name: String,
        /// A list of parameter/value pairs associated with the event
        #[structopt(skip)]
        params: HashMap<String, String>,
    },
    /// USP ObjectCreation notification
    ObjectCreation {
        /// The path of the created object
        obj_path: String,
        /// A list of parameter/value pairs which are unique keys for the created object
        #[structopt(skip)]
        unique_keys: HashMap<String, String>,
    },
    /// USP ObjectDeletion notification
    ObjectDeletion {
        /// The path of the deleted object
        obj_path: String,
    },

    /// USP OperationComplete notification
    OperationComplete {
        /// The path of the operation object
        obj_path: String,
        /// The name of the operated command
        command_name: String,
        /// The command key associated with the operation
        command_key: String,
        /// The result of the operation
        #[structopt(skip)]
        operation_resp: OperateResponse,
    },
}

#[derive(StructOpt, Copy, Clone, Debug, PartialEq)]
pub enum PayloadSARState {
    /// No segmentation
    NONE = 0,
    /// Begin segmentation
    BEGIN = 1,
    /// Segmentation in process
    INPROCESS = 2,
    /// Segmentation is complete
    COMPLETE = 3,
}

impl From<PayloadSARState> for crate::usp_record::mod_SessionContextRecord::PayloadSARState {
    fn from(s: PayloadSARState) -> Self {
        use crate::usp_record::mod_SessionContextRecord::PayloadSARState as PBPayloadSARState;
        match s {
            PayloadSARState::NONE => PBPayloadSARState::NONE,
            PayloadSARState::BEGIN => PBPayloadSARState::BEGIN,
            PayloadSARState::INPROCESS => PBPayloadSARState::INPROCESS,
            PayloadSARState::COMPLETE => PBPayloadSARState::COMPLETE,
        }
    }
}
