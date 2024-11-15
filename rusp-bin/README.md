![Continuous integration](https://github.com/axiros/rusp/workflows/Continuous%20integration/badge.svg)

# rusp-bin

This crate contains the application part of Rusp, which contains:

* A **rusp** binary granting access to library functionality via command line. Included functionality at the moment are:
  * Decoding of **USP** Msg Protobuf bytestreams from standard input
  * Decoding of **USP** Msg Protobuf bytestreams from file(s)
  * Decoding of **USP** Record Protobuf bytestreams from standard input
  * Decoding of **USP** Record Protobuf bytestreams from file(s)
  * Extraction of **USP** Msg Protobuf bytestreams from the payload of a **USP** Record Protobuf bytestream
  * Generation of **USP** Msg Protobuf bytestreams and C char arrays for selected messages and Error

## How to use the rusp binary?

**rusp** includes a binary with the same name demonstrating some of the uses.

At the moment this mostly allows converting Protobuf encapsulated USP **Record** and **Msg** structures into human-readable text and other useful formats like code and to extracting a **Msg** structure from a **Record**.

Currently, it also still supports synthesizing the following USP Messages

* **Add**
* **Delete**
* **Deregister**
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
* **Register**

and USP Records via command line, however this feature is going to be phased out soon in favour of something way better. 😉

In order to download, compile and install the `rusp` binary it is sufficient to have a stable Rust environment and run:

```
# cargo install rusp
```

After this you should be able to use the `rusp` binary, which has built-in help for all the commands to guide the way.

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
