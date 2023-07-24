use tracing_appender::rolling;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

pub mod configuration;
pub mod routes;
pub mod startup;

type NonBlockingAppender = (
    tracing_appender::non_blocking::NonBlocking,
    tracing_appender::non_blocking::WorkerGuard,
);

pub fn setup_subscriber(env_filter: EnvFilter, is_debug: bool) -> (NonBlockingAppender, EnvFilter) {
    let log_file: rolling::RollingFileAppender = rolling::hourly(
        "./logs",
        match is_debug {
            false => "Main.Log",
            true => "Test.Log",
        },
    );
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(log_file);

    ((non_blocking_appender, guard), env_filter)
}

pub fn init_subscriber(
    non_blocking: tracing_appender::non_blocking::NonBlocking,
    env_filter: EnvFilter,
) {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .with_ansi(false)
        .with_line_number(true)
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();
}
