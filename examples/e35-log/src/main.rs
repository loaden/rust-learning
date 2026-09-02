#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    error!("Error log.");
    warn!("Warn log.");
    info!("Info log");
    debug!("Debug log.");
    trace!("Trace log.");
}
