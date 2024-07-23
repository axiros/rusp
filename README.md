![Continuous integration](https://github.com/axiros/rusp/workflows/Continuous%20integration/badge.svg)

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
* Generated lowlevel Rust Protobuf bindings
* A library providing:
  * Higher level access to de-/serialisation functionality
  * Convenience functions to generate messages
  * Convenience functions to work with the native Msg types
  * Pretty printing of **USP** records and messages
  * Serde de-/serialisation of **USP** records and messages
  * Unittests and documentation (including doctests/examples)
* A **rusp** binary granting access to library functionality via command line. Included functionality at the moment are:
  * Decoding of **USP** Msg Protobuf bytestreams from standard input
  * Decoding of **USP** Msg Protobuf bytestreams from file(s)
  * Decoding of **USP** Record Protobuf bytestreams from standard input
  * Decoding of **USP** Record Protobuf bytestreams from file(s)
  * Extraction of **USP** Msg Protobuf bytestreams from the payload of a **USP** Record Protobuf bytestream
  * Generation of **USP** Msg Protobuf bytestreams and C char arrays for selected messages and Error

## How to use it?

### rusp binary

**rusp** includes a binary with the same name demonstrating some of the uses.

At the moment this mostly allows converting Protobuf encapsulated USP **Record** and **Msg** structures into human-readable text. It also allows extracting a **Msg** structure from a **Record** for own implementations as well as to encode USP
* **Add**
* **Delete**
* **Error**
* **Get**
* **GetInstances**
* **GetResp**
* **GetSupportedDM**
* **GetSupportedProtocol**
* **Notify**
* **NotifyResp**
* **Operate**
* **Set**

messages via command line tool.

**NEWSFLASH**: 0.17 adds some basic USP 1.4 support (breaking API, hence the bump)

In order to download, compile and install the `rusp` binary it is sufficient to have a stable Rust environment and run:

```
# cargo install rusp
```

After this you should be able to use the `rusp` binary, which has built-in help for all the commands to guide the way.

### rusp library

**rusp** can also be used as a library in your own Rust applications. To use **rusp** as a library you simply need to add the **rusp** and most likely **quick-protobuf** crates to your `Cargo.toml` as dependencies:

```
...

[dependencies]
rusp = "0.17"
quick-protobuf = "0.8"

...
```

Documentation and examples for its use can be found on [docs.rs](https://docs.rs/rusp/latest/rusp/index.html).

## What else?

You may use this crate however you like under the **BSD 3-Clause Licence**.

The toolkit is still far from feature complete but already a big aid in our product development and also helped to find one or the other discrepancy in the standard.

Feel free to spread the word or drop us a note if you like it. Collaboration on this crate is highly welcome as are pull requests!

## Contact us

If you are in need of software for **USP** management software (agent, controller or testing) or expertise please get in touch with us via our [web form](https://www.axiros.com). We're also happy to solve all other device management and monitoring needs!

[Rust]: https://www.rust-lang.org/
[USP]: https://usp.technology/
[BBF]: https://www.broadband-forum.org/
[Axiros]: https://www.axiros.com/

Licence
-------

[BSD 3-Clause Licence](LICENSE).
