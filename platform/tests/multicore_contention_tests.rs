use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

struct Counter {
    val: u32,
}

#[test]
fn test_multicore_contention_raw_mutex() {
    let counter = Arc::new(Mutex::<CriticalSectionRawMutex, RefCell<Counter>>::new(
        RefCell::new(Counter { val: 0 }),
    ));
    let mut handles = vec![];

    // Spawn 10 threads simulating concurrent core/interrupt executions
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.lock(|cell| {
                    let mut guard = cell.borrow_mut();
                    let current = guard.val;
                    // Yield to maximize scheduler context switches and potential race conditions
                    thread::yield_now();
                    guard.val = current + 1;
                });
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_val = counter.lock(|cell| cell.borrow().val);
    assert_eq!(final_val, 1000);
}
