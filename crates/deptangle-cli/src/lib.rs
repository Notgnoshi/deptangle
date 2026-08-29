use std::io::IsTerminal;

pub mod stdio;

/// Install the color-eyre error report handler and initialize stderr logging.
///
/// The default log level filter can be overridden with the `DEPTANGLE_LOG` environment variable.
/// Color is enabled only when stderr is a terminal.
pub fn init(log_level: tracing::Level) -> eyre::Result<()> {
    let use_color = std::io::stderr().is_terminal();
    if use_color {
        color_eyre::install()?;
    }

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(log_level.into())
        .with_env_var("DEPTANGLE_LOG")
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(use_color)
        .with_writer(std::io::stderr)
        .init();

    Ok(())
}
