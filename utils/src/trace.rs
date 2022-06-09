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
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .pretty()
            .init();
        Self {}
    }

    #[cfg(not(debug_assertions))]
    pub fn init() -> Self {
        let (blocking, guard) =
            tracing_appender::non_blocking(tracing_appender::rolling::never("log", "todo.log"));
        #[cfg(not(debug_assertions))]
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .init();
        Self { blocking, guard }
    }
}
