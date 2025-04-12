use log::{info, warn};
use std::{thread, time::Duration};
use tree_logger::{TreeLogger, profile};

fn main() {
    TreeLogger::new()
        .with_colors(true)
        .with_threads(true)
        .init()
        .unwrap();

    warn!("Basic warning, not nested, shows up immediately");
    let result = profile!("First span", || {
        info!("Info inside a span. Doesn't print until the whole span is done");

        thread::sleep(Duration::from_secs(2));

        profile!("Child span #1", || {
            info!("Info inside a child span #1");
            thread::sleep(Duration::from_secs(1));
        });

        profile!("Child span #2", || {
            info!("Info inside a child span #2");
            thread::sleep(Duration::from_secs(1));
        });

        42 // Optionally we can return a value
    });

    info!("Profile returns a value: {result}");
}
