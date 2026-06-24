pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let msg = record.args().to_string().into();

            match record.level() {
                log::Level::Trace => web_sys::console::debug_1(&msg),
                log::Level::Debug => web_sys::console::log_1(&msg),
                log::Level::Info => web_sys::console::info_1(&msg),
                log::Level::Warn => web_sys::console::warn_1(&msg),
                log::Level::Error => web_sys::console::error_1(&msg),
            }
        }
    }

    fn flush(&self) {}
}
