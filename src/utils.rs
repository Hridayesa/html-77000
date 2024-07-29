use std::path::{Path, PathBuf};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn join_file_path(base_dir_name: &str, file_name: &str) -> PathBuf {
    Path::new(base_dir_name).join(file_name)
}
pub fn init_logger() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");    // a builder for `FmtSubscriber`.
}