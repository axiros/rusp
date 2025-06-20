![Continuous integration](https://github.com/axiros/rusp/workflows/Continuous%20integration/badge.svg)

# rusp

This is **rusp**, the **Rust USP** toolkit.

[Rust][] is a fast and safe systems programming language.

[USP][] (or User Services Platform) is a new protocol for management of connected devices, maintained and developed by the [Broadband Forum][BBF].

This repository has been proudly sponsored by [Axiros][], a market leader in device management and monitoring solutions:
![Axiros logo](https://raw.github.com/axiros/rusp/master/Axiros_logo.svg)

## What is rusp?

**rusp** is a toolkit, written in **Rust**, providing support to work with **USP** records and messages which are encoded in Protobuf bytestreams.

While a Protobuf schema exists which allows generating bindings for several commonly used programming languages, those are either uncomfortable to use and/or highly unsafe. By leveraging the strong Rust type system and the strict compiler, Rust is capable of detecting many incorrect or incomplete uses of the Protobuf encoding at compile time which allows for confident use of the **USP** protocol.

## What is included?

The toolkit currently provides these parts: 

1. [The Rusp library (aptly named: rusp-lib)](https://github.com/axiros/rusp/tree/master/rusp-lib)
2. [The Rhai-Rusp library](https://github.com/axiros/rusp/tree/master/rhai-rusp)
3. [The Rusp binaries](https://github.com/axiros/rusp/tree/master/rusp-bin)

## What else?

You may use this crate however you like under the **BSD 3-Clause Licence**.

The toolkit is still far from feature complete but already a big aid in our product development and also helped to find one or the other discrepancy in the standard.

Feel free to spread the word or drop us a note if you like it. Collaboration on this crate is highly welcome as are pull requests in [our GitHub repo](https://github.com/axiros/rusp/).

## Contact us

If you are in need of software for **USP** management software (agent, controller or testing) or expertise please get in touch with us via our [web form](https://www.axiros.com). We're also happy to solve all other device management and monitoring needs!

[Rust]: https://www.rust-lang.org/
[USP]: https://usp.technology/
[BBF]: https://www.broadband-forum.org/
[Axiros]: https://www.axiros.com/

Licence
-------

[BSD 3-Clause Licence](LICENSE).
