![Continuous integration](https://github.com/axiros/rusp/workflows/Continuous%20integration/badge.svg)

This crate has been proudly sponsored by [Axiros][], a market leader in device management and monitoring solutions:
![Axiros logo](https://raw.github.com/axiros/rusp/master/Axiros_logo.svg)

# rusp-bin

`rusp-bin` (published on crates.io as `rusp` for backwards compatibility) contains the application part of the **rusp** ecosystem, which is based on the [rusp-lib](https://crates.io/crates/rusp-lib) and [rhai-rusp](https://crates.io/crates/rhai-rusp) crates.

##

This crate provides:

* A `rusp` application, granting access to library functionality via command line. As of version 1.0, this application is **deprecated** and superseded by the `rusp-run` application. At the moment this application supports:
  * Decoding of **USP** Msg Protobuf bytestreams from standard input
  * Decoding of **USP** Msg Protobuf bytestreams from file(s)
  * Decoding of **USP** Record Protobuf bytestreams from standard input
  * Decoding of **USP** Record Protobuf bytestreams from file(s)
  * Extraction of a **USP** Msg from the payload of a **USP** Record Protobuf bytestream
* A `rusp-run` application, providing a simple frontend to the `rhai-rusp` bindings via an embedded [Rhai][] interpreter

In order to download, compile and install the binaries, it is sufficient to have a stable [Rust][] environment and run:

```
# cargo install rusp
```

## How to use the deprecated `rusp` binary?

**rusp** includes a binary with the same name demonstrating some of the uses.

At the moment this mostly allows converting Protobuf encapsulated USP **Record** and **Msg** structures into human-readable text and other useful formats like code and to extracting a **Msg** structure from a **Record**.

Starting from version 1.0, the `rusp` binary is deprecated and will be removed in a future version altogether. All encoding functionality and Protobuf output have been removed, the latter due being incompatible with the notice printed about the upcoming removal of the tool. Please use `rhai-run` instead.

## How to use the `rhai-run` binary?

We are now also including a simply binary called `rusp-run`, demonstrating how
to embed the bindings and allowing you to execute `Rhai` code directly via
command line, read either from `stdin` or a file.

```
# rusp-run --script 'let body = rusp::get_builder().with_max_depth(1).with_params(["Device."]).build();
let msg = rusp::msg_builder().with_msg_id ("Foo").with_body (body).build();
print (msg)'
{
  "Header": {
    "msg_id": "Foo",
    "msg_type": "GET"
  },
  "Body": {
    "Request": {
      "Get": {
        "param_paths": [
          "Device."
        ],
        "max_depth": 1
      }
    }
  }
}
```

More examples can be found at [the Rhai-Rusp repository](https://github.com/axiros/rusp/tree/master/rhai-rusp).

Starting with version 0.99, `rusp-run` also supports a `-c` switch, which can
be used to process a Rhai script embedded in a ```/** */``` comment to e.g. turn
a comment into an array representing a USP Message or Record in a unittest.

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
