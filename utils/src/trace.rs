#[cfg(not(debug_assertions))]
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct Trace {}

#[cfg(not(debug_assertions))]
pub struct Trace {
    blocking: NonBlocking,
    guard: WorkerGuard,
}

impl Trace {
    #[cfg(debug_assertions)]
    pub fn init() -> Self {
        Self::debug_console();
        Self {}
    }

    #[cfg(not(debug_assertions))]
    pub fn init() -> Self {
        Self::release_init()
    }

    /// ## Debug Console fn
    /// use tracing_subscriber to log logs which level greater or equal to DEBUG with pretty
    /// enabled
    pub fn debug_console() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .pretty()
            .init();
    }

    #[cfg(not(debug_assertions))]
    pub fn release_init() -> Self {
        let (blocking, guard) =
            tracing_appender::non_blocking(tracing_appender::rolling::never("log", "todo.log"));
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .init();
        Self { blocking, guard }
    }
}
