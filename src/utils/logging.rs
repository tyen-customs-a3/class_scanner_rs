use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
    prelude::*,
};

/// Initialize logging with the specified log level.
/// 
/// # Examples
/// ```
/// use class_scanner::utils::init_logging;
/// init_logging(Some("debug")).expect("Failed to initialize logging");
/// ```
pub fn init_logging(level: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new(level.unwrap_or("info"))
        });

    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::FULL))
        .with(filter)
        .try_init()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging() {
        assert!(init_logging(Some("debug")).is_ok());
        // Second initialization should fail gracefully
        assert!(init_logging(Some("debug")).is_err());
    }
}