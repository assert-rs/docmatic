# docmatic

> Be dogmatic about working documentation.

[![Build Status](https://travis-ci.org/assert-rs/docmatic.svg?branch=master)](https://travis-ci.org/assert-rs/docmatic)
[![Build status](https://ci.appveyor.com/api/projects/status/bte7gyfxylva10ax?svg=true)](https://ci.appveyor.com/project/epage/docmatic)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)](https://docs.rs/docmatic)
![License](https://img.shields.io/crates/l/docmatic.svg)
[![Crates.io](https://img.shields.io/crates/v/docmatic.svg?maxAge=2592000)](https://crates.io/crates/docmatic)


## Usage

First, add this to your `Cargo.toml`:

```toml
[dev-dependencies]
docmatic = "0.1"
```

Next, in your test file:

```rust
extern crate docmatic;

fn test_readme() {
    docmatic::assert_file("README.md");
}
```

For more information on using docmatic, look at the
[documentation](https://docs.rs/docmatic)

## Why Docmatic?

Compared to doing nothing:
- When you have stale documentation, it gives a sour taste to those considering your crate

Compared to [`#![doc(include = "../README.md")]`](https://github.com/yoshuawuyts/human-panic/tree/ed11055e0602c3c8d223ed8354058fefb9ac47ec)
- Allows your README to focus on potential contributors and your API docs on potential users
- Doesn't require nightly

Compared to `rustdoc -L target/debug/deps/ --test README.md`
- Docmatic pros:
  - Easier for contributors to know how to run
- `rustdoc` pros:
  - Simple
  - No dependencies to muck with

Compared to [skeptic](https://github.com/budziq/rust-skeptic):
- Docmatic pros:
  - Fewer dependencies for faster build
  - Doesn't require exposing those dependencies to your users via a `[build-dependencies]`
  - Runs in the expected working directory
- Skeptic pros:
  - Skeptic templates
  - Better integration with `cargo test` (each block is a distinct test)

## License

`docmatic` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.


## Credits

We're grateful for all of the work done on
[skeptic](https://github.com/budziq/rust-skeptic), the spiritual predecessor to
docmatic and the work of [people iterating on a lighter weight
solution](https://github.com/budziq/rust-skeptic/issues/60).
