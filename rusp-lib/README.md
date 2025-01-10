![Continuous integration](https://github.com/axiros/rusp/workflows/Continuous%20integration/badge.svg)

# rusp-lib

The crate contains is the library part or Rusp, which contains:

* Generated low-level Rust Protobuf bindings
* A library providing:
  * Higher level access to de-/serialisation functionality
  * Convenience functions to generate messages
  * A builder-style API to generate USP Messages and Records of all kinds
  * Convenience functions to work with the native Msg types
  * Pretty printing of **USP** records and messages
  * Serde de-/serialisation of **USP** records and messages
  * Unittests and documentation (including doctests/examples)

## How to use rusp-lib?

**rusp-lib** can be used as a library in your own Rust applications. To use **rusp** as a library you simply need to add the **rusp** and most likely **quick-protobuf** crates to your `Cargo.toml` as dependencies:

```
[dependencies]
rusp-lib = "0.96.0"
quick-protobuf = "0.8"
```

Please note that as of v0.95, the library and the application have been split into two parts, `rusp-lib` and `rusp`. In order to seamlessly continue what is now `rusp-lib` you have three options:

1. Rename all uses of the `rusp` namespace into `rusp_lib` in your source code
2. Put a `use rusp_lib as rusp;` line at the top of each code file using the `rusp` namespace
2. Rename the library via `Cargo.toml` by declaring the dependency as `rusp = { version = "0.96", package = "rusp-lib" }`

Documentation and examples for its use can be found on [docs.rs](https://docs.rs/rusp/latest/rusp-lib/index.html).

## What else?

You may use this crate however you like under the [BSD 3-Clause Licence](LICENSE).

Feel free to spread the word or drop us a note if you like it. Collaboration on
this crate is highly welcome as are pull requests in [our GitHub
repo](https://github.com/axiros/rusp/).

## Contact us

If you are in need of software for [USP][] management software (agent,
controller or testing) or expertise please get [in touch with us][Axiros]. We're
also happy to solve all other device management and monitoring needs!

Licence
-------

[BSD 3-Clause Licence](LICENSE).

[Rhai]: https://rhai.rs
[Rust]: https://www.rust-lang.org/
[USP]: https://usp.technology/
[Axiros]: https://www.axiros.com/
[BBF]: https://www.broadband-forum.org/
