//! A toolkit, written in **[Rust][]**, providing support to work with **[USP][]** Records and Messages which are encoded in Protobuf bytestreams.
//!
//! # What is it?
//!
//! While a Protobuf schema exists which allows generating bindings for several commonly used
//! programming languages, those are either uncomfortable to use and/or highly unsafe. By leveraging
//! the strong Rust type system and the strict compiler, Rust is capable of detecting many incorrect
//! or incomplete uses of the Protobuf encoding at compile time which allows for confident use of the
//! **[USP][]** protocol. This crate provides an abstraction over the automatically generated
//! Protobuf De-/Serialisers as well as a tool to work with USP Records and Messages on the
//! command line.
//!
//! # What is included?
//!
//! The toolkit includes:
//! * Generated low-level Rust Protobuf bindings
//!   * [USP Record][`rusp::usp_record`]
//!   * [USP Messages][`rusp::usp`]
//! * A library providing:
//!   * Higher level access to [deserialisation][`rusp::usp_decoder`] functionality
//!   * Convenience functions to [work with the native Msg types][`rusp::usp_decoder`]
//!   * Pretty printing of **USP** Records and Messages
//!   * Serde de-/serialisation of **USP** Records and Messages
//!   * Unittests and documentation (including doctests/examples)
//! * A **rusp** binary granting access to library functionality via command line. Included functionality at the moment are:
//!   * Decoding of **USP** Msg Protobuf byte streams from standard input
//!   * Decoding of **USP** Msg Protobuf byte streams from file(s)
//!   * Decoding of **USP** Record Protobuf byte streams from standard input
//!   * Decoding of **USP** Record Protobuf byte streams from file(s)
//!   * Extraction of **USP** Msg Protobuf byte streams from the payload of a **USP** Record Protobuf byte stream
//!   * Generation of **USP** Msg Protobuf byte streams and C char arrays for selected messages and Error
//!
//! [Rust]: https://www.rust-lang.org/
//! [USP]: https://usp.technology/
//! [BBF]: https://www.broadband-forum.org/
//! [Axiros]: https://www.axiros.com/
//! [`rusp::usp`]: crate::usp
//! [`rusp::usp_record`]: crate::usp_record
//! [`rusp::usp_decoder`]: crate::usp_decoder

/// Automatically generated bindings for USP Msgs from the [`USP Messages Protobuf schema`]
///
/// [`USP Messages Protobuf schema`]: <https://usp.technology/specification/usp-msg-1-3.proto>
pub mod usp;

/// Builder style functions to generate USP Messages
pub mod usp_builder;

/// Helper functions to decode a Protobuf encoded byte stream into Rust types
pub mod usp_decoder;

/// Helper functions to encode native Rust types into Protobuf encoded byte streams
pub mod usp_encoder;

/// Automatically generated bindings for USP Records from the [`USP Records Protobuf schema`]
///
/// [`USP Records Protobuf schema`]: <https://usp.technology/specification/usp-record-1-3.proto>
pub mod usp_record;

/// Helper functions for checking and conversion of USP error codes and messages
pub mod usp_errors;

mod usp_json;

#[doc(hidden)]
pub use ::core;

/// Macro to help construct the `non_exhaustive` types in [`usp`] and [`usp_record`]
///
/// Example:
///
/// ```rust
/// use rusp_lib::fru;
/// use rusp_lib::usp::error::ParamError;
/// use rusp_lib::usp::Error;
/// use rusp_lib::usp_errors::get_err_msg;
///
/// let err_code = 7006;
/// let err_msg: String = get_err_msg(err_code).into();
/// let param = fru!(
///     ParamError {
///         param_path: "Path.Foo".into(),
///         err_code: err_code,
///         err_msg: err_msg.clone(),
///         ..
///     }
/// );
///
/// let _err = fru!(
///     Error {
///         err_code: err_code,
///         err_msg: err_msg,
///         param_errs: vec![param],
///         ..
///     }
/// );
/// ```
#[macro_export]
macro_rules! fru {(
    $Struct:path {
        $(
            $field_name:ident : $value:expr,
        )*
        .. $($initial:expr)?
    }
) => ({
    let it = $crate::core::default::Default::default();
    $(
        let () = it;
        let it = $initial;
    )?
    let mut it = it;
    let $Struct { $($field_name: _,)* .. } = it; // Ensure any `DerefMut` shenanigans are shadowed
    $(
        it.$field_name = $value;
    )*
    it
})}
