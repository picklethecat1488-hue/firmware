use firmware_lib::log_info;
use firmware_lib::panic_handler::CRASH_LOG_BUFFER;

#[test]
fn test_crash_log_buffer_writing_and_wrapping() {
    log_info!("Event A");
    log_info!("Event B");

    critical_section::with(|cs| {
        let buffer = CRASH_LOG_BUFFER.borrow(cs).borrow();
        let end = buffer.head;
        let logged_str = core::str::from_utf8(&buffer.buffer[..end]).unwrap();
        assert!(logged_str.contains("Event A"));
        assert!(logged_str.contains("Event B"));
    });
}

#[test]
fn test_log_info_module_prefixing() {
    critical_section::with(|cs| {
        let mut buffer = CRASH_LOG_BUFFER.borrow(cs).borrow_mut();
        buffer.head = 0;
        buffer.wrapped = false;
    });

    log_info!("Test prefix");

    critical_section::with(|cs| {
        let buffer = CRASH_LOG_BUFFER.borrow(cs).borrow();
        let end = buffer.head;
        let logged_str = core::str::from_utf8(&buffer.buffer[..end]).unwrap();
        // Since we are running in an integration test now, the module path prefix will be
        // "[panic_handler_tests]" instead of "[firmware_lib::panic_handler::tests]"!
        assert!(logged_str.contains("[panic_handler_tests] Test prefix"));
    });
}
