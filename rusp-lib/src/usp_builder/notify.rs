use std::collections::HashMap;

use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp;
use crate::usp::mod_Notify::OneOfnotification::{
    event, obj_creation, obj_deletion, on_board_req, oper_complete, value_change,
};
use crate::usp::mod_Notify::{
    Event, ObjectCreation, ObjectDeletion, OnBoardRequest, OperationComplete, ValueChange,
};
use crate::usp::mod_Request::OneOfreq_type::notify;
use crate::usp::mod_Response::OneOfresp_type::notify_resp;
use crate::usp::{Body, Notify, Request};
use crate::usp::{NotifyResp, Response};

use anyhow::{Context, Result};

#[derive(Clone)]
pub enum OperationCompleteType {
    OutputArgs(HashMap<String, String>),
    CommandFailure(u32, String),
}

#[derive(Clone)]
pub enum NotifyType {
    /// USP `OnBoardRequest` notification
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
    /// USP `ValueChange` notification
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
        params: HashMap<String, String>,
    },
    /// USP `ObjectCreation` notification
    ObjectCreation {
        /// The path of the created object
        obj_path: String,
        /// A stringified JSON object containing the `unique_keys` and values of the created Object
        unique_keys: HashMap<String, String>,
    },
    /// USP `ObjectDeletion` notification
    ObjectDeletion {
        /// The path of the deleted object
        obj_path: String,
    },

    /// USP `OperationComplete` notification
    OperationComplete {
        /// The path of the operation object
        obj_path: String,
        /// The name of the operated command
        command_name: String,
        /// The command key associated with the operation
        command_key: String,
        /// The result of the operation
        operation_resp: OperationCompleteType,
    },
}

#[derive(Clone)]
pub struct NotifyBuilder {
    subscription_id: String,
    send_resp: bool,
    notify_type: Option<NotifyType>,
}

impl NotifyBuilder {
    #[must_use]
    pub const fn new(subscription_id: String) -> Self {
        Self {
            subscription_id,
            send_resp: false,
            notify_type: None,
        }
    }

    #[must_use]
    pub const fn with_send_resp(mut self, send_resp: bool) -> Self {
        self.send_resp = send_resp;
        self
    }

    #[must_use]
    pub fn with_onboard_request(
        mut self,
        oui: String,
        product_class: String,
        serial_number: String,
        aspv: String,
    ) -> Self {
        self.notify_type = Some(NotifyType::OnBoardRequest {
            oui,
            product_class,
            serial_number,
            agent_supported_protocol_versions: aspv,
        });
        self
    }

    #[must_use]
    pub fn with_value_change(mut self, param_path: String, param_value: String) -> Self {
        self.notify_type = Some(NotifyType::ValueChange {
            param_path,
            param_value,
        });
        self
    }

    #[must_use]
    pub fn with_event(
        mut self,
        obj_path: String,
        event_name: String,
        params: HashMap<String, String>,
    ) -> Self {
        self.notify_type = Some(NotifyType::Event {
            obj_path,
            event_name,
            params,
        });
        self
    }

    #[must_use]
    pub fn with_object_creation(
        mut self,
        obj_path: String,
        unique_keys: HashMap<String, String>,
    ) -> Self {
        self.notify_type = Some(NotifyType::ObjectCreation {
            obj_path,
            unique_keys,
        });
        self
    }

    #[must_use]
    pub fn with_object_deletion(mut self, obj_path: String) -> Self {
        self.notify_type = Some(NotifyType::ObjectDeletion { obj_path });
        self
    }

    #[must_use]
    pub fn with_operation_complete_output_args(
        mut self,
        obj_path: String,
        command_name: String,
        command_key: String,
        output_args: HashMap<String, String>,
    ) -> Self {
        self.notify_type = Some(NotifyType::OperationComplete {
            obj_path,
            command_name,
            command_key,
            operation_resp: OperationCompleteType::OutputArgs(output_args),
        });
        self
    }

    #[must_use]
    pub fn with_operation_complete_cmd_failure(
        mut self,
        obj_path: String,
        command_name: String,
        command_key: String,
        err_code: u32,
        err_msg: String,
    ) -> Self {
        self.notify_type = Some(NotifyType::OperationComplete {
            obj_path,
            command_name,
            command_key,
            operation_resp: OperationCompleteType::CommandFailure(err_code, err_msg),
        });
        self
    }

    pub fn build(self) -> Result<Body> {
        let notify_type = self
            .notify_type
            .context("Must specify a notification type")?;

        let notify_type = match notify_type {
            NotifyType::OnBoardRequest {
                oui,
                product_class,
                serial_number,
                agent_supported_protocol_versions,
            } => on_board_req(OnBoardRequest {
                agent_supported_protocol_versions,
                oui,
                product_class,
                serial_number,
            }),
            NotifyType::ValueChange {
                param_path,
                param_value,
            } => value_change(ValueChange {
                param_path,
                param_value,
            }),
            NotifyType::Event {
                obj_path,
                event_name,
                params,
            } => event(Event {
                obj_path,
                event_name,
                params: params.into_iter().collect::<HashMap<_, _>>(),
            }),
            NotifyType::ObjectCreation {
                obj_path,
                unique_keys,
            } => obj_creation(ObjectCreation {
                obj_path,
                unique_keys: unique_keys.into_iter().collect::<HashMap<_, _>>(),
            }),
            NotifyType::ObjectDeletion { obj_path } => obj_deletion(ObjectDeletion { obj_path }),
            NotifyType::OperationComplete {
                obj_path,
                command_name,
                command_key,
                operation_resp,
            } => oper_complete(OperationComplete {
                obj_path,
                command_name,
                command_key,
                operation_resp: match operation_resp {
                    OperationCompleteType::OutputArgs(h) => OneOfoperation_resp::req_output_args(
                        crate::usp::mod_Notify::mod_OperationComplete::OutputArgs {
                            output_args: h.into_iter().collect::<HashMap<_, _>>(),
                        },
                    ),
                    OperationCompleteType::CommandFailure(code, msg) => {
                        OneOfoperation_resp::cmd_failure(
                            crate::usp::mod_Notify::mod_OperationComplete::CommandFailure {
                                err_code: code,
                                err_msg: msg,
                            },
                        )
                    }
                },
            }),
        };

        Ok(Body {
            msg_body: request({
                Request {
                    req_type: notify({
                        Notify {
                            subscription_id: self.subscription_id,
                            send_resp: self.send_resp,
                            notification: notify_type,
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct NotifyRespBuilder {
    subscription_id: String,
}

impl NotifyRespBuilder {
    #[must_use]
    pub const fn new(subscription_id: String) -> Self {
        Self { subscription_id }
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: notify_resp({
                        NotifyResp {
                            subscription_id: self.subscription_id,
                        }
                    }),
                }
            }),
        })
    }
}
