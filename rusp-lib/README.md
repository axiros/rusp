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

The toolkit is split into two parts: this is the library part, which contains:

* Generated low-level Rust Protobuf bindings
* A library providing:
  * Higher level access to de-/serialisation functionality
  * Convenience functions to generate messages
  * A builder-style API to generate USP Messages and Records of all kinds
  * Convenience functions to work with the native Msg types
  * Pretty printing of **USP** records and messages
  * Serde de-/serialisation of **USP** records and messages
  * Unittests and documentation (including doctests/examples)

## How to use it?

**rusp-lib** can be used as a library in your own Rust applications. To use **rusp** as a library you simply need to add the **rusp** and most likely **quick-protobuf** crates to your `Cargo.toml` as dependencies:

```
[dependencies]
rusp-lib = "0.95.0"
quick-protobuf = "0.8"
```

Please note that as of v0.95, the library and the application have been split into two parts, `rusp-lib` and `rusp`. In order to seamlessly continue what is now `rusp-lib` you have three options:

1. Rename all uses of the `rusp` namespace into `rusp_lib` in your source code
2. Put a `use rusp_lib as rusp;` line at the top of each code file using the `rusp` namespace
2. Rename the library via `Cargo.toml` by declaring the dependency as `rusp-lib = { version = "0.95", package = "rusp" }`

Documentation and examples for its use can be found on [docs.rs](https://docs.rs/rusp/latest/rusp-lib/index.html).

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
