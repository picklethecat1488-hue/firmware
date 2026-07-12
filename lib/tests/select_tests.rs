use embassy_time::{Duration, Timer};
use firmware_lib::{select_branch, select_branch_with_timeout, with_timeout};

// Macro to parameterize testing of 2-way select_branch permutations.
macro_rules! test_2_way_permutation {
    ($fut1:expr, $fut2:expr, ||, ||, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => || 1,
                $fut2 => || 2,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, ||, |$v2:ident|, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => || 1,
                $fut2 => |$v2| 2,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, |$v1:ident|, ||, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => |$v1| 1,
                $fut2 => || 2,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, |$v1:ident|, |$v2:ident|, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => |$v1| 1,
                $fut2 => |$v2| 2,
            );
            assert_eq!(result, $expected_idx);
        }
    };
}

// Macro to parameterize testing of 3-way select_branch permutations.
macro_rules! test_3_way_permutation {
    ($fut1:expr, $fut2:expr, $fut3:expr, ||, ||, ||, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => || 1,
                $fut2 => || 2,
                $fut3 => || 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, ||, ||, |$v3:ident|, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => || 1,
                $fut2 => || 2,
                $fut3 => |$v3| 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, ||, |$v2:ident|, ||, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => || 1,
                $fut2 => |$v2| 2,
                $fut3 => || 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, ||, |$v2:ident|, |$v3:ident|, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => || 1,
                $fut2 => |$v2| 2,
                $fut3 => |$v3| 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, |$v1:ident|, ||, ||, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => |$v1| 1,
                $fut2 => || 2,
                $fut3 => || 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, |$v1:ident|, ||, |$v3:ident|, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => |$v1| 1,
                $fut2 => || 2,
                $fut3 => |$v3| 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, |$v1:ident|, |$v2:ident|, ||, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => |$v1| 1,
                $fut2 => |$v2| 2,
                $fut3 => || 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
    ($fut1:expr, $fut2:expr, $fut3:expr, |$v1:ident|, |$v2:ident|, |$v3:ident|, $expected_idx:expr) => {
        {
            let result = select_branch!(
                $fut1 => |$v1| 1,
                $fut2 => |$v2| 2,
                $fut3 => |$v3| 3,
            );
            assert_eq!(result, $expected_idx);
        }
    };
}

async fn test_select_branch_2_way_closures_impl() {
    // 1. Permutation: || and ||
    test_2_way_permutation!(
        Timer::after(Duration::from_millis(5)),
        Timer::after(Duration::from_millis(50)),
        ||,
        ||,
        1
    );
    test_2_way_permutation!(
        Timer::after(Duration::from_millis(50)),
        Timer::after(Duration::from_millis(5)),
        ||,
        ||,
        2
    );

    // 2. Permutation: || and |val|
    test_2_way_permutation!(
        Timer::after(Duration::from_millis(5)),
        async { 42 },
        ||,
        |v|,
        2 // async completes immediately, so 2
    );
    test_2_way_permutation!(
        Timer::after(Duration::from_millis(5)),
        async {
            Timer::after(Duration::from_millis(50)).await;
            42
        },
        ||,
        |v|,
        1 // timer completes first, so 1
    );

    // 3. Permutation: |val| and ||
    test_2_way_permutation!(
        async { 100 },
        Timer::after(Duration::from_millis(50)),
        |v|,
        ||,
        1
    );

    // 4. Permutation: |val| and |val|
    test_2_way_permutation!(
        async { 100 },
        async { 200 },
        |v|,
        |v|,
        1
    );
}

async fn test_select_branch_3_way_closures_impl() {
    let slow = || Timer::after(Duration::from_millis(100));
    let fast = || Timer::after(Duration::from_millis(5));
    let imm = || async { 42 };

    // 1. Permutation: || and || and ||
    test_3_way_permutation!(fast(), slow(), slow(), ||, ||, ||, 1);
    test_3_way_permutation!(slow(), fast(), slow(), ||, ||, ||, 2);
    test_3_way_permutation!(slow(), slow(), fast(), ||, ||, ||, 3);

    // 2. Permutation: || and || and |val|
    test_3_way_permutation!(slow(), slow(), imm(), ||, ||, |v|, 3);

    // 3. Permutation: || and |val| and ||
    test_3_way_permutation!(slow(), imm(), slow(), ||, |v|, ||, 2);

    // 4. Permutation: || and |val| and |val|
    test_3_way_permutation!(slow(), imm(), imm(), ||, |v|, |v|, 2);

    // 5. Permutation: |val| and || and ||
    test_3_way_permutation!(imm(), slow(), slow(), |v|, ||, ||, 1);

    // 6. Permutation: |val| and || and |val|
    test_3_way_permutation!(imm(), slow(), imm(), |v|, ||, |v|, 1);

    // 7. Permutation: |val| and |val| and ||
    test_3_way_permutation!(imm(), imm(), slow(), |v|, |v|, ||, 1);

    // 8. Permutation: |val| and |val| and |val|
    test_3_way_permutation!(imm(), imm(), imm(), |v|, |v|, |v|, 1);
}

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
    test_select_branch_2_way_closures_impl().await;
    test_select_branch_3_way_closures_impl().await;
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
