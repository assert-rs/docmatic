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

extern crate which;

use std::path;
use std::ffi::OsStr;

/// A specialized process builder managing a `rustdoc` test session.
///
/// # Example
///
/// The following code will test the crate README with the `docmatic`
/// configuration set and a default library path:
///
/// ```rust
/// extern crate docmatic;
///
/// use std::default::Default;
///
/// fn test_readme() {
///     docmatic::Assert::default()
///         .cfg("docmatic")
///         .test_file("README.md")
/// }
/// ```
pub struct Assert(std::process::Command);

impl Assert {
    /// Construct a new `Assert` with no flags set.
    ///
    /// Will likely fail if you don't provide at least one library path
    /// containing the tested crate. Instead, you should probably use
    /// [`Assert::default`]
    ///
    /// [`Assert::default`]: #tymethod.default
    pub fn new() -> Self {
        let executable = which::which("rustdoc").expect("rustdoc not found");
        Assert(std::process::Command::new(executable))
    }

    /// Add a path to the library paths passed to `rustdoc`.
    pub fn library_path<S>(&mut self, path: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.0.arg("--library-path").arg(path);
        self
    }

    /// Add a *cfg* to the configuration passed to `rustdoc`.
    pub fn cfg<S>(&mut self, cfg: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.0.arg("--cfg").arg(cfg);
        self
    }

    /// Test the given file, and panics on failure.
    pub fn test_file<P>(&mut self, path: P)
    where
        P: AsRef<path::Path>,
    {
        let process = self.0.arg("--test").arg(path.as_ref()).spawn();

        let result = process
            .expect("rustdoc is runnable")
            .wait()
            .expect("rustdoc can run");

        assert!(
            result.success(),
            format!("Failed to run rustdoc tests on '{:?}'", path.as_ref())
        );
    }
}

impl Default for Assert {
    /// Create an `Assert` instance with the following default parameters:
    ///
    /// * `--library-path` set to the current *deps* directory (`target/debug/deps` or
    ///   `target/release/deps` depending on the test compilation mode).
    ///
    fn default() -> Self {
        let mut assert = Self::new();
        let current_exe = std::env::current_exe()
            .and_then(|p| p.canonicalize())
            .expect("could not get path to test executable");
        assert.library_path(current_exe.parent().expect("parent exists"));
        assert
    }
}

/// Test a single file with default parameters.
pub fn assert_file<P>(documentation: P)
where
    P: AsRef<path::Path>,
{
    Assert::default().test_file(documentation);
}
