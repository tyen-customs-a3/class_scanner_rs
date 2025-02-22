use std::sync::Once;
use env_logger::{Builder, Target};
use log::LevelFilter;

static INIT: Once = Once::new();

pub fn setup_test_logging() {
    INIT.call_once(|| {
        let mut builder = Builder::new();
        builder.filter_level(LevelFilter::Debug)
               .target(Target::Stdout)
               .format_timestamp(None)
               .format_module_path(true)
               .init();
    });
}