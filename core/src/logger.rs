pub fn init() {
    // initialize better_panic
    better_panic::install();

    // initialize tracing
    tracing_subscriber::fmt()
        .init();
}
