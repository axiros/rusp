use crate::usp::Msg;
use crate::usp_record::{Record, SessionContextRecord};

use anyhow::{Context, Result};

/// Encode the Record into a Protobuf byte stream returned as `Vec<[u8]>`
///
/// # Arguments
///
/// * `self` - A USP `Record` structure
///
/// # Example
///
/// ```
/// use rusp::usp_decoder::try_decode_record;
/// use rusp::usp_encoder::try_encode_record;
/// let bytes = &[
///         0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64,
///         0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09,
///         0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f,
///         0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74,
///         0x6f, 0x70, 0x69, 0x63,
///     ];
/// let record = try_decode_record(bytes).unwrap();
/// assert_eq!(try_encode_record(&record).unwrap(), bytes);
/// ```
pub fn try_encode_record(record: &Record) -> Result<Vec<u8>> {
    record.to_vec()
}

/// Encode the Msg into a Protobuf byte stream returned as `Vec<[u8]>`
///
/// # Arguments
///
/// * `self` - A decoded USP Msg structure
///
/// # Example
///
/// ```
/// use rusp::usp_decoder::try_decode_msg;
/// use rusp::usp_encoder::try_encode_msg;
/// let bytes = &[
///         0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74,
///         0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24,
///         0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10,
///         0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34,
///         0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f,
///         0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22,
///         0x03, 0x31, 0x2e, 0x33,
///     ];
/// let msg = try_decode_msg(bytes).unwrap();
/// assert_eq!(try_encode_msg(&msg).unwrap(), bytes);
/// ```
pub fn try_encode_msg(msg: &Msg) -> Result<Vec<u8>> {
    msg.to_vec()
}

impl SessionContextRecord {
    /// Creates a new [`SessionContextRecord`] with an unfragmented payload
    pub fn new_unfragmented(
        session_id: u64,
        sequence_id: u64,
        expected_id: u64,
        retransmit_id: u64,
        payload: Vec<u8>,
    ) -> Self {
        use crate::usp_record::mod_SessionContextRecord::PayloadSARState;

        Self {
            session_id,
            sequence_id,
            expected_id,
            retransmit_id,
            payload_sar_state: PayloadSARState::NONE,
            payloadrec_sar_state: PayloadSARState::NONE,
            payload: vec![payload],
        }
    }
}

/// Implementation of some extension methods for `Record`s
impl Record {
    /// Render the `Record` into JSON
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Record` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_record;
    /// let record =
    ///     try_decode_record(&[
    ///         0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64,
    ///         0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09,
    ///         0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f,
    ///         0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74,
    ///         0x6f, 0x70, 0x69, 0x63,
    ///     ]).unwrap();
    /// assert_eq!(record.to_json().unwrap(), "{\"version\":\"1.3\",\"to_id\":\"doc::to\",\"from_id\":\"doc::from\",\"payload_security\":\"PLAINTEXT\",\"mac_signature\":[],\"sender_cert\":[],\"mqtt_connect\":{\"version\":\"V5\",\"subscribed_topic\":\"topic\"}}");
    /// ```
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).context("Failed serializing USP Record to JSON")
    }

    /// Render the `Record` into pretty printed JSON
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Record` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_record;
    /// let record =
    ///     try_decode_record(&[
    ///         0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64,
    ///         0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09,
    ///         0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f,
    ///         0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74,
    ///         0x6f, 0x70, 0x69, 0x63,
    ///     ]).unwrap();
    /// assert_eq!(record.to_json_pretty().unwrap(), "{\n  \"version\": \"1.3\",\n  \"to_id\": \"doc::to\",\n  \"from_id\": \"doc::from\",\n  \"payload_security\": \"PLAINTEXT\",\n  \"mac_signature\": [],\n  \"sender_cert\": [],\n  \"mqtt_connect\": {\n    \"version\": \"V5\",\n    \"subscribed_topic\": \"topic\"\n  }\n}");
    /// ```
    pub fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed serializing USP Record to JSON")
    }

    /// Encode the Record into a Protobuf byte stream returned as `Vec<[u8]>`
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Record` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_record;
    /// let bytes = &[
    ///         0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64,
    ///         0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09,
    ///         0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f,
    ///         0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74,
    ///         0x6f, 0x70, 0x69, 0x63,
    ///     ];
    /// let record = try_decode_record(bytes).unwrap();
    /// assert_eq!(record.to_vec().unwrap(), bytes);
    /// ```
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        use quick_protobuf::{message::MessageWrite, Writer};

        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf);
        self.write_message(&mut writer)
            .context("Failed serializing USP Record to Protobuf")?;

        Ok(buf)
    }

    /// Render the `Record` into a raw C string representation
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Record` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_record;
    /// let record =
    ///     try_decode_record(&[
    ///         0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64,
    ///         0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09,
    ///         0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f,
    ///         0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74,
    ///         0x6f, 0x70, 0x69, 0x63,
    ///     ]).unwrap();
    /// assert_eq!(record.to_c_str().unwrap(), "\"\\x0a\\x031.3\\x12\\x07doc\\x3a\\x3ato\\x1a\\x09doc\\x3a\\x3afromR\\x09\\x08\\x01\\x12\\x05topic\"\n");
    /// ```
    pub fn to_c_str(&self) -> Result<String> {
        use std::fmt::Write as _;

        const fn check_printable(c: u8) -> bool {
            match c as char {
                ' ' | '.' | '!' | '(' | ')' | '\'' | ',' | '*' | '[' | ']' | '=' | '<' | '>'
                | '-' | '_' => true,
                _ if c.is_ascii_alphanumeric() => true,
                _ => false,
            }
        }

        let data = self.to_vec()?;
        let mut out = String::new();

        write!(out, "\"")?;
        for i in data.iter() {
            if check_printable(*i) {
                write!(out, "{}", char::from(*i))?;
            } else {
                write!(out, "\\x{i:02x}")?;
            }
        }
        writeln!(out, "\"")?;

        Ok(out)
    }

    /// Render the `Record` into a raw C array representation
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Record` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_record;
    /// let record =
    ///     try_decode_record(&[
    ///         0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64,
    ///         0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09,
    ///         0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f,
    ///         0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74,
    ///         0x6f, 0x70, 0x69, 0x63,
    ///     ]).unwrap();
    /// assert_eq!(record.to_c_array().unwrap(), "unsigned int pb_len = 36;\nconst char pb[] = {\n  0x0a, 0x03, 0x31, 0x2e, 0x33, 0x12, 0x07, 0x64, /* __1.3__d */\n  0x6f, 0x63, 0x3a, 0x3a, 0x74, 0x6f, 0x1a, 0x09, /* oc__to__ */\n  0x64, 0x6f, 0x63, 0x3a, 0x3a, 0x66, 0x72, 0x6f, /* doc__fro */\n  0x6d, 0x52, 0x09, 0x08, 0x01, 0x12, 0x05, 0x74, /* mR_____t */\n  0x6f, 0x70, 0x69, 0x63,                         /* opic */\n};\n");
    /// ```
    pub fn to_c_array(&self) -> Result<String> {
        use std::fmt::Write as _;

        const fn check_printable(c: u8) -> bool {
            match c as char {
                ' ' | '.' | '!' | '(' | ')' | '\'' | '"' | ',' | '*' | '[' | ']' | '=' | '<'
                | '>' | '-' | '_' => true,
                _ if c.is_ascii_alphanumeric() => true,
                _ => false,
            }
        }

        let data = self.to_vec()?;
        let mut out = String::new();

        const CHUNK_LEN: usize = 8;
        writeln!(out, "unsigned int pb_len = {};", data.len())?;
        writeln!(out, "const char pb[] = {{")?;
        for chunk in data.chunks(CHUNK_LEN) {
            write!(out, "  ")?;
            for i in chunk {
                write!(out, "0x{i:02x}, ")?;
            }

            for _ in chunk.len()..CHUNK_LEN {
                write!(out, "      ")?;
            }

            write!(out, "/* ")?;
            for i in chunk {
                if check_printable(*i) {
                    write!(out, "{}", char::from(*i))?;
                } else {
                    write!(out, "_")?;
                }
            }
            write!(out, " */")?;

            writeln!(out)?;
        }
        writeln!(out, "}};")?;

        Ok(out)
    }
}

/// Implementation of some extension methods for `Msg`s
impl Msg {
    /// Encode the Msg as JSON format
    ///
    /// # Arguments
    ///
    /// * `self` - A decoded USP Msg structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74,
    ///         0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24,
    ///         0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10,
    ///         0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34,
    ///         0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f,
    ///         0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22,
    ///         0x03, 0x31, 0x2e, 0x33,
    ///     ]).unwrap();
    /// assert_eq!(msg.to_json().unwrap(), "{\"Header\":{\"msg_id\":\"test\",\"msg_type\":\"NOTIFY\"},\"Body\":{\"Request\":{\"Notify\":{\"subscription_id\":\"notif\",\"send_resp\":true,\"on_board_req\":{\"oui\":\"0044FF\",\"product_class\":\"Foo\",\"serial_number\":\"01234\",\"agent_supported_protocol_versions\":\"1.3\"}}}}}");
    /// ```
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).context("Failed serializing USP Msg to JSON")
    }

    /// Encode the Msg in pretty printed JSON format
    ///
    /// # Arguments
    ///
    /// * `self` - A decoded USP Msg structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74,
    ///         0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24,
    ///         0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10,
    ///         0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34,
    ///         0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f,
    ///         0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22,
    ///         0x03, 0x31, 0x2e, 0x33,
    ///     ]).unwrap();
    /// assert_eq!(msg.to_json_pretty().unwrap(), "{\n  \"Header\": {\n    \"msg_id\": \"test\",\n    \"msg_type\": \"NOTIFY\"\n  },\n  \"Body\": {\n    \"Request\": {\n      \"Notify\": {\n        \"subscription_id\": \"notif\",\n        \"send_resp\": true,\n        \"on_board_req\": {\n          \"oui\": \"0044FF\",\n          \"product_class\": \"Foo\",\n          \"serial_number\": \"01234\",\n          \"agent_supported_protocol_versions\": \"1.3\"\n        }\n      }\n    }\n  }\n}");
    /// ```
    pub fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed serializing USP Msg to JSON")
    }

    /// Encode the Msg into a Protobuf byte stream returned as `Vec<[u8]>`
    ///
    /// # Arguments
    ///
    /// * `self` - A decoded USP Msg structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let bytes = &[
    ///         0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74,
    ///         0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24,
    ///         0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10,
    ///         0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34,
    ///         0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f,
    ///         0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22,
    ///         0x03, 0x31, 0x2e, 0x33,
    ///     ];
    /// let msg = try_decode_msg(bytes).unwrap();
    /// assert_eq!(msg.to_vec().unwrap(), bytes);
    /// ```
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        use quick_protobuf::{message::MessageWrite, Writer};

        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf);
        self.write_message(&mut writer)
            .context("Failed serializing USP Msg to Protobuf")?;

        Ok(buf)
    }

    /// Render the `Msg` into a raw C string representation
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Msg` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let bytes = &[
    ///         0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74,
    ///         0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24,
    ///         0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10,
    ///         0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34,
    ///         0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f,
    ///         0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22,
    ///         0x03, 0x31, 0x2e, 0x33,
    ///     ];
    /// let msg = try_decode_msg(bytes).unwrap();
    /// assert_eq!(msg.to_c_str().unwrap(), "\"\\x0a\\x08\\x0a\\x04test\\x10\\x03\\x12(\\x0a\\x26B\\x24\\x0a\\x05notif\\x10\\x01B\\x19\\x0a\\x060044FF\\x12\\x03Foo\\x1a\\x0501234\\x22\\x031.3\"\n");
    /// ```
    pub fn to_c_str(&self) -> Result<String> {
        use std::fmt::Write as _;

        const fn check_printable(c: u8) -> bool {
            match c as char {
                ' ' | '.' | '!' | '(' | ')' | '\'' | ',' | '*' | '[' | ']' | '=' | '<' | '>'
                | '-' | '_' => true,
                _ if c.is_ascii_alphanumeric() => true,
                _ => false,
            }
        }

        let data = self.to_vec()?;
        let mut out = String::new();

        write!(out, "\"")?;
        for i in data.iter() {
            if check_printable(*i) {
                write!(out, "{}", char::from(*i))?;
            } else {
                write!(out, "\\x{i:02x}")?;
            }
        }
        writeln!(out, "\"")?;

        Ok(out)
    }

    /// Render the `Record` into a raw C array representation
    ///
    /// # Arguments
    ///
    /// * `self` - A USP `Record` structure
    ///
    /// # Example
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let bytes = &[
    ///         0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74,
    ///         0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24,
    ///         0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10,
    ///         0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34,
    ///         0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f,
    ///         0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22,
    ///         0x03, 0x31, 0x2e, 0x33,
    ///     ];
    /// let msg = try_decode_msg(bytes).unwrap();
    /// assert_eq!(msg.to_c_array().unwrap(), "unsigned int pb_len = 52;\nconst char pb[] = {\n  0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74, /* ____test */\n  0x10, 0x03, 0x12, 0x28, 0x0a, 0x26, 0x42, 0x24, /* ___(__B_ */\n  0x0a, 0x05, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x10, /* __notif_ */\n  0x01, 0x42, 0x19, 0x0a, 0x06, 0x30, 0x30, 0x34, /* _B___004 */\n  0x34, 0x46, 0x46, 0x12, 0x03, 0x46, 0x6f, 0x6f, /* 4FF__Foo */\n  0x1a, 0x05, 0x30, 0x31, 0x32, 0x33, 0x34, 0x22, /* __01234\" */\n  0x03, 0x31, 0x2e, 0x33,                         /* _1.3 */\n};\n");
    /// ```
    pub fn to_c_array(&self) -> Result<String> {
        use std::fmt::Write as _;

        const fn check_printable(c: u8) -> bool {
            match c as char {
                ' ' | '.' | '!' | '(' | ')' | '\'' | '"' | ',' | '*' | '[' | ']' | '=' | '<'
                | '>' | '-' | '_' => true,
                _ if c.is_ascii_alphanumeric() => true,
                _ => false,
            }
        }

        let data = self.to_vec()?;
        let mut out = String::new();

        const CHUNK_LEN: usize = 8;
        writeln!(out, "unsigned int pb_len = {};", data.len())?;
        writeln!(out, "const char pb[] = {{")?;
        for chunk in data.chunks(CHUNK_LEN) {
            write!(out, "  ")?;
            for i in chunk {
                write!(out, "0x{i:02x}, ")?;
            }

            for _ in chunk.len()..CHUNK_LEN {
                write!(out, "      ")?;
            }

            write!(out, "/* ")?;
            for i in chunk {
                if check_printable(*i) {
                    write!(out, "{}", char::from(*i))?;
                } else {
                    write!(out, "_")?;
                }
            }
            write!(out, " */")?;

            writeln!(out)?;
        }
        writeln!(out, "}};")?;

        Ok(out)
    }
}
