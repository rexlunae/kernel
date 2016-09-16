
extern crate log;

use log::{LogRecord, LogMetadata, LogLevelFilter, SetLoggerError, ShutdownLoggerError};

#[derive(Copy, Clone, Debug)]
pub struct SysLog{
    log_level: LogLevelFilter
}

impl log::Log for SysLog {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        //metadata.level() <= &self.log_level
        true
    }

    fn log(&self, record: &LogRecord) {

    }
}

pub fn init() -> Result<(), SetLoggerError> {
    unsafe {
        log::set_logger_raw(|max_log_level| {
            static LOGGER: SysLog = SysLog{ log_level: LogLevelFilter::Info } ;
            max_log_level.set(LogLevelFilter::Info);
            &LOGGER
        })
    }
}
pub fn shutdown() -> Result<(), ShutdownLoggerError> {
    log::shutdown_logger_raw().map(|logger| {
        let logger = unsafe { &*(logger as *const SysLog) };
        //logger.flush();
    })
}
