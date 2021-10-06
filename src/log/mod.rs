use slog::Logger;
use slog::Drain;

mod log_macros;

pub use log_macros::*;

struct Log {
    log: Option<Logger>,
}

impl Log {
    fn new() -> Log {
        Log {
            log: Some(Log::create_logger()),
        }
    }

    fn create_logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        slog::Logger::root(drain, o!())
    }

    fn log(&self) -> &Option<Logger> {
        &self.log
    }

    fn flush(&mut self) {
        self.log = Some(Log::create_logger());
    }

    fn destroy(&mut self) {
        self.log = None;
    }
}