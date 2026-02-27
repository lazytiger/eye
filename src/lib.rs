use derive_more::{Display, Error};
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Error, Debug, Display)]
pub struct OptionIsNone;

pub trait OptionToResult<T> {
    fn to_ok(self) -> anyhow::Result<T>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_ok(self) -> anyhow::Result<T> {
        self.ok_or_else(|| OptionIsNone.into())
    }
}

#[cfg(any(target_os = "ios", target_os = "android"))]
const LOG_LEVEL: tracing::metadata::LevelFilter = tracing::metadata::LevelFilter::INFO;

#[cfg(target_os = "android")]
pub fn init_tracing(_: Option<PathBuf>) -> anyhow::Result<Option<WorkerGuard>> {
    fn tracing_level_filter(level: tracing::metadata::LevelFilter) -> tracing::log::LevelFilter {
        match level {
            tracing::metadata::LevelFilter::DEBUG => tracing::log::LevelFilter::Debug,
            tracing::metadata::LevelFilter::TRACE => tracing::log::LevelFilter::Trace,
            tracing::metadata::LevelFilter::INFO => tracing::log::LevelFilter::Info,
            tracing::metadata::LevelFilter::WARN => tracing::log::LevelFilter::Warn,
            tracing::metadata::LevelFilter::ERROR => tracing::log::LevelFilter::Error,
            tracing::metadata::LevelFilter::OFF => tracing::log::LevelFilter::Off,
        }
    }

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(tracing_level_filter(LOG_LEVEL))
            .with_tag("eye"),
    );
    Ok(None)
}

#[cfg(not(target_os = "android"))]
pub fn init_tracing(log_path: Option<std::path::PathBuf>) -> anyhow::Result<Option<WorkerGuard>> {
    let (writer, guard) = if let Some(log_path) = log_path {
        let path = if log_path.is_dir() {
            log_path.as_path()
        } else {
            log_path.parent().to_ok()?
        };
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        tracing_appender::non_blocking(tracing_appender::rolling::daily(
            path,
            "eye.log",
        ))
    } else {
        tracing_appender::non_blocking(std::io::stdout())
    };
    let builder = tracing_subscriber::fmt::Subscriber::builder();
    #[cfg(target_os = "ios")]
    let builder = builder.with_max_level(LOG_LEVEL);
    #[cfg(not(target_os = "ios"))]
    let builder = builder.with_env_filter(
        tracing_subscriber::EnvFilter::builder()
            .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
            .from_env_lossy(),
    );
    let subscriber = builder
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_writer(writer)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(Some(guard))
}
