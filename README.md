# 3rd party BOLOS SDK for Rust

_bolos-rs_ is a 3rd party (experimental) BOLOS SDK written in Rust for developing applications for the Ledger hardware security devices. At this time, it is primarily a research project that explores what benefits a Rust based SDK can bring to the Ledger developer experience.

## Quick start

Check out [demos/](https://github.com/roosmaa/bolos-rs/tree/master/demos) folder for instructions on how to build this code. 

## Why does this exist?

While developing Ledger applications with the [official Ledger C SDK](https://github.com/LedgerHQ/nanos-secure-sdk), I saw and experienced quite a few ways to shoot oneself painfully in the foot. I got to thinking several times how a more powerful type system (like the one Rust features) could be used to model APIs in a way that would protect the users. And some time later I found myself scratching that itch.

## Project goals

The overall direction for _bolos-rs_ is to create a superior developer experience than that of the official C SDK.

- Quick and easy development environment setup by relying on the Rust infrastructure
- Application code should be 100% safe Rust
- Opinionated APIs that enforce (or nudge towards) safety

## Project status

It is not possible to build anything useful currently. APIs will have a lot of breaking changes, there is no documentation nor tests. In short - you shouldn't use it.

What works:

- Rendering of user interfaces

What doesn't work:

- Communicating with the host computer via USB
- Communicating with the browser via U2F
- Invoking various cryptography related fuctions from the firmware
- (and many other smaller things)

## License

This code is intentionally without a license. I haven't decided on the license yet, so you may not use this code in any way. If you do, however, happen to read through it and want to give some feedback, then please [open an issue](https://github.com/roosmaa/bolos-rs/issues/new).