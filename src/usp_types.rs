pub use crate::usp_record::mod_Record::PayloadSecurity;
use clap::Parser;
use std::collections::HashMap;

/// Parse a JSON object into a Rust HashMap
fn parse_key_val_json(s: &str) -> Result<HashMap<String, String>, String> {
    serde_json::from_str::<HashMap<String, String>>(s).map_err(|e| e.to_string())
}

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

#[derive(Parser, Clone, Debug, PartialEq)]
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
        /// A stringified JSON object containing the output arguments of the USP Event
        #[clap(value_parser = parse_key_val_json)]
        params: HashMap<String, String>,
    },
    /// USP ObjectCreation notification
    ObjectCreation {
        /// The path of the created object
        obj_path: String,
        /// A stringified JSON object containing the unique_keys and values of the created Object
        #[clap(value_parser = parse_key_val_json)]
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

#[derive(Parser, Copy, Clone, Debug, PartialEq)]
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
