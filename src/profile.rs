use log::Record;
use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::constants;

static ID: AtomicUsize = AtomicUsize::new(0);

pub fn profile_core<L, F, T>(label: L, location: &'static str, func: F, quiet: bool) -> T
where
    L: Display,
    F: FnOnce() -> T,
{
    let id = ID.fetch_add(1, Ordering::SeqCst); // TODO: is this correct?
    log::logger().log(
        &Record::builder()
            .key_values(&[(constants::INCREMENT, ())])
            .build(),
    );
    log::logger().log(
        &Record::builder()
            .level(log::Level::Info)
            .key_values(&[
                (constants::ID, id),
                (constants::QUIET, if quiet { 1 } else { 0 }),
            ])
            .target(location)
            .args(format_args!("{label}"))
            .build(),
    );

    let now = std::time::Instant::now();

    log::logger().log(
        &Record::builder()
            .key_values(&[(constants::INCREMENT, ())])
            .build(),
    );
    let rv = func();
    log::logger().log(
        &Record::builder()
            .key_values(&[(constants::DECREMENT, ())])
            .build(),
    );

    let elapsed = now.elapsed().as_millis();
    log::logger().log(
        &Record::builder()
            .key_values(&[
                (constants::SET_TIME, ""),
                (constants::TIME, &elapsed.to_string()),
                (constants::ID, &id.to_string()),
            ])
            .build(),
    );
    log::logger().log(
        &Record::builder()
            .key_values(&[(constants::DECREMENT, ())])
            .build(),
    );
    rv
}

/// Utility macro that profiles code and nests the logging output.
///
/// ```no_run
/// use tree_logger::TreeLogger;
/// TreeLogger::new().with_colors(true).with_threads(true).init().unwrap();
/// log::warn!("This is an example message.");
/// ```
///
#[macro_export]
macro_rules! profile {
    ($label:expr, $func:expr) => {
        tree_logger::profile::profile_core($label, file!(), $func, false)
    };
}

#[macro_export]
macro_rules! profile_quiet {
    ($label:literal, $func:expr) => {
        tree_logger::profile::profile_core($label, file!(), $func, true)
    };
}
