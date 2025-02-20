use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Registry};

pub fn init_logger() {
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);
    let file_appender = rolling::daily("logs", "work_diary.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    Registry::default()
        .with(formatting_layer)
        .with(file_layer)
        .init();
}
