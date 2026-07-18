use embassy_time::{Duration, Timer};
use firmware_lib::{select_branch_with_timeout, with_timeout};

async fn test_with_timeout_impl() {
    // Test future finishes before timeout
    let f1 = async { 123 };
    let res1 = with_timeout!(f1, Duration::from_millis(50)).await;
    assert_eq!(res1, Some(123));

    // Test timeout triggers first
    let f2 = Timer::after(Duration::from_millis(100));
    let res2 = with_timeout!(f2, Duration::from_millis(5)).await;
    assert_eq!(res2, None);
}

async fn test_select_branch_with_timeout_impl() {
    // 1. Test timeout path (no branch completes)
    let f1 = Timer::after(Duration::from_millis(100));
    let f2 = Timer::after(Duration::from_millis(100));
    let res = select_branch_with_timeout!(
        Duration::from_millis(5),
        f1 => || Some("f1"),
        f2 => || Some("f2"),
    );
    assert_eq!(res, None);

    // 2. Test branch completion path
    let f1 = async { Some("completed") };
    let f2 = Timer::after(Duration::from_millis(100));
    let res2 = select_branch_with_timeout!(
        Duration::from_millis(50),
        f1 => |v| v,
        f2 => || None,
    );
    assert_eq!(res2, Some("completed"));
}

#[embassy_executor::task]
async fn run_all_tests_task(tx: std::sync::mpsc::Sender<()>) {
    test_with_timeout_impl().await;
    test_select_branch_with_timeout_impl().await;
    let _ = tx.send(());
}

#[test]
fn test_all_select_macros() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
        executor.run(|spawner| {
            spawner.spawn(run_all_tests_task(tx)).unwrap();
        });
    });
    rx.recv().unwrap();
}
