use crate::usp::{self, Error, Msg, Notify};
use crate::usp_record::Record;
use quick_protobuf::message::MessageRead;
use quick_protobuf::BytesReader;

use anyhow::{Context, Result};

/// Decodes a slice of bytes containing a Protobuf encoded USP Record into a Record structure for
/// further processing
///
/// # Arguments
///
/// * `bytes` - A slice of bytes containing the Protobuf encoded USP Record
///
/// # Example
///
/// ```
/// use rusp::usp_decoder::try_decode_record;
/// let record =
///     try_decode_record(&[
///         0x0a, 0x03, 0x31, 0x2e, 0x30, 0x1a, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
///         0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
///         0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
///         0x3a, 0x4d, 0x12, 0x4b, 0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74, 0x10, 0x03,
///         0x12, 0x3f, 0x0a, 0x3d, 0x42, 0x3b, 0x0a, 0x0f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72,
///         0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x42, 0x28, 0x0a, 0x03, 0x6f,
///         0x75, 0x69, 0x12, 0x0d, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x5f, 0x63, 0x6c,
///         0x61, 0x73, 0x73, 0x1a, 0x0d, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x5f, 0x6e, 0x75,
///         0x6d, 0x62, 0x65, 0x72, 0x22, 0x03, 0x31, 0x2e, 0x30,
///     ]);
/// ```
pub fn try_decode_record(bytes: &[u8]) -> Result<Record> {
    let mut reader = BytesReader::from_bytes(bytes);
    Record::from_reader(&mut reader, bytes).context("while parsing protobuf as USP Record")
}

/// Decodes a slice of bytes containing a Protobuf encoded USP Msg into a Msg structure for further
/// processing
///
/// # Arguments
///
/// * `bytes` - A slice of bytes containing the Protobuf encoded USP Message
///
/// # Example
///
/// ```
/// use rusp::usp_decoder::try_decode_msg;
/// let msg =
///     try_decode_msg(&[
///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
///         0x65, 0x74, 0x65, 0x72
///     ]);
/// ```
pub fn try_decode_msg(bytes: &[u8]) -> Result<Msg> {
    let mut reader = BytesReader::from_bytes(bytes);
    Msg::from_reader(&mut reader, bytes).context("while parsing protobuf as USP Message")
}

/// Implementation of some extension methods for `Msg`s
impl<'a> Msg<'a> {
    /// Retrieves the message ID from a Msg structure
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
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]);
    /// assert_eq!(msg.unwrap().msg_id(), "AXSS-1544114045.442596");
    /// ```
    pub fn msg_id(&'a self) -> &str {
        if let Some(header) = self.header.as_ref() {
            header.msg_id.as_ref()
        } else {
            ""
        }
    }

    /// Checks whether the body contains a message of type request
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
    /// assert_eq!(msg.is_request(), true);
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]).unwrap();
    /// assert_eq!(msg.is_request(), false);
    /// ```
    pub fn is_request(&'a self) -> bool {
        if let Some(body) = self.body.as_ref() {
            matches!(&body.msg_body, usp::mod_Body::OneOfmsg_body::request(_))
        } else {
            false
        }
    }

    /// Checks whether the body contains a message of type notify (request)
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
    /// assert_eq!(msg.is_notify_request(), true);
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]).unwrap();
    /// assert_eq!(msg.is_notify_request(), false);
    /// ```
    pub fn is_notify_request(&'a self) -> bool {
        self.get_notify_request().is_some()
    }

    /// Retrieves the notify request from the Msg
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
    /// assert!(msg.get_notify_request().is_some());
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]).unwrap();
    /// assert!(msg.get_notify_request().is_none());
    /// ```
    pub fn get_notify_request(&'a self) -> Option<&Notify> {
        if let Some(body) = self.body.as_ref() {
            if let usp::mod_Body::OneOfmsg_body::request(request) = &body.msg_body {
                if let usp::mod_Request::OneOfreq_type::notify(notify) = &request.req_type {
                    return Some(notify);
                }
            }
        }

        None
    }

    /// Checks whether the body contains a message of type response
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
    /// assert_eq!(msg.is_response(), false);
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]).unwrap();
    /// assert_eq!(msg.is_response(), true);
    /// ```
    pub fn is_response(&'a self) -> bool {
        if let Some(body) = self.body.as_ref() {
            matches!(&body.msg_body, usp::mod_Body::OneOfmsg_body::response(_))
        } else {
            false
        }
    }

    /// Checks whether the body contains a message of type response
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
    /// assert_eq!(msg.is_error(), false);
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]).unwrap();
    /// assert_eq!(msg.is_error(), false);
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x05, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x12,
    ///         0x17, 0x1a, 0x15, 0x0d, 0x5b, 0x1b, 0x00, 0x00,
    ///         0x12, 0x0e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    ///         0x61, 0x6c, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    ///     ]).unwrap();
    /// assert_eq!(msg.is_error(), true);
    /// ```
    pub fn is_error(&'a self) -> bool {
        self.get_error().is_some()
    }

    /// Retrieves the notify request from the Msg
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
    /// assert!(msg.get_error().is_none());
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34,
    ///         0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35,
    ///         0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42, 0x0a, 0x40,
    ///         0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
    ///         0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e,
    ///         0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
    ///         0x15, 0x62, 0x1b, 0x00, 0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70,
    ///         0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    ///         0x65, 0x74, 0x65, 0x72
    ///     ]).unwrap();
    /// assert!(msg.get_error().is_none());
    /// ```
    ///
    /// ```
    /// use rusp::usp_decoder::try_decode_msg;
    /// let msg =
    ///     try_decode_msg(&[
    ///         0x0a, 0x05, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x12,
    ///         0x17, 0x1a, 0x15, 0x0d, 0x5b, 0x1b, 0x00, 0x00,
    ///         0x12, 0x0e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    ///         0x61, 0x6c, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    ///     ]).unwrap();
    /// assert!(msg.get_error().is_some());
    /// ```
    pub fn get_error(&'a self) -> Option<&Error> {
        if let Some(body) = self.body.as_ref() {
            if let usp::mod_Body::OneOfmsg_body::error(error) = &body.msg_body {
                return Some(error);
            }
        }

        None
    }

    /// Encode the Msg as "native" JSON format
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
    pub fn to_json(&'a self) -> Option<String> {
        serde_json::to_string(self).ok()
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
            .context("Failed serializing USP Msg")?;

        Ok(buf)
    }
}
