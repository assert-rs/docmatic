# docmatic

> Be dogmatic about working documentation.

[![Build Status](https://travis-ci.org/assert-rs/docmatic.svg?branch=master)](https://travis-ci.org/assert-rs/docmatic)
[![Build status](https://ci.appveyor.com/api/projects/status/enru6k55xme867u6?svg=true)](https://ci.appveyor.com/project/epage/docmatic)
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


## License

`docmatic` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.


## Credits

We're grateful for all of the work done on
[skeptic](https://github.com/budziq/rust-skeptic), the spiritual predecessor to
docmatic and the work of [people iterating on a lighter weight
solution](https://github.com/budziq/rust-skeptic/issues/60).
