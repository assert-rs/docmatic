//! docmatic:
//!
//! `docmatic` runs `rustdoc` on your documentation files.
//!
//! ## Writing code blocks
//!
//! See ["Documentation tests"](https://doc.rust-lang.org/beta/rustdoc/documentation-tests.html)
//! for how to customize your code blocks being run as tests.
//!
//! ## Example
//!
//! First, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! docmatic = "0.1"
//! ```
//!
//! Next, in your test file:
//!
//! ```rust
//! extern crate docmatic;
//!
//! fn test_readme() {
//!     docmatic::assert_file("README.md");
//! }
//! ```

extern crate glob;
extern crate which;

use std::path;

pub fn assert_file<P>(documentation: P)
where
    P: AsRef<path::Path>,
{
    assert_file_impl(documentation.as_ref())
}

fn assert_file_impl(documentation: &path::Path) {
    let rustdoc = which::which("rustdoc").expect("run with rust toolchain available");

    let current_exe = std::path::Path::new(&std::env::current_exe().unwrap())
        .canonicalize()
        .unwrap();
    let deps = current_exe.parent().unwrap();

    let mut cmd = std::process::Command::new(rustdoc);
    cmd.arg("--verbose")
        .args(&["--library-path", deps.to_str().unwrap()])
        .arg("--test")
        .arg(documentation);

    let result = cmd.spawn()
        .expect("rustdoc is runnable")
        .wait()
        .expect("rustdoc can run");

    assert!(result.success(), "Failed to run rustdoc tests on README.md");
}
