# rusp

**rusp** is the **Rust USP** toolkit, brought to you by Axiros!

[Rust][] is a fast and safe systems programming language.

[USP][] (or User Services Platform) is a new protocol for management of connected devices, maintained and developed by the [Broadband Forum][BBF].

[Axiros][] is a leading company for device management.

## What is rusp?

**rusp** is a toolkit, written in **Rust**, providing support to work with **USP** records and messages which are encoded in Protobuf bytestreams.

While a Protobuf schema exists which allows generating bindings for several commonly used programming languages, those are either uncomfortable to use and/or highly unsafe. By leveraging the strong Rust type system and the strict compiler, Rust is capable of detecting many incorrect or incomplete uses of the Protobuf encoding at compile time which allows for confident use of the **USP** protocol.

## What is included?

The toolkit includes:
* Generated lowlevel Rust protobuf bindings
* A library providing:
  * Higher level access to serialisation/deserialisation functionality
  * Convenience functions to generate messages
  * Pretty printing of **USP** records and messages
  * Unittests and documentation (including doctests/examples)
* A **rusp** binary granting access to library functionality via command line. Included functionality at the moment are:
  * Decoding of **USP** Msg Protobuf bytestreams from standard input
  * Decoding of **USP** Msg Protobuf bytestreams from file(s)
  * Decoding of **USP** Record Protobuf bytestreams from standard input
  * Decoding of **USP** Record Protobuf bytestreams from file(s)
  * Extraction of **USP** Msg Protobuf bytestreams from the payload of a **USP** Record Protobuf bytestream

## What else?

You may use this crate however you like under the **BSD 3-Clause License**.

The toolkit is still far from feature complete but already a big aid in our product development and also helped to find one or the other discrepancy in the standard.

Feel free to spread the word or drop us a note if you like it. Collaboration on this crate is highly welcome as are pull requests!

## Contact us

If you are in need of software for **USP** management software (agent, controller or testing) or expertise please get in touch with us via our [web form](https://www.axiros.com/contact-us/). We're also happy to solve all other your device management and monitoring needs!

[Rust]: https://www.rust-lang.org/
[USP]: https://usp.technology/
[BBF]: https://www.broadband-forum.org/
[Axiros]: https://www.axiros.com/

License
-------

[BSD 3-Clause License](LICENSE).
