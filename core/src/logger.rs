use tracing::level_filters::LevelFilter;

// Max Logger Level on debug build
#[cfg(debug_assertions)]
const MAX_LEVEL: LevelFilter = LevelFilter::DEBUG;

// Max Logger Level on release build
#[cfg(not(debug_assertions))]
const MAX_LEVEL: LevelFilter = LevelFilter::INFO;

pub fn init() {
    // initialize better_panic
    better_panic::install();

    // initialize tracing
    tracing_subscriber::fmt().with_max_level(MAX_LEVEL).init();
}
