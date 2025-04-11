//! `rhai-rusp` offers `Rhai` bindings for the `Rust` `USP` (or
//! [`rusp`](https://crates.io/crates/rusp) library to for comfortable introspection, creation, and
//! manipulation of [`USP`](https://usp.technology) protocol communication primitives.
use rhai::def_package;
use rhai::{
    plugin::{
        combine_with_exported_module, export_module, mem, Dynamic, EvalAltResult, FnNamespace,
        FuncRegistration, ImmutableString, Module, NativeCallContext, PluginFunc, RhaiResult,
        TypeId,
    },
    Array, Map, Variant,
};
use rusp_lib::usp::{Body, Msg};
use rusp_lib::usp_builder;
use rusp_lib::usp_record::{self, Record};

/// Evaluate a Rhai script in the context of the `rusp` package and return a supported type, like
/// [`Record`], [`Msg`] or [`String`]
///
/// E.g. you can do:
/// ```
/// let script = r#"
/// // Rhai script
/// rusp::record_builder()
///   .with_to_id("proto::to")
///   .with_from_id("proto::from")
///   .as_disconnect_record("Bye", 0)
///   .build()
/// "#;
/// let record = rhai_rusp::eval_rusp::<rusp_lib::usp_record::Record>(script).unwrap();
/// ```
///
/// # Errors
///
/// This function will return `Err` containing a textual description of the encountered error if
/// the provided Rhai script fails to evaluate.
pub fn eval_rusp<T>(str: &str) -> Result<T, String>
where
    T: Variant + Clone,
{
    use rhai::{packages::Package, Engine};

    let mut engine = Engine::new();
    engine.register_static_module("rusp", RuspPackage::new().as_shared_module());

    engine.eval::<T>(str).map_err(|e| e.to_string())
}

/// Supply Rusp [`Record`] generation functionality
///
/// The general usage pattern from within a Rhai script is straight forward: First you obtain a
/// builder type by calling `rusp::record_builder()`, then add data using any of the available
/// builder methods and in the end obtain the USP [`Record`] structure by calling `build()`.
///
/// Here's a full example:
/// ```
/// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type,MQTTConnectRecord,mod_MQTTConnectRecord::MQTTVersion,Record};
/// // Rhai script
/// # let script = r#"
/// rusp::record_builder()
///   .with_version("1.3")
///   .with_to_id("proto::to")
///   .with_from_id("proto::from")
///   .as_mqtt_connect_record("V5", "/topic")
///   .build()
/// # "#;
/// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
/// # assert_eq!(record.to_id, "proto::to");
/// # assert_eq!(record.from_id, "proto::from");
/// # assert_eq!(record.record_type, OneOfrecord_type::mqtt_connect(MQTTConnectRecord { subscribed_topic: "/topic".into(), version: MQTTVersion::V5 }));
/// ```
#[export_module]
pub mod rhai_rusp_record {
    use usp_builder::RecordBuilder;

    /// Sets up a new USP `RecordBuilder`
    #[must_use]
    pub fn record_builder() -> RecordBuilder {
        RecordBuilder::new()
    }

    /// Sets the version of the USP standard being used by the [`Record`]
    /// ```
    /// # use rusp_lib::usp_record::Record;
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_version("1.3")
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script);
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_version(builder: RecordBuilder, version: &str) -> RecordBuilder {
        builder.with_version(version.into())
    }

    /// Sets the recipient endpoint id of the [`Record`]
    /// ```
    /// # use rusp_lib::usp_record::Record;
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_to_id("proto::controller")
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script);
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_to_id(builder: RecordBuilder, id: &str) -> RecordBuilder {
        builder.with_to_id(id.into())
    }

    /// Sets the sender endpoint id of the [`Record`]
    /// ```
    /// # use rusp_lib::usp_record::Record;
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_from_id("proto::agent")
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script);
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_from_id(builder: RecordBuilder, id: &str) -> RecordBuilder {
        builder.with_from_id(id.into())
    }

    /// Assigns the provided USP [`Msg`] as the "no session context" payload of the USP [`Record`]
    /// ```
    /// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type, NoSessionContextRecord, Record};
    ///
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::get_builder()
    ///   .with_params(["Device."])
    ///   .build();
    /// let msg = rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build();
    /// rusp::record_builder()
    ///   .with_version("1.2")
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .with_no_session_context_payload(msg)
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
    ///
    /// # assert_eq!(record.version, "1.2");
    /// # assert_eq!(record.to_id, "proto::to");
    /// # assert_eq!(record.from_id, "proto::from");
    /// # assert!(matches!(record.record_type, OneOfrecord_type::no_session_context(NoSessionContextRecord { .. })));
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_no_session_context_payload(builder: RecordBuilder, payload: Msg) -> RecordBuilder {
        builder.with_no_session_context_payload(&payload)
    }

    /// Designates the USP [`Record`] to be of type WebSocketConnectRecord
    /// ```
    /// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type,WebSocketConnectRecord,Record};
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_version("1.3")
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_websocket_connect_record()
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
    /// # assert_eq!(record.to_id, "proto::to");
    /// # assert_eq!(record.from_id, "proto::from");
    /// # assert_eq!(record.record_type, OneOfrecord_type::websocket_connect(WebSocketConnectRecord { }));
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn as_websocket_connect_record(builder: RecordBuilder) -> RecordBuilder {
        builder.as_websocket_connect_record()
    }

    /// Designates the USP [`Record`] to be of type MQTTConnectRecord
    /// ```
    /// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type,MQTTConnectRecord,mod_MQTTConnectRecord::MQTTVersion,Record};
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_mqtt_connect_record("V5", "/topic")
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
    /// # assert_eq!(record.to_id, "proto::to");
    /// # assert_eq!(record.from_id, "proto::from");
    /// # assert_eq!(record.record_type, OneOfrecord_type::mqtt_connect(MQTTConnectRecord { subscribed_topic: "/topic".into(), version: MQTTVersion::V5 }));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `version` is not `V3_1_1` or `V5`.
    #[rhai_fn(global, return_raw)]
    pub fn as_mqtt_connect_record(
        builder: RecordBuilder,
        version: &str,
        subscribed_topic: &str,
    ) -> Result<RecordBuilder, Box<EvalAltResult>> {
        use usp_record::mod_MQTTConnectRecord::MQTTVersion;

        let version = match version {
            "V3_1_1" | "V5" => MQTTVersion::from(version),
            _ => return Err("MQTT version must be either V3_1_1 or V5".into()),
        };

        Ok(builder.as_mqtt_connect_record(version, subscribed_topic.into()))
    }

    /// Designates the USP [`Record`] to be of type STOMPConnectRecord
    /// ```
    /// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type,STOMPConnectRecord,mod_STOMPConnectRecord::STOMPVersion,Record};
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_stomp_connect_record("V1_2", "/dest")
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
    /// # assert_eq!(record.to_id, "proto::to");
    /// # assert_eq!(record.from_id, "proto::from");
    /// # assert_eq!(record.record_type, OneOfrecord_type::stomp_connect(STOMPConnectRecord { subscribed_destination: "/dest".into(), version: STOMPVersion::V1_2 }));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `version` is not `V1_2`.
    #[rhai_fn(global, return_raw)]
    pub fn as_stomp_connect_record(
        builder: RecordBuilder,
        version: &str,
        subscribed_destination: &str,
    ) -> Result<RecordBuilder, Box<EvalAltResult>> {
        use usp_record::mod_STOMPConnectRecord::STOMPVersion;

        let version = match version {
            "V1_2" => STOMPVersion::from(version),
            _ => return Err("STOMP version must be V1_2".into()),
        };

        Ok(builder.as_stomp_connect_record(version, subscribed_destination.into()))
    }

    /// Designates the USP [`Record`] to be of type DisconnectRecord
    /// ```
    /// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type,DisconnectRecord,Record};
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_disconnect_record("Bye", 0)
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
    /// # assert_eq!(record.to_id, "proto::to");
    /// # assert_eq!(record.from_id, "proto::from");
    /// # assert_eq!(record.record_type, OneOfrecord_type::disconnect(DisconnectRecord { reason: "Bye".into(), reason_code: 0 }));
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn as_disconnect_record(
        builder: RecordBuilder,
        reason: &str,
        reason_code: i64,
    ) -> RecordBuilder {
        builder.as_disconnect_record(reason.into(), u32::try_from(reason_code).unwrap_or(7003))
    }

    /// Designates the USP [`Record`] to be of type UDSConnectRecord
    /// ```
    /// # use rusp_lib::usp_record::{mod_Record::OneOfrecord_type,UDSConnectRecord,Record};
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_uds_connect_record()
    ///   .build()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<Record>(script).unwrap();
    /// # assert_eq!(record.to_id, "proto::to");
    /// # assert_eq!(record.from_id, "proto::from");
    /// # assert_eq!(record.record_type, OneOfrecord_type::uds_connect(UDSConnectRecord {}));
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn as_uds_connect_record(builder: RecordBuilder) -> RecordBuilder {
        builder.as_uds_connect_record()
    }

    /// Turns the builder into a USP [`Record`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: RecordBuilder) -> Result<Record, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp [`Msg`] generation functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .build()
/// "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script);
///
/// // This will result in an `Err`:
/// // Err("Runtime error: Cannot produce USP Msg without msg_body (line 8, position 10)")
/// # assert!(msg.is_err());
/// ```
///
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::get_builder()
///   .with_params(["Device."])
///   .build();
/// rusp::msg_builder()
///   .with_body(body)
///   .build()
/// "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script);
///
/// // This will result in an `Err`:
/// // Err("Runtime error: Cannot produce USP Msg without msg_id (line 8, position 10)")
/// # assert!(msg.is_err());
/// ```
///
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::get_builder()
///   .with_params(["Device."])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// ```
#[export_module]
pub mod rhai_rusp_msg {
    use usp_builder::MsgBuilder;

    /// Sets up a new USP `MsgBuilder`
    #[must_use]
    pub fn msg_builder() -> MsgBuilder {
        MsgBuilder::new()
    }

    /// Sets the `msg_id` of the USP [`Msg`]
    /// ```
    /// # use rusp_lib::usp::Msg;
    /// // Rhai script
    /// # let script = r#"
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<Msg>(script);
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_msg_id(builder: MsgBuilder, msg_id: &str) -> MsgBuilder {
        builder.with_msg_id(msg_id.into())
    }

    /// Sets the `body` of the USP [`Msg`]
    /// ```
    /// # use rusp_lib::usp::Msg;
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::get_builder()
    ///   .with_params(["Device."])
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<Msg>(script);
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_body(builder: MsgBuilder, body: Body) -> MsgBuilder {
        builder.with_body(body)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: MsgBuilder) -> Result<Msg, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Delete Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::delete_builder()
///   .with_allow_partial(true)
///   .with_obj_paths(["Device.Foo.1.", "Device.Bar.2"])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_delete {
    use usp_builder::DeleteBuilder;

    /// Sets up a new USP `DeleteBuilder`
    #[must_use]
    pub fn delete_builder() -> DeleteBuilder {
        DeleteBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_allow_partial(builder: DeleteBuilder, allow_partial: bool) -> DeleteBuilder {
        builder.with_allow_partial(allow_partial)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_obj_paths(builder: DeleteBuilder, obj_paths: Array) -> DeleteBuilder {
        builder.with_obj_paths(obj_paths.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: DeleteBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp DeleteResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let deleted_objs = [
///   rusp::deleteresp_oper_failure("Foo", 7004, ""),
///   rusp::deleteresp_oper_success("Foo.Bar.", ["Foo.Bar.1", "Foo.Bar.2."], [["Foo.Bar.3.", 7004, ""]])
/// ];
/// let body = rusp::deleteresp_builder()
///   .with_deleted_obj_results(deleted_objs)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_deleteresp {
    use usp_builder::{
        DeleteRespBuilder, DeleteRespUnaffectedPathError, DeletedObjectResultsBuilder,
    };

    /// Sets up a new USP `DeleteRespBuilder`
    #[must_use]
    pub fn deleteresp_builder() -> DeleteRespBuilder {
        DeleteRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_deleted_obj_results(
        builder: DeleteRespBuilder,
        deleted_obj_results: Array,
    ) -> DeleteRespBuilder {
        let deleted_obj_results = deleted_obj_results.into_iter().map(Dynamic::cast).collect();
        builder.with_deleted_obj_results(deleted_obj_results)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn deleteresp_oper_failure(
        requested_path: &str,
        err_code: i64,
        err_msg: &str,
    ) -> DeletedObjectResultsBuilder {
        DeletedObjectResultsBuilder::new(requested_path.into()).set_failure(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    /// # Errors
    ///
    /// This function will return `Err` if the provided `unaffected_path_errs` cannot be converted
    /// into an array of type `(String, u32, String)` or `affected_paths` cannot be converted into
    /// an array of type `String`.
    #[rhai_fn(global, return_raw)]
    pub fn deleteresp_oper_success(
        requested_path: &str,
        affected_paths: Array,
        unaffected_path_errs: Array,
    ) -> Result<DeletedObjectResultsBuilder, Box<EvalAltResult>> {
        let affected_paths = affected_paths
            .into_iter()
            .map(|p| {
                p.try_cast::<String>()
                    .ok_or("Expected to have an array of Strings".into())
            })
            .collect::<Result<Vec<String>, Box<EvalAltResult>>>()?;

        let unaffected_path_errs = unaffected_path_errs
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, err_code: u32, err_msg: &str]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<i64>()
                    .ok_or("err_code (#2) needs to be a u32".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("err_msg (#3) needs to be a string".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok(DeleteRespUnaffectedPathError{unaffected_path: el0, err_code: u32::try_from(el1).unwrap_or(7003), err_msg: el2 }),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<DeleteRespUnaffectedPathError>, Box<EvalAltResult>>>()?;

        Ok(DeletedObjectResultsBuilder::new(requested_path.into())
            .set_success(affected_paths, unaffected_path_errs))
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: DeleteRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Deregister Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::deregister_builder()
///   .with_req_paths(["Device.Foo.", "Device.Bar."])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_deregister {
    use usp_builder::DeregisterBuilder;

    /// Sets up a new USP `DeregisterBuilder`
    #[must_use]
    pub fn deregister_builder() -> DeregisterBuilder {
        DeregisterBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_req_paths(builder: DeregisterBuilder, paths: Array) -> DeregisterBuilder {
        builder.with_paths(paths.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: DeregisterBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp DeregistertResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let regpathres = [];
/// regpathres += rusp::deregistered_path_result_builder("Device.")
///   .set_failure(7002, "Look, a fancy error");
/// regpathres += rusp::deregistered_path_result_builder("Device.")
///   .set_success(["Device.Foo."]);
/// let body = rusp::deregisterresp_builder()
///   .with_deregistered_path_results(regpathres)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_deregisterresp {
    use usp_builder::{DeregisterRespBuilder, DeregisteredPathResultBuilder};

    /// Sets up a new USP `DeregisterRespBuilder`
    #[must_use]
    pub fn deregisterresp_builder() -> DeregisterRespBuilder {
        DeregisterRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_deregistered_path_results(
        builder: DeregisterRespBuilder,
        deregistered_path_results: Array,
    ) -> DeregisterRespBuilder {
        builder.with_deregistered_path_results(
            deregistered_path_results
                .into_iter()
                .map(Dynamic::cast)
                .collect(),
        )
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: DeregisterRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }

    #[must_use]
    pub fn deregistered_path_result_builder(requested_path: &str) -> DeregisteredPathResultBuilder {
        DeregisteredPathResultBuilder::new(requested_path.into())
    }

    #[rhai_fn(global, name = "set_failure")]
    #[must_use]
    pub fn deregisterresp_set_failure(
        builder: DeregisteredPathResultBuilder,
        err_code: i64,
        err_msg: &str,
    ) -> DeregisteredPathResultBuilder {
        builder.set_failure(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    #[rhai_fn(global, name = "set_success")]
    #[must_use]
    pub fn deregisterresp_set_success(
        builder: DeregisteredPathResultBuilder,
        deregistered_path: Array,
    ) -> DeregisteredPathResultBuilder {
        builder.set_success(deregistered_path.into_iter().map(Dynamic::cast).collect())
    }
}

/// Supply Rusp Register Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::register_builder()
///   .with_allow_partial(false)
///   .with_reg_paths(["Device.Foo.", "Device.Bar."])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_register {
    use usp_builder::RegisterBuilder;

    /// Sets up a new USP `RegisterBuilder`
    #[must_use]
    pub fn register_builder() -> RegisterBuilder {
        RegisterBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_allow_partial(builder: RegisterBuilder, allow_partial: bool) -> RegisterBuilder {
        builder.with_allow_partial(allow_partial)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_reg_paths(builder: RegisterBuilder, req_paths: Array) -> RegisterBuilder {
        builder.with_reg_paths(req_paths.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: RegisterBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp RegisterResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let regpathres = [];
/// regpathres += rusp::registered_path_result_builder("Device.")
///   .set_failure(7002, "Look, a fancy error");
/// regpathres += rusp::registered_path_result_builder("Device.")
///   .set_success("Device.Foo.");
/// let body = rusp::registerresp_builder()
///   .with_registered_path_results(regpathres)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_registerresp {
    use usp_builder::{RegisterRespBuilder, RegisteredPathResultBuilder};

    /// Sets up a new USP `RegisterRespBuilder`
    #[must_use]
    pub fn registerresp_builder() -> RegisterRespBuilder {
        RegisterRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_registered_path_results(
        builder: RegisterRespBuilder,
        registered_path_results: Array,
    ) -> RegisterRespBuilder {
        builder.with_registered_path_results(
            registered_path_results
                .into_iter()
                .map(Dynamic::cast)
                .collect(),
        )
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: RegisterRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }

    #[must_use]
    pub fn registered_path_result_builder(requested_path: &str) -> RegisteredPathResultBuilder {
        RegisteredPathResultBuilder::new(requested_path.into())
    }

    #[rhai_fn(global, name = "set_failure")]
    #[must_use]
    pub fn registerresp_set_failure(
        builder: RegisteredPathResultBuilder,
        err_code: i64,
        err_msg: &str,
    ) -> RegisteredPathResultBuilder {
        builder.set_failure(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    #[rhai_fn(global, name = "set_success")]
    #[must_use]
    pub fn registerresp_set_success(
        builder: RegisteredPathResultBuilder,
        registered_path: &str,
    ) -> RegisteredPathResultBuilder {
        builder.set_success(registered_path.into())
    }
}

/// Supply Rusp Set Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let update_obj = rusp::set_update_object_builder("Device.IP.Interface.")
///   .with_param_settings([["Foo", "Bar", true],["Whee", "What?", false]]);
/// let body = rusp::set_builder()
///   .with_allow_partial(true)
///   .with_update_objs([update_obj])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_set {
    use usp_builder::{SetBuilder, UpdateObjectBuilder};

    /// Sets up a new USP `SetBuilder`
    #[must_use]
    pub fn set_builder() -> SetBuilder {
        SetBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_allow_partial(builder: SetBuilder, allow_partial: bool) -> SetBuilder {
        builder.with_allow_partial(allow_partial)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_update_objs(builder: SetBuilder, create_objs: Array) -> SetBuilder {
        builder.with_update_objs(create_objs.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_update_object_builder(obj_path: &str) -> UpdateObjectBuilder {
        UpdateObjectBuilder::new(obj_path.into())
    }

    #[rhai_fn(global, return_raw)]
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `param_settings` cannot be converted into
    /// an array of type `(String, String, bool)`.
    pub fn with_param_settings(
        mut builder: UpdateObjectBuilder,
        param_settings: Array,
    ) -> Result<UpdateObjectBuilder, Box<EvalAltResult>> {
        let param_settings = param_settings
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, value: &str, required: bool]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("value (#2) needs to be a string".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<bool>()
                    .ok_or("required (#3) needs to be a bool".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok((el0, el1, el2)),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<(String, String, bool)>, Box<EvalAltResult>>>()?;
        builder = builder.with_param_settings(param_settings);

        Ok(builder)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: SetBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp SetResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let failure = setresp_updated_instance_failure_builder("Foo.Bar.")
///   .with_param_errs ([["Baz", 7002, ""]]);
/// let updated_objs = [
///   rusp::setresp_updated_obj_failure("Foo.Bar.", 0, "", [failure]),
///   rusp::setresp_updated_obj_success("Foo", "Foo.Bar.1", [["Bar", 7004, ""]], #{"Foo": "Bar"})
/// ];
/// let body = rusp::setresp_builder()
///   .with_updated_obj_results(updated_objs)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_setresp {
    use usp_builder::{
        SetOperationStatus, SetOperationSuccessBuilder, SetRespBuilder, SetRespParameterError,
        UpdatedInstanceFailureBuilder, UpdatedObjectResultsBuilder,
    };

    /// Sets up a new USP `SetRespBuilder`
    #[must_use]
    pub fn setresp_builder() -> SetRespBuilder {
        SetRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_updated_obj_results(
        builder: SetRespBuilder,
        updated_obj_results: Array,
    ) -> SetRespBuilder {
        let updated_obj_results = updated_obj_results.into_iter().map(Dynamic::cast).collect();
        builder.with_updated_obj_results(updated_obj_results)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn setresp_updated_instance_failure_builder(
        affected_path: &str,
    ) -> UpdatedInstanceFailureBuilder {
        UpdatedInstanceFailureBuilder::new(affected_path.into())
    }

    #[rhai_fn(global, return_raw)]
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `param_errs` cannot be converted into
    /// an array of type `(String, u32, String)`.
    pub fn with_param_errs(
        builder: UpdatedInstanceFailureBuilder,
        param_errs: Array,
    ) -> Result<UpdatedInstanceFailureBuilder, Box<EvalAltResult>> {
        let param_errs = param_errs
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, err_code: u32, err_msg: &str]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<i64>()
                    .ok_or("err_code (#2) needs to be a u32".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("err_msg (#3) needs to be a string".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok(SetRespParameterError::new(el0, u32::try_from(el1).unwrap_or(7003), Some(el2))),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<SetRespParameterError>, Box<EvalAltResult>>>()?;
        Ok(builder.with_param_errs(param_errs))
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn setresp_updated_obj_failure(
        requested_path: &str,
        err_code: i64,
        err_msg: &str,
        updated_inst_failures: Array,
    ) -> UpdatedObjectResultsBuilder {
        let oper_status = SetOperationStatus::new().set_failure(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
            updated_inst_failures
                .into_iter()
                .map(Dynamic::cast)
                .collect(),
        );
        UpdatedObjectResultsBuilder::new(requested_path.into(), oper_status)
    }

    /// # Errors
    ///
    /// This function will return `Err` if the provided `param_errs` cannot be converted into
    /// an array of type `(String, u32, String)`.
    #[rhai_fn(global, return_raw)]
    pub fn setresp_updated_obj_success(
        requested_path: &str,
        affected_path: &str,
        param_errs: Array,
        updated_params: Map,
    ) -> Result<UpdatedObjectResultsBuilder, Box<EvalAltResult>> {
        let param_errs = param_errs
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, err_code: u32, err_msg: &str]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<i64>()
                    .ok_or("err_code (#2) needs to be a u32".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("err_msg (#3) needs to be a string".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok(SetRespParameterError::new(el0, u32::try_from(el1).unwrap_or(7003)
, Some(el2))),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<SetRespParameterError>, Box<EvalAltResult>>>()?;

        let updated_params = updated_params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        Ok(UpdatedObjectResultsBuilder::new(
            requested_path.into(),
            SetOperationStatus::new().set_success(vec![SetOperationSuccessBuilder::new(
                affected_path.into(),
            )
            .with_updated_params(updated_params)
            .with_param_errs(param_errs)]),
        ))
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: SetRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Error Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::error_builder()
///   .set_err(7002, "I don't know")
///   .with_param_errs([["Foo", 7002, ""]])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_error {
    use usp_builder::ErrorBuilder;

    /// Sets up a new USP `ErrorBuilder`
    #[must_use]
    pub fn error_builder() -> ErrorBuilder {
        ErrorBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_err(builder: ErrorBuilder, err_code: i64, err_msg: &str) -> ErrorBuilder {
        builder.set_err(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    /// # Errors
    ///
    /// This function will return `Err` if the provided `param_errs` cannot be converted into
    /// an array of type `(String, u32, String)`.
    #[rhai_fn(global, return_raw)]
    pub fn with_param_errs(
        builder: ErrorBuilder,
        param_errs: Array,
    ) -> Result<ErrorBuilder, Box<EvalAltResult>> {
        let param_errs = param_errs
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, err_code: u32, err_msg: &str]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<i64>()
                    .ok_or("err_code (#2) needs to be a u32".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("err_msg (#3) needs to be a string".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok((el0, u32::try_from(el1).unwrap_or(7003)
, el2)),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<_>, Box<EvalAltResult>>>()?;

        Ok(builder.with_param_errs(param_errs))
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: ErrorBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Get Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::get_builder()
///   .with_params(["Device."])
///   .with_max_depth(1)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_get {
    use usp_builder::GetBuilder;

    /// Sets up a new USP `GetBuilder`
    #[must_use]
    pub fn get_builder() -> GetBuilder {
        GetBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_max_depth(builder: GetBuilder, max_depth: i64) -> GetBuilder {
        builder.with_max_depth(u32::try_from(max_depth).unwrap_or(0))
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_params(builder: GetBuilder, params: Array) -> GetBuilder {
        builder.with_params(params.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp GetResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let respathres = [];
/// respathres += rusp::get_res_path_result_builder("Device.")
///   .with_result_params(#{"Foo": "Bar"});
/// let reqpathres = [];
/// reqpathres += rusp::req_path_result_builder("Device.")
///   .set_err(7002, "Look, a fancy error")
///   .with_res_path_result(respathres);
/// let body = rusp::getresp_builder()
///   .with_req_path_results(reqpathres)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getresp {
    use usp_builder::{GetReqPathResultBuilder, GetRespBuilder, ResolvedPathResultBuilder};

    /// Sets up a new USP `GetRespBuilder`
    #[must_use]
    pub fn getresp_builder() -> GetRespBuilder {
        GetRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_req_path_results(
        builder: GetRespBuilder,
        req_path_results: Array,
    ) -> GetRespBuilder {
        builder.with_req_path_results(req_path_results.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }

    #[must_use]
    pub fn req_path_result_builder(requested_path: &str) -> GetReqPathResultBuilder {
        GetReqPathResultBuilder::new(requested_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_err(
        builder: GetReqPathResultBuilder,
        err_code: i64,
        err_msg: &str,
    ) -> GetReqPathResultBuilder {
        builder.set_err(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_res_path_result(
        builder: GetReqPathResultBuilder,
        resolved_path_results: Array,
    ) -> GetReqPathResultBuilder {
        builder.with_res_path_results(
            resolved_path_results
                .into_iter()
                .map(Dynamic::cast)
                .collect(),
        )
    }

    #[must_use]
    pub fn get_res_path_result_builder(resolved_path: &str) -> ResolvedPathResultBuilder {
        ResolvedPathResultBuilder::new(resolved_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_result_params(
        builder: ResolvedPathResultBuilder,
        result_params: Map,
    ) -> ResolvedPathResultBuilder {
        let result_params = result_params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        builder.with_result_params(result_params)
    }
}

/// Supply Rusp GetSupportedDM Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::getsupporteddm_builder()
///   .with_obj_paths(["Device.", "Device.DeviceInfo."])
///   .with_first_level_only(true)
///   .with_return_commands(false)
///   .with_return_events(false)
///   .with_return_params(false)
///   .with_return_unique_key_sets(true)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getsupporteddm {
    use usp_builder::GetSupportedDMBuilder;

    /// Sets up a new USP `GetSupportedDMBuilder`
    #[must_use]
    pub fn getsupporteddm_builder() -> GetSupportedDMBuilder {
        GetSupportedDMBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_first_level_only(
        builder: GetSupportedDMBuilder,
        first_level_only: bool,
    ) -> GetSupportedDMBuilder {
        builder.with_first_level_only(first_level_only)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_obj_paths(
        builder: GetSupportedDMBuilder,
        obj_paths: Array,
    ) -> GetSupportedDMBuilder {
        builder.with_obj_paths(obj_paths.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_return_commands(
        builder: GetSupportedDMBuilder,
        return_commands: bool,
    ) -> GetSupportedDMBuilder {
        builder.with_return_commands(return_commands)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_return_events(
        builder: GetSupportedDMBuilder,
        return_events: bool,
    ) -> GetSupportedDMBuilder {
        builder.with_return_events(return_events)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_return_params(
        builder: GetSupportedDMBuilder,
        return_params: bool,
    ) -> GetSupportedDMBuilder {
        builder.with_return_params(return_params)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_return_unique_key_sets(
        builder: GetSupportedDMBuilder,
        return_unique_key_sets: bool,
    ) -> GetSupportedDMBuilder {
        builder.with_return_unique_key_sets(return_unique_key_sets)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetSupportedDMBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp GetSupportedDMResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let command = rusp::getsupporteddmresp_command_result_builder("Foo()")
///   .with_input_arg_names(["Foo", "Bar", "Baz"])
///   .with_output_arg_names(["Bam"])
///   .set_sync();
/// let event = rusp::getsupporteddmresp_event_result_builder("Foo!")
///   .with_arg_names(["Foo", "Bar", "Baz"]);
/// let param = rusp::getsupporteddmresp_param_result_builder("Foo")
///   .set_type_boolean()
///   .set_value_change_allowed()
///   .set_access_read_write();
/// let req_obj1 = rusp::getsupporteddm_req_obj_result_builder("Device.")
///   .set_err(7005, "");
/// let supported_obj = rusp::getsupporteddmresp_supported_obj_result_builder("Device.DeviceInfo.")
///   .with_is_multi_instance(false)
///   .set_access_add_delete()
///   .with_supported_commands([command])
///   .with_supported_events([event])
///   .with_supported_params([param])
///   .with_divergent_paths(["Device.DeviceInfo.Foo.1.", "Device.DeviceInfo.Foo.2."])
///   .with_unique_key_sets([["Foo", "Bar"], ["Baz"]]);
/// let req_obj2 = rusp::getsupporteddm_req_obj_result_builder("Device.DeviceInfo.")
///   .with_data_model_inst_uri("urn:broadband-forum-org:tr-181-2-17-0-usp")
///   .with_supported_objs([supported_obj]);
/// let body = rusp::getsupporteddmresp_builder()
///   .with_req_obj_results([req_obj1, req_obj2])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getsupporteddmresp {
    use usp_builder::{
        GSDMCommandResult, GSDMEventResult, GSDMParamResult, GSDMReqObjectResultBuilder,
        GSDMSupportedObjectResultBuilder, GetSupportedDMRespBuilder,
    };

    /// Sets up a new USP `GetSupportedDMRespBuilder`
    #[must_use]
    pub fn getsupporteddmresp_builder() -> GetSupportedDMRespBuilder {
        GetSupportedDMRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_req_obj_results(
        builder: GetSupportedDMRespBuilder,
        req_obj_results: Array,
    ) -> GetSupportedDMRespBuilder {
        builder.with_req_obj_results(req_obj_results.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetSupportedDMRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }

    #[must_use]
    pub fn getsupporteddm_req_obj_result_builder(req_obj_path: &str) -> GSDMReqObjectResultBuilder {
        GSDMReqObjectResultBuilder::new(req_obj_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_err(
        builder: GSDMReqObjectResultBuilder,
        err_code: i64,
        err_msg: &str,
    ) -> GSDMReqObjectResultBuilder {
        builder.set_err(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_data_model_inst_uri(
        builder: GSDMReqObjectResultBuilder,
        data_model_inst_uri: &str,
    ) -> GSDMReqObjectResultBuilder {
        builder.with_data_model_inst_uri(data_model_inst_uri.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_supported_objs(
        builder: GSDMReqObjectResultBuilder,
        supported_objs: Array,
    ) -> GSDMReqObjectResultBuilder {
        builder.with_supported_objs(supported_objs.into_iter().map(Dynamic::cast).collect())
    }

    #[must_use]
    pub fn getsupporteddmresp_supported_obj_result_builder(
        req_obj_path: &str,
    ) -> GSDMSupportedObjectResultBuilder {
        GSDMSupportedObjectResultBuilder::new(req_obj_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_is_multi_instance(
        builder: GSDMSupportedObjectResultBuilder,
        is_multi_instance: bool,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.with_is_multi_instance(is_multi_instance)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_access_add_only(
        builder: GSDMSupportedObjectResultBuilder,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.set_access_add_only()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_access_delete_only(
        builder: GSDMSupportedObjectResultBuilder,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.set_access_delete_only()
    }

    #[rhai_fn(global, name = "set_access_read_only")]
    #[must_use]
    pub fn obj_set_access_read_only(
        builder: GSDMSupportedObjectResultBuilder,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.set_access_read_only()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_access_add_delete(
        builder: GSDMSupportedObjectResultBuilder,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.set_access_add_delete()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_supported_commands(
        builder: GSDMSupportedObjectResultBuilder,
        supported_commands: Array,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.with_supported_commands(supported_commands.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_supported_events(
        builder: GSDMSupportedObjectResultBuilder,
        supported_events: Array,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.with_supported_events(supported_events.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_supported_params(
        builder: GSDMSupportedObjectResultBuilder,
        supported_params: Array,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.with_supported_params(supported_params.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_divergent_paths(
        builder: GSDMSupportedObjectResultBuilder,
        divergent_paths: Array,
    ) -> GSDMSupportedObjectResultBuilder {
        builder.with_divergent_paths(divergent_paths.into_iter().map(Dynamic::cast).collect())
    }

    /// # Errors
    ///
    /// This function will return `Err` if the provided `unique_key_sets` cannot be converted into
    /// an array of type `String`.
    #[rhai_fn(global, return_raw)]
    pub fn with_unique_key_sets(
        builder: GSDMSupportedObjectResultBuilder,
        unique_key_sets: Array,
    ) -> Result<GSDMSupportedObjectResultBuilder, Box<EvalAltResult>> {
        let unique_key_sets = unique_key_sets
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays of string".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                el.iter()
                    .map(|el| {
                        el.clone()
                            .try_cast::<String>()
                            .ok_or("param needs to be a string".to_string())
                    })
                    .collect::<Result<Vec<String>, _>>()
            })
            .collect::<Result<Vec<Vec<String>>, _>>()?;
        Ok(builder.with_unique_key_sets(unique_key_sets))
    }

    #[must_use]
    pub fn getsupporteddmresp_command_result_builder(command_name: &str) -> GSDMCommandResult {
        GSDMCommandResult::new(command_name.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_input_arg_names(
        builder: GSDMCommandResult,
        input_arg_names: Array,
    ) -> GSDMCommandResult {
        builder.with_input_arg_names(input_arg_names.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_output_arg_names(
        builder: GSDMCommandResult,
        output_arg_names: Array,
    ) -> GSDMCommandResult {
        builder.with_output_arg_names(output_arg_names.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_sync(builder: GSDMCommandResult) -> GSDMCommandResult {
        builder.set_sync()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_async(builder: GSDMCommandResult) -> GSDMCommandResult {
        builder.set_async()
    }

    #[must_use]
    pub fn getsupporteddmresp_event_result_builder(event_name: &str) -> GSDMEventResult {
        GSDMEventResult::new(event_name.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_arg_names(builder: GSDMEventResult, input_arg_names: Array) -> GSDMEventResult {
        builder.with_arg_names(input_arg_names.into_iter().map(Dynamic::cast).collect())
    }

    #[must_use]
    pub fn getsupporteddmresp_param_result_builder(param_name: &str) -> GSDMParamResult {
        GSDMParamResult::new(param_name.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_access_read_only(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_access_read_only()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_access_write_only(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_access_write_only()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_access_read_write(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_access_read_write()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_int(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_int()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_unsigned_int(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_unsigned_int()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_long(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_long()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_unsigned_long(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_unsigned_long()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_string(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_string()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_base64(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_base64()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_hexbinary(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_hexbinary()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_datetime(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_datetime()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_decimal(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_decimal()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_type_boolean(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_type_boolean()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_value_change_allowed(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_value_change_allowed()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_value_change_will_ignore(builder: GSDMParamResult) -> GSDMParamResult {
        builder.set_value_change_will_ignore()
    }
}

/// Supply Rusp GetInstances Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::getinstances_builder()
///   .with_first_level_only(true)
///   .with_obj_paths(["Device."])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getinstances {
    use usp_builder::GetInstancesBuilder;

    /// Sets up a new USP `GetInstancesBuilder`
    #[must_use]
    pub fn getinstances_builder() -> GetInstancesBuilder {
        GetInstancesBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_first_level_only(
        builder: GetInstancesBuilder,
        first_level_only: bool,
    ) -> GetInstancesBuilder {
        builder.with_first_level_only(first_level_only)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_obj_paths(builder: GetInstancesBuilder, obj_paths: Array) -> GetInstancesBuilder {
        builder.with_obj_paths(obj_paths.into_iter().map(Dynamic::cast).collect())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetInstancesBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp GetInstancesResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let curr_inst = rusp::curr_instance_builder("Device.")
///   .with_unique_keys(#{"Foo": "Bar"});
/// let reqpathres = rusp::getinstances_req_path_result_builder("Device.")
///   .set_err(7002, "Look, a fancy error")
///   .with_curr_insts([curr_inst]);
/// let body = rusp::getinstancesresp_builder()
///   .with_req_path_results([reqpathres])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getinstancesresp {
    use usp_builder::{
        CurrInstanceBuilder, GetInstancesRespBuilder, GetInstancesRespReqPathResultBuilder,
    };

    /// Sets up a new USP `GetInstancesRespBuilder`
    #[must_use]
    pub fn getinstancesresp_builder() -> GetInstancesRespBuilder {
        GetInstancesRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_req_path_results(
        builder: GetInstancesRespBuilder,
        req_path_results: Array,
    ) -> GetInstancesRespBuilder {
        builder.with_req_path_results(req_path_results.into_iter().map(Dynamic::cast).collect())
    }

    #[must_use]
    pub fn getinstances_req_path_result_builder(
        requested_path: &str,
    ) -> GetInstancesRespReqPathResultBuilder {
        GetInstancesRespReqPathResultBuilder::new(requested_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn set_err(
        builder: GetInstancesRespReqPathResultBuilder,
        err_code: i64,
        err_msg: &str,
    ) -> GetInstancesRespReqPathResultBuilder {
        builder.set_err(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_curr_insts(
        builder: GetInstancesRespReqPathResultBuilder,
        curr_insts: Array,
    ) -> GetInstancesRespReqPathResultBuilder {
        builder.with_curr_insts(curr_insts.into_iter().map(Dynamic::cast).collect())
    }

    #[must_use]
    pub fn curr_instance_builder(resolved_path: &str) -> CurrInstanceBuilder {
        CurrInstanceBuilder::new(resolved_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_unique_keys(builder: CurrInstanceBuilder, unique_keys: Map) -> CurrInstanceBuilder {
        let unique_keys = unique_keys
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        builder.with_unique_keys(unique_keys)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetInstancesRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp GetSupportedProtocol Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::getsupportedprotocol_builder("1.3,1.4")
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getsupportedprotocol {
    use usp_builder::GetSupportedProtocolBuilder;

    /// Sets up a new USP `GetSupportedProtocolBuilder`
    #[must_use]
    pub fn getsupportedprotocol_builder(
        controller_supported_protocol_versions: &str,
    ) -> GetSupportedProtocolBuilder {
        GetSupportedProtocolBuilder::new(controller_supported_protocol_versions.into())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetSupportedProtocolBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp GetSupportedProtocolResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let body = rusp::getsupportedprotocolresp_builder("1.2,1.3")
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_getsupportedprotocolresp {
    use usp_builder::GetSupportedProtocolRespBuilder;

    /// Sets up a new USP `GetSupportedProtocolRespBuilder`
    #[must_use]
    pub fn getsupportedprotocolresp_builder(
        agent_supported_protocol_versions: &str,
    ) -> GetSupportedProtocolRespBuilder {
        GetSupportedProtocolRespBuilder::new(agent_supported_protocol_versions.into())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: GetSupportedProtocolRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Add Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let create_obj = rusp::add_create_object_builder("Device.IP.Interface.")
///   .with_param_settings([["Foo", "Bar", true],["Whee", "What?", false]]);
/// let body = rusp::add_builder()
///   .with_allow_partial(true)
///   .with_create_objs([create_obj])
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_add {
    use usp_builder::{AddBuilder, CreateObjectBuilder};

    /// Sets up a new USP `AddBuilder`
    #[must_use]
    pub fn add_builder() -> AddBuilder {
        AddBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_allow_partial(builder: AddBuilder, allow_partial: bool) -> AddBuilder {
        builder.with_allow_partial(allow_partial)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_create_objs(builder: AddBuilder, create_objs: Array) -> AddBuilder {
        builder.with_create_objs(create_objs.into_iter().map(Dynamic::cast).collect())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn add_create_object_builder(obj_path: &str) -> CreateObjectBuilder {
        CreateObjectBuilder::new(obj_path.into())
    }

    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `param_settings` cannot be converted into
    /// an array of type `(String, String, bool)`.
    #[rhai_fn(global, return_raw)]
    pub fn with_param_settings(
        mut builder: CreateObjectBuilder,
        param_settings: Array,
    ) -> Result<CreateObjectBuilder, Box<EvalAltResult>> {
        let param_settings = param_settings
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, value: &str, required: bool]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("value (#2) needs to be a string".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<bool>()
                    .ok_or("required (#3) needs to be a bool".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok((el0, el1, el2)),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<(String, String, bool)>, Box<EvalAltResult>>>()?;
        builder = builder.with_param_settings(param_settings);

        Ok(builder)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: AddBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp AddResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let created_objs = [
///   rusp::addresp_created_obj_failure("Foo", 7004, ""),
///   rusp::addresp_created_obj_success("Foo", "Foo.Bar.1", [["Bar", 7004, ""]], #{"Foo": "Bar"})
/// ];
/// let body = rusp::addresp_builder()
///   .with_created_obj_results(created_objs)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_addresp {
    use usp_builder::{
        AddOperationStatus, AddRespBuilder, AddRespParameterError, CreatedObjectResultsBuilder,
    };

    /// Sets up a new USP `AddRespBuilder`
    #[must_use]
    pub fn addresp_builder() -> AddRespBuilder {
        AddRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_created_obj_results(
        builder: AddRespBuilder,
        created_obj_results: Array,
    ) -> AddRespBuilder {
        let created_obj_results = created_obj_results.into_iter().map(Dynamic::cast).collect();
        builder.with_created_obj_results(created_obj_results)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn addresp_created_obj_failure(
        requested_path: &str,
        err_code: i64,
        err_msg: &str,
    ) -> CreatedObjectResultsBuilder {
        let oper_status = AddOperationStatus::new().set_failure(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        );
        CreatedObjectResultsBuilder::new(requested_path.into(), oper_status)
    }

    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `param_errs` cannot be converted into
    /// an array of type `(String, u32, String)`.
    #[rhai_fn(global, return_raw)]
    pub fn addresp_created_obj_success(
        requested_path: &str,
        instantiated_path: &str,
        param_errs: Array,
        unique_keys: Map,
    ) -> Result<CreatedObjectResultsBuilder, Box<EvalAltResult>> {
        let param_errs = param_errs
            .into_iter()
            .map(|p| {
                p.try_cast::<Array>()
                    .ok_or("Expected to have an array of arrays [param: &str, err_code: u32, err_msg: &str]".into())
            })
            .collect::<Result<Vec<Array>, Box<EvalAltResult>>>()?
            .iter()
            .map(|el| {
                let el0 = el[0]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("param (#1) needs to be a string".to_string());
                let el1 = el[1]
                    .clone()
                    .try_cast::<i64>()
                    .ok_or("err_code (#2) needs to be a u32".to_string());
                let el2 = el[2]
                    .clone()
                    .try_cast::<String>()
                    .ok_or("err_msg (#3) needs to be a string".to_string());

                match (el0, el1, el2) {
                    (Ok(el0), Ok(el1), Ok(el2)) => Ok(AddRespParameterError{param: el0, err_code: u32::try_from(el1).unwrap_or(7003) , err_msg: el2 }),
                    (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err.into()),
                }
            })
            .collect::<Result<Vec<AddRespParameterError>, Box<EvalAltResult>>>()?;

        let unique_keys = unique_keys
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        Ok(CreatedObjectResultsBuilder::new(
            requested_path.into(),
            AddOperationStatus::new().set_success(
                instantiated_path.into(),
                param_errs,
                unique_keys,
            ),
        ))
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: AddRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Notify Message functionality
///
/// This module facilities the assembly of [USP Notify
/// Message](https://usp.technology/specification/index.htm#sec:notify) [`bodies`](Body).
#[export_module]
pub mod rhai_rusp_notify {
    use usp_builder::NotifyBuilder;

    /// Sets up a new USP `NotifyBuilder`
    #[must_use]
    pub fn notify_builder(subscription_id: &str) -> NotifyBuilder {
        usp_builder::NotifyBuilder::new(subscription_id.into())
    }

    /// Set a notification body of type `ValueChange`
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_event("Device.", "Boot!", #{"CommandKey": "ck"})
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_event(
        builder: NotifyBuilder,
        obj_path: &str,
        event_name: &str,
        params: Map,
    ) -> NotifyBuilder {
        let params = params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        builder.with_event(obj_path.into(), event_name.into(), params)
    }

    /// Set a notification body of type `ValueChange`
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_value_change("Device.Foo", "bar")
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_value_change(
        builder: NotifyBuilder,
        param_path: &str,
        param_value: &str,
    ) -> NotifyBuilder {
        builder.with_value_change(param_path.into(), param_value.into())
    }

    /// Set a notification body of type `ObjectCreation`
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_send_resp(true)
    ///   .with_object_creation("Device.Foo.1.", #{"Alias": "cpe-01"})
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_object_creation(
        builder: NotifyBuilder,
        obj_path: &str,
        unique_keys: Map,
    ) -> NotifyBuilder {
        let unique_keys = unique_keys
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        builder.with_object_creation(obj_path.into(), unique_keys)
    }

    /// Set a notification body of type `ObjectDeletion`
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_send_resp(true)
    ///   .with_object_deletion("Device.Foo.1.")
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_object_deletion(builder: NotifyBuilder, obj_path: &str) -> NotifyBuilder {
        builder.with_object_deletion(obj_path.into())
    }

    /// Set a notification body of type `OperationComplete` with success
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_send_resp(true)
    ///   .with_operation_complete_output_args("Device.", "Reboot()", "Foo", #{"Status": "Complete", "Results": ""})
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_operation_complete_output_args(
        builder: NotifyBuilder,
        obj_path: &str,
        command_name: &str,
        command_key: &str,
        output_args: Map,
    ) -> NotifyBuilder {
        let output_args = output_args
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        builder.with_operation_complete_output_args(
            obj_path.into(),
            command_name.into(),
            command_key.into(),
            output_args,
        )
    }

    /// Set a notification body of type `OperationComplete` with failure
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_send_resp(true)
    ///   .with_operation_complete_cmd_failure("Device.", "Reboot()", "Foo", 7002, "Don't want to")
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_operation_complete_cmd_failure(
        builder: NotifyBuilder,
        obj_path: &str,
        command_name: &str,
        command_key: &str,
        err_code: i64,
        err_msg: &str,
    ) -> NotifyBuilder {
        builder.with_operation_complete_cmd_failure(
            obj_path.into(),
            command_name.into(),
            command_key.into(),
            u32::try_from(err_code).unwrap_or(7003),
            err_msg.into(),
        )
    }

    /// Set a notification body of type `OnBoardRequest`
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_send_resp(true)
    ///   .with_onboard_request("00CAFE", "None", "000111", "1.3")
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_onboard_request(
        builder: NotifyBuilder,
        oui: &str,
        product_class: &str,
        serial_number: &str,
        aspv: &str,
    ) -> NotifyBuilder {
        builder.with_onboard_request(
            oui.into(),
            product_class.into(),
            serial_number.into(),
            aspv.into(),
        )
    }

    /// Sets the `send_resp` flag in the notification
    ///
    /// Setting this to `true` causes the recipient to acknowledge the received notification.
    /// Conversely, setting this to `false` also disables the retry mechanism so it should usually
    /// be set to `true`.
    ///
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::notify_builder("not-sub")
    ///   .with_send_resp(true)
    ///   .with_operation_complete_cmd_failure("Device.", "Reboot()", "Foo", 7002, "Don't want to")
    ///   .build();
    /// rusp::msg_builder()
    ///   .with_msg_id("Foo")
    ///   .with_body(body)
    ///   .build()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
    /// # assert!(!msg.is_error());
    /// # assert!(msg.is_request());
    /// # assert!(!msg.is_response());
    /// ```
    #[rhai_fn(global)]
    #[must_use]
    pub fn with_send_resp(builder: NotifyBuilder, send_resp: bool) -> NotifyBuilder {
        builder.with_send_resp(send_resp)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: NotifyBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp NotifyResp Message functionality
/// ```
/// let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(r#"
///     let body = rusp::notifyresp_builder("sub")
///         .build();
///     rusp::msg_builder()
///         .with_msg_id("Foo")
///         .with_body(body)
///         .build()
/// "#).unwrap();
/// assert!(!msg.is_error());
/// assert!(!msg.is_request());
/// assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_notifyresp {
    use usp_builder::NotifyRespBuilder;

    /// Sets up a new USP `NotifyRespBuilder`
    #[must_use]
    pub fn notifyresp_builder(subscription_id: &str) -> NotifyRespBuilder {
        usp_builder::NotifyRespBuilder::new(subscription_id.into())
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: NotifyRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp Operate Message functionality
/// ```
/// # let script = r#"
/// // Rhai script
/// let body = rusp::operate_builder("Device.Reboot()")
///   .with_command_key("Command")
///   .with_send_resp(true)
///   .with_input_args(#{"Foo": "Bar", "Baz": "Bam"})
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(msg.is_request());
/// # assert!(!msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_operate {
    use usp_builder::OperateBuilder;

    /// Sets up a new USP `OperateBuilder`
    #[must_use]
    pub fn operate_builder(command: &str) -> OperateBuilder {
        OperateBuilder::new(command.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_command_key(builder: OperateBuilder, command_key: &str) -> OperateBuilder {
        builder.with_command_key(command_key.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_send_resp(builder: OperateBuilder, send_resp: bool) -> OperateBuilder {
        builder.with_send_resp(send_resp)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_input_args(builder: OperateBuilder, input_args: Map) -> OperateBuilder {
        let input_args = input_args
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        builder.with_input_args(input_args)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: OperateBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp OperateResp Message functionality
/// ```
/// // Rhai script
/// # let script = r#"
/// let oper_results = [
///   rusp::operateresp_result_failure("Device.SelfTestDiagnostics()", 7004, ""),
///   rusp::operateresp_result_path("Device.Foo()", "Device.LocalAgent.Request.25."),
///   rusp::operateresp_result_output_args("Device.Bar()", #{"Baz": "Foo"}),
/// ];
/// let body = rusp::operateresp_builder()
///   .with_operation_results(oper_results)
///   .build();
/// rusp::msg_builder()
///   .with_msg_id("Foo")
///   .with_body(body)
///   .build()
/// # "#;
/// # let msg = rhai_rusp::eval_rusp::<rusp_lib::usp::Msg>(script).unwrap();
/// # assert!(!msg.is_error());
/// # assert!(!msg.is_request());
/// # assert!(msg.is_response());
/// ```
#[export_module]
pub mod rhai_rusp_operateresp {
    use usp_builder::{OperateRespBuilder, OperateRespResultBuilder};

    /// Sets up a new USP `OperateRespBuilder`
    #[must_use]
    pub fn operateresp_builder() -> OperateRespBuilder {
        OperateRespBuilder::new()
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn with_operation_results(
        builder: OperateRespBuilder,
        operation_results: Array,
    ) -> OperateRespBuilder {
        let operation_results = operation_results.into_iter().map(Dynamic::cast).collect();
        builder.with_operation_results(operation_results)
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn operateresp_result_failure(
        executed_command: &str,
        err_code: i64,
        err_msg: &str,
    ) -> OperateRespResultBuilder {
        OperateRespResultBuilder::new(executed_command.into()).set_failure(
            u32::try_from(err_code).unwrap_or(7003),
            (!err_msg.is_empty()).then_some(err_msg.into()),
        )
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn operateresp_result_path(
        executed_command: &str,
        req_obj_path: &str,
    ) -> OperateRespResultBuilder {
        OperateRespResultBuilder::new(executed_command.into()).set_path(req_obj_path.into())
    }

    #[rhai_fn(global)]
    #[must_use]
    pub fn operateresp_result_output_args(
        executed_command: &str,
        output_args: Map,
    ) -> OperateRespResultBuilder {
        let output_args = output_args
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        OperateRespResultBuilder::new(executed_command.into()).set_output_args(output_args)
    }

    /// Turns the builder into a USP [`Body`] structure
    ///
    /// # Errors
    ///
    /// This function will return `Err` if the provided `builder` was set up with incomplete or
    /// incorrect data.
    #[rhai_fn(global, return_raw)]
    pub fn build(builder: OperateRespBuilder) -> Result<Body, Box<EvalAltResult>> {
        Ok(builder.build().map_err(|e| e.to_string())?)
    }
}

/// Supply Rusp de-/serialization functionality
/// ```
/// // Rhai script
/// let script = r#"
///     rusp::record_builder()
///         .with_version("1.3")
///         .with_to_id("proto::to")
///         .with_from_id("proto::from")
///         .as_websocket_connect_record()
///         .build()
///         .to_string()
/// "#;
/// let record = rhai_rusp::eval_rusp::<String>(script).unwrap();
///
/// assert_eq!(record, "{\n  \"version\": \"1.3\",\n  \"to_id\": \"proto::to\",\n  \"from_id\": \"proto::from\",\n  \"payload_security\": \"PLAINTEXT\",\n  \"mac_signature\": [],\n  \"sender_cert\": [],\n  \"websocket_connect\": null\n}");
/// ```
#[export_module]
pub mod rhai_rusp {
    use std::io::{BufReader, Read, Write as _};

    use rusp_lib::usp_decoder::{try_decode_msg, try_decode_record};
    use usp_record::mod_Record::OneOfrecord_type;

    /// Render a USP Body into JSON format, this function is polymorphic in Rhai and available as `to_string()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::get_builder().with_max_depth(1).with_params(["Device."]).build();
    /// body.to_string()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<String>(script).unwrap();
    /// # assert_eq!(msg, "{\n  \"Request\": {\n    \"Get\": {\n      \"param_paths\": [\n        \"Device.\"\n      ],\n      \"max_depth\": 1\n    }\n  }\n}");
    /// ```
    ///
    /// This example will return a JSON output like:
    /// ```text
    /// {
    ///   "Request": {
    ///     "Get": {
    ///       "param_paths": [
    ///         "Device."
    ///       ],
    ///       "max_depth": 1
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// `to_string()` is also implicitly called by the `print()` function
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into JSON format fails.
    #[rhai_fn(global, name = "to_string", return_raw)]
    pub fn body_to_string(body: &mut Body) -> Result<String, Box<EvalAltResult>> {
        body_to_json(body)
    }

    /// Render a [`Msg`] into JSON format, this function is polymorphic in Rhai and available as `to_string()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::getsupportedprotocol_builder("1.3,1.4")
    ///     .build();
    /// rusp::msg_builder()
    ///     .with_msg_id("Foo")
    ///     .with_body(body)
    ///     .build()
    ///     .to_string()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<String>(script).unwrap();
    /// # assert_eq!(msg, "{\n  \"Header\": {\n    \"msg_id\": \"Foo\",\n    \"msg_type\": \"GET_SUPPORTED_PROTO\"\n  },\n  \"Body\": {\n    \"Request\": {\n      \"GetSupportedProtocol\": {\n        \"controller_supported_protocol_versions\": \"1.3,1.4\"\n      }\n    }\n  }\n}");
    /// ```
    ///
    /// This example will return a JSON output like:
    /// ```text
    /// {
    ///   "Header": {
    ///     "msg_id": "Foo",
    ///     "msg_type": "GET_SUPPORTED_PROTO"
    ///   },
    ///   "Body": {
    ///     "Request": {
    ///       "GetSupportedProtocol": {
    ///         "controller_supported_protocol_versions": "1.3,1.4"
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// `to_string()` is also implicitly called by the `print()` function
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into JSON format fails.
    #[rhai_fn(global, name = "to_string", return_raw)]
    pub fn msg_to_string(msg: &mut Msg) -> Result<String, Box<EvalAltResult>> {
        msg_to_json(msg)
    }

    /// Render a USP [`Record`] into JSON format, this function is polymorphic in Rhai and available as `to_string()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_version("1.3")
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_websocket_connect_record()
    ///   .build()
    ///   .to_string()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<String>(script).unwrap();
    /// # assert_eq!(record, "{\n  \"version\": \"1.3\",\n  \"to_id\": \"proto::to\",\n  \"from_id\": \"proto::from\",\n  \"payload_security\": \"PLAINTEXT\",\n  \"mac_signature\": [],\n  \"sender_cert\": [],\n  \"websocket_connect\": null\n}");
    /// ```
    ///
    /// This example will return a JSON output like:
    /// ```text
    /// {
    ///   "version": "1.3",
    ///   "to_id": "proto::to",
    ///   "from_id": "proto::from",
    ///   "payload_security": "PLAINTEXT",
    ///   "mac_signature": [],
    ///   "sender_cert": [],
    ///   "websocket_connect": null
    /// }
    /// ```
    ///
    /// `to_string()` is also implicitly called by the `print()` function
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into JSON format fails.
    #[rhai_fn(global, name = "to_string", return_raw)]
    pub fn record_to_string(record: &mut Record) -> Result<String, Box<EvalAltResult>> {
        record_to_json(record)
    }

    /// Render a USP Body into JSON format, this function is polymorphic in Rhai and available as `to_json()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::get_builder().with_max_depth(1).with_params(["Device."]).build();
    /// body.to_json()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<String>(script).unwrap();
    /// # assert_eq!(msg, "{\n  \"Request\": {\n    \"Get\": {\n      \"param_paths\": [\n        \"Device.\"\n      ],\n      \"max_depth\": 1\n    }\n  }\n}");
    /// ```
    ///
    /// This example will return a JSON output like:
    /// ```text
    /// {
    ///   "Request": {
    ///     "Get": {
    ///       "param_paths": [
    ///         "Device."
    ///       ],
    ///       "max_depth": 1
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into JSON format fails.
    #[rhai_fn(global, name = "to_json", return_raw)]
    pub fn body_to_json(body: &mut Body) -> Result<String, Box<EvalAltResult>> {
        Ok(serde_json::to_string_pretty(&body).map_err(|e| e.to_string())?)
    }

    /// Render a [`Msg`] into JSON format, this function is polymorphic in Rhai and available as `to_json()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::getsupportedprotocol_builder("1.3,1.4")
    ///     .build();
    /// rusp::msg_builder()
    ///     .with_msg_id("Foo")
    ///     .with_body(body)
    ///     .build()
    ///     .to_json()
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<String>(script).unwrap();
    /// # assert_eq!(msg, "{\n  \"Header\": {\n    \"msg_id\": \"Foo\",\n    \"msg_type\": \"GET_SUPPORTED_PROTO\"\n  },\n  \"Body\": {\n    \"Request\": {\n      \"GetSupportedProtocol\": {\n        \"controller_supported_protocol_versions\": \"1.3,1.4\"\n      }\n    }\n  }\n}");
    /// ```
    ///
    /// This example will return a JSON output like:
    /// ```text
    /// {
    ///   "Header": {
    ///     "msg_id": "Foo",
    ///     "msg_type": "GET_SUPPORTED_PROTO"
    ///   },
    ///   "Body": {
    ///     "Request": {
    ///       "GetSupportedProtocol": {
    ///         "controller_supported_protocol_versions": "1.3,1.4"
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into JSON format fails.
    #[rhai_fn(global, name = "to_json", return_raw)]
    pub fn msg_to_json(msg: &mut Msg) -> Result<String, Box<EvalAltResult>> {
        Ok(serde_json::to_string_pretty(&msg).map_err(|e| e.to_string())?)
    }

    /// Render a [`Msg`] into JSON format, this function is polymorphic in Rhai and available as `to_json()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_version("1.3")
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_websocket_connect_record()
    ///   .build()
    ///   .to_json()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<String>(script).unwrap();
    /// # assert_eq!(record, "{\n  \"version\": \"1.3\",\n  \"to_id\": \"proto::to\",\n  \"from_id\": \"proto::from\",\n  \"payload_security\": \"PLAINTEXT\",\n  \"mac_signature\": [],\n  \"sender_cert\": [],\n  \"websocket_connect\": null\n}");
    /// ```
    ///
    /// This example will return a JSON output like:
    /// ```text
    /// {
    ///   "version": "1.3",
    ///   "to_id": "proto::to",
    ///   "from_id": "proto::from",
    ///   "payload_security": "PLAINTEXT",
    ///   "mac_signature": [],
    ///   "sender_cert": [],
    ///   "websocket_connect": null
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into JSON format fails.
    #[rhai_fn(global, name = "to_json", return_raw)]
    pub fn record_to_json(record: &mut Record) -> Result<String, Box<EvalAltResult>> {
        Ok(serde_json::to_string_pretty(&record).map_err(|e| e.to_string())?)
    }

    /// Render a USP Body into a Rhai Map, this function is polymorphic in Rhai and available as `to_map()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::get_builder().with_max_depth(1).with_params(["Device."]).build().to_map();
    /// body.Request
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rhai::Map>(script).unwrap();
    /// ```
    ///
    /// This example will return a Rhai Map like:
    /// ```text
    /// #{"Get": #{"max_depth": 1, "param_paths": ["Device."]}}
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a Rhai map fails.
    #[rhai_fn(global, name = "to_map", return_raw)]
    pub fn body_to_map(body: &mut Body) -> Result<Dynamic, Box<EvalAltResult>> {
        rhai::serde::to_dynamic(body)
    }

    /// Render a [`Msg`] into a Rhai Map, this function is polymorphic in Rhai and available as `to_map()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let body = rusp::getsupportedprotocol_builder("1.3,1.4")
    ///     .build();
    /// let msg = rusp::msg_builder()
    ///     .with_msg_id("Foo")
    ///     .with_body(body)
    ///     .build()
    ///     .to_map();
    /// msg.Header
    /// # "#;
    /// # let msg = rhai_rusp::eval_rusp::<rhai::Map>(script).unwrap();
    /// ```
    ///
    /// This example will return a Rhai Map like:
    /// ```text
    /// #{"msg_id": "Foo", "msg_type": "GET_SUPPORTED_PROTO"}
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a Rhai map fails.
    #[rhai_fn(global, name = "to_map", return_raw)]
    pub fn msg_to_map(msg: &mut Msg) -> Result<Dynamic, Box<EvalAltResult>> {
        rhai::serde::to_dynamic(msg)
    }

    /// Render a USP [`Record`] into a Rhai Map, this function is polymorphic in Rhai and available as `to_map()`
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// rusp::record_builder()
    ///   .with_version("1.3")
    ///   .with_to_id("proto::to")
    ///   .with_from_id("proto::from")
    ///   .as_websocket_connect_record()
    ///   .build()
    ///   .to_map()
    /// # "#;
    /// # let record = rhai_rusp::eval_rusp::<rhai::Map>(script).unwrap();
    /// # assert_eq!(record.get("version").and_then(|v| v.clone().into_string().ok()), Some("1.3".into()));
    /// ```
    ///
    /// This example will return a Rhai map like:
    /// ```text
    /// #{"from_id": "proto::from", "mac_signature": [], "payload_security": "PLAINTEXT", "sender_cert": [], "to_id": "proto::to", "version": "1.3", "websocket_connect": ()}
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a Rhai map fails.
    #[rhai_fn(global, name = "to_map", return_raw)]
    pub fn record_to_map(record: &mut Record) -> Result<Dynamic, Box<EvalAltResult>> {
        rhai::serde::to_dynamic(record)
    }

    /// Render a [`Msg`] into C string format
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a C string fails.
    #[rhai_fn(global, name = "to_c_string", return_raw)]
    pub fn msg_to_c_string(msg: &mut Msg) -> Result<String, Box<EvalAltResult>> {
        Ok(msg.to_c_str().map_err(|e| e.to_string())?)
    }

    /// Render a [`Record`] into C string format
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a C string fails.
    #[rhai_fn(global, name = "to_c_string", return_raw)]
    pub fn record_to_c_string(record: &mut Record) -> Result<String, Box<EvalAltResult>> {
        Ok(record.to_c_str().map_err(|e| e.to_string())?)
    }

    /// Render a [`Msg`] into C array format
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a C array fails.
    #[rhai_fn(global, name = "to_c_array", return_raw)]
    pub fn msg_to_c_array(msg: &mut Msg) -> Result<String, Box<EvalAltResult>> {
        Ok(msg.to_c_array().map_err(|e| e.to_string())?)
    }

    /// Render a [`Record`] into C array format
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error if
    /// the serialization of the structure into a C array fails.
    #[rhai_fn(global, name = "to_c_array", return_raw)]
    pub fn record_to_c_array(record: &mut Record) -> Result<String, Box<EvalAltResult>> {
        Ok(record.to_c_array().map_err(|e| e.to_string())?)
    }

    /// Render a [`Msg`] into C array format and save it to the specified file
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the serialization of the structure into a C array or the creation of the specified file
    /// fails.
    #[rhai_fn(global, name = "save_c_array", return_raw)]
    pub fn msg_save_c_array(msg: &mut Msg, filename: &str) -> Result<(), Box<EvalAltResult>> {
        let data = msg.to_c_array().map_err(|e| e.to_string())?;
        std::fs::write(filename, data).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Render a [`Record`] into C array format and save it to the specified file
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the serialization of the structure into a C array or the creation of the specified file
    /// fails.
    #[rhai_fn(global, name = "save_c_array", return_raw)]
    pub fn record_save_c_array(
        record: &mut Record,
        filename: &str,
    ) -> Result<(), Box<EvalAltResult>> {
        let data = record.to_c_array().map_err(|e| e.to_string())?;
        std::fs::write(filename, data).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Render a [`Msg`] into Protobuf and print it to stdout
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the serialization of the structure into Protobuf format fails.
    #[rhai_fn(global, name = "print_protobuf", return_raw)]
    pub fn msg_print_protobuf(msg: &mut Msg) -> Result<(), Box<EvalAltResult>> {
        std::io::stdout()
            .write_all(&msg.to_vec().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Render a [`Record`] into Protobuf and print it to stdout
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the serialization of the structure into Protobuf format fails.
    #[rhai_fn(global, name = "print_protobuf", return_raw)]
    pub fn record_print_protobuf(record: &mut Record) -> Result<(), Box<EvalAltResult>> {
        std::io::stdout()
            .write_all(&record.to_vec().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Render a [`Msg`] into Protobuf and save it to the specified file
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the serialization of the structure into Protobuf format or the creation of the specified file
    /// fails.
    #[rhai_fn(global, name = "save_protobuf", return_raw)]
    pub fn msg_save_protobuf(msg: &mut Msg, filename: &str) -> Result<(), Box<EvalAltResult>> {
        let data = msg.to_vec().map_err(|e| e.to_string())?;
        std::fs::File::create(filename)
            .and_then(|mut f| f.write_all(&data))
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Render a [`Record`] into Protobuf and save it to the specified file
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the serialization of the structure into Protobuf format or the creation of the specified file
    /// fails.
    #[rhai_fn(global, name = "save_protobuf", return_raw)]
    pub fn record_save_protobuf(
        record: &mut Record,
        filename: &str,
    ) -> Result<(), Box<EvalAltResult>> {
        let data = record.to_vec().map_err(|e| e.to_string())?;
        std::fs::File::create(filename)
            .and_then(|mut f| f.write_all(&data))
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Load a [`Msg`] from a Protobuf file
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the file pointed to by the filename doesn't exist, is not readable or the
    /// deserialization of the structure from Protobuf format fails.
    #[rhai_fn(global, name = "load_msg", return_raw)]
    pub fn load_msg_protobuf(filename: &str) -> Result<Msg, Box<EvalAltResult>> {
        let contents = std::fs::File::open(filename)
            .map(BufReader::new)
            .map_err(|e| e.to_string())?
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(try_decode_msg(&contents).map_err(|e| e.to_string())?)
    }

    /// Load a [`Record`] from a Protobuf file, in Rhai this function is called `load_record`.
    /// ```
    /// // Rhai script
    /// # let script = r#"
    /// let record = rusp::load_record("test.pb");
    /// record.to_string()
    /// # "#;
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return `Err` containing a textual description of the encountered error
    /// if the file pointed to by the filename doesn't exist, is not readable or the
    /// deserialization of the structure from Protobuf format fails.
    #[rhai_fn(global, name = "load_record", return_raw)]
    pub fn load_record_protobuf(filename: &str) -> Result<Record, Box<EvalAltResult>> {
        let contents = std::fs::File::open(filename)
            .map(BufReader::new)
            .map_err(|e| e.to_string())?
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| e.to_string())?;
        let record = try_decode_record(&contents).map_err(|e| e.to_string())?;
        if record.record_type == OneOfrecord_type::None {
            Err("Protobuf file doesn't contain a valid USP Record")?
        }
        Ok(record)
    }
}

def_package! {
    pub RuspPackage(module) {
        combine_with_exported_module!(module, "rusp", rhai_rusp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_msg);
        combine_with_exported_module!(module, "rusp", rhai_rusp_record);
        combine_with_exported_module!(module, "rusp", rhai_rusp_add);
        combine_with_exported_module!(module, "rusp", rhai_rusp_addresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_delete);
        combine_with_exported_module!(module, "rusp", rhai_rusp_deleteresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_deregister);
        combine_with_exported_module!(module, "rusp", rhai_rusp_deregisterresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_error);
        combine_with_exported_module!(module, "rusp", rhai_rusp_get);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getinstances);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getinstancesresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getsupportedprotocol);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getsupportedprotocolresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getsupporteddm);
        combine_with_exported_module!(module, "rusp", rhai_rusp_getsupporteddmresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_notify);
        combine_with_exported_module!(module, "rusp", rhai_rusp_notifyresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_operate);
        combine_with_exported_module!(module, "rusp", rhai_rusp_operateresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_register);
        combine_with_exported_module!(module, "rusp", rhai_rusp_registerresp);
        combine_with_exported_module!(module, "rusp", rhai_rusp_set);
        combine_with_exported_module!(module, "rusp", rhai_rusp_setresp);
    }
}
