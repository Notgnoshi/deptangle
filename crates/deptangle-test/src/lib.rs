use std::process::Output;

pub mod prelude {
    pub use crate::{CommandExt, tempfile, tool};
}

pub trait CommandExt {
    /// Same as `output()` except with hooks to print stdout/stderr in failed tests
    fn captured_output(&mut self) -> Output;
}

impl CommandExt for assert_cmd::Command {
    #[track_caller]
    fn captured_output(&mut self) -> Output {
        let output = self.output().expect("failed to run command");

        // libtest injects magic in print! macros to capture output in tests
        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));

        output
    }
}

impl CommandExt for std::process::Command {
    #[track_caller]
    fn captured_output(&mut self) -> Output {
        let output = self.output().expect("failed to run command");

        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));

        output
    }
}

/// Get a temporary file with the given contents
pub fn tempfile<S: AsRef<str>>(contents: S) -> eyre::Result<tempfile::NamedTempFile> {
    let mut file = tempfile::NamedTempFile::new()?;
    std::io::Write::write_all(&mut file, contents.as_ref().as_bytes())?;
    Ok(file)
}

/// Get a command to run the given tool binary.
///
/// Uses `CARGO_BIN_EXE_<name>` which cargo sets at compile time for
/// integration tests in the same crate as the binary.
///
/// # Example
/// ```ignore
/// use deptangle_test::prelude::*;
///
/// let output = tool!("depconv").write_stdin("a b\n").captured_output();
/// ```
#[macro_export]
macro_rules! tool {
    ($name:literal) => {{
        let mut cmd = assert_cmd::Command::new(env!(concat!("CARGO_BIN_EXE_", $name)));
        cmd.arg("--log-level=TRACE");
        cmd
    }};
}

#[ctor::ctor(unsafe)]
fn setup_test_logging() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::Level::DEBUG.into())
        .with_env_var("DEPTANGLE_LOG")
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_test_writer()
        .init();
}
