use tracechain::demo::run_demo;

/// This test simply runs the demo to ensure it executes without panics.
/// You can extend it to capture stdout and check expected values.
#[test]
fn test_demo_runs() {
    run_demo();
}
