//! A toolkit, written in **[Rust][]**, providing support to work with **[USP][]** Records and Messages which are encoded in Protobuf bytestreams.
//!
//! # What is it?
//!
//! While a Protobuf schema exists which allows generating bindings for several commonly used
//! programming languages, those are either uncomfortable to use and/or highly unsafe. By leveraging
//! the strong Rust type system and the strict compiler, Rust is capable of detecting many incorrect
//! or incomplete uses of the Protobuf encoding at compile time which allows for confident use of the
//! **[USP][]** protocol. Thise crate provides an abstraction over the automatically generated
//! protobuf De-/Serialisers as well as a tool to work with USP Records and Messages on the
//! commandline.
//!
//! # What is included?
//!
//! The toolkit includes:
//! * Generated lowlevel Rust Protobuf bindings
//!   * [USP Record][`rusp::usp_record`]
//!   * [USP Messages][`rusp::usp`]
//! * A library providing:
//!   * Higher level access to [de-][`rusp::usp_decoder`]/[serialisation][`rusp::usp_generator`] functionality
//!   * Convenience functions to [generate messages][`rusp::usp_generator`]
//!   * Convenience functions to [work with the native Msg types][`rusp::usp_decoder::MsgTools`]
//!   * Pretty printing of **USP** Records and Messages
//!   * Serde de-/serialisation of **USP** Records and Messages
//!   * Unittests and documentation (including doctests/examples)
//! * A **rusp** binary granting access to library functionality via command line. Included functionality at the moment are:
//!   * Decoding of **USP** Msg Protobuf bytestreams from standard input
//!   * Decoding of **USP** Msg Protobuf bytestreams from file(s)
//!   * Decoding of **USP** Record Protobuf bytestreams from standard input
//!   * Decoding of **USP** Record Protobuf bytestreams from file(s)
//!   * Extraction of **USP** Msg Protobuf bytestreams from the payload of a **USP** Record Protobuf bytestream
//!   * Generation of **USP** Msg Protobuf bytestreams and C char arrays for selected messages and Error
//!
//! [Rust]: https://www.rust-lang.org/
//! [USP]: https://usp.technology/
//! [BBF]: https://www.broadband-forum.org/
//! [Axiros]: https://www.axiros.com/
//! [`rusp::usp`]: crate::usp
//! [`rusp::usp_record`]: crate::usp_record
//! [`rusp::usp_decoder`]: crate::usp_decoder
//! [`rusp::usp_generator`]: crate::usp_generator
//! [`rusp::usp_decoder::MsgTools`]: crate::usp_decoder::MsgTools

/// Automatically generated bindings for USP Msgs from the [`USP Messages protobuf schema`]
///
/// [`USP Messages protobuf schema`]: <https://usp.technology/specification/usp-msg-1-3.proto>
pub mod usp;
/// Helper functions to decode a protobuf encoded bytestream into Rust types
pub mod usp_decoder;
/// Helper functions to generate USP Messages
pub mod usp_generator;
/// Automatically generated bindings for USP Recors from the [`USP Records protobuf schema`]
///
/// [`USP Records protobuf schema`]: <https://usp.technology/specification/usp-record-1-3.proto>
pub mod usp_record;
/// Helper types to simplify handling of commonly used notifications
pub mod usp_types;

mod usp_json;
