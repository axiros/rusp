![Continuous integration](https://github.com/axiros/rusp/workflows/Continuous%20integration/badge.svg)

# `rhai-rusp`

`rhai-rusp` is a new approach to address your [USP][] protocol needs by
providing [Rhai][] bindings for the [rusp-lib crate](https://crates.io/crates/rusp-lib), thus providing scripting capabilities
to USP in Rhai for either standalone runners or [Rust][] applications which can
embed the [Rhai][] interpreter together with the `rhai-rusp` bindings.

## How to embed `rhai-rusp`?

`rhai-rusp` can be used as a library in your own Rust applications to embed a
Rhai interpreter . To use `rhai-rusp` as a library, you simply need to add the
`rhai-rusp` crate to your `Cargo.toml` as dependency:

```
[dependencies]
rhai-rusp = "0.96.0"
```

The usual steps to embed a Rhai interpreter with rusp support are:

1. Initialise a Rhai engine via `rhai::Engine::new()`
2. Add the `rusp` bindings into the namespace per
`engine.register_static_module("rusp", RuspPackage::new().as_shared_module())`
3. Compile the `Rhai` code into an AST via `engine.compile(contents)`
4. Execute the `Rhai` AST via `engine.run_ast(ast)`
5. Handle errors, err... profit!

There's also the `rusp-run` binary as part of the [rusp crate](https://crates.io/crates/rusp) which you can install via:

```
# cargo install rusp
```

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