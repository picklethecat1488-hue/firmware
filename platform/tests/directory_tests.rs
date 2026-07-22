use platform::directory::{add_to_directory, remove_from_directory, string_to_key, KEY_SIZE};

#[test]
fn test_string_to_key_boundaries() {
    // Empty string
    let key_empty = string_to_key("");
    assert_eq!(key_empty, [0u8; KEY_SIZE]);

    // Short string
    let key_short = string_to_key("test.txt");
    let mut expected_short = [0u8; KEY_SIZE];
    expected_short[..8].copy_from_slice(b"test.txt");
    assert_eq!(key_short, expected_short);

    // Exact KEY_SIZE string
    let exact_str = "abcdefghijklmnopqrstuvwxyz123456"; // 32 chars
    let key_exact = string_to_key(exact_str);
    assert_eq!(&key_exact[..], exact_str.as_bytes());

    // Overflow string
    let overflow_str = "abcdefghijklmnopqrstuvwxyz1234567890_extra";
    let key_overflow = string_to_key(overflow_str);
    assert_eq!(&key_overflow[..], &overflow_str.as_bytes()[..KEY_SIZE]);
}

#[test]
fn test_add_to_directory_empty() {
    let result = add_to_directory("", "file1.txt");
    assert!(result.is_some());
    let new_dir = result.unwrap();
    assert_eq!(new_dir.as_str(), "file1.txt");
}

#[test]
fn test_add_to_directory_existing() {
    let existing = "file1.txt\nfile2.txt\nfile3.txt";

    // Adding existing file (should do nothing and return None)
    let result_existing = add_to_directory(existing, "file2.txt");
    assert!(result_existing.is_none());

    // Adding new file (should append with newline)
    let result_new = add_to_directory(existing, "file4.txt");
    assert!(result_new.is_some());
    let new_dir = result_new.unwrap();
    assert_eq!(
        new_dir.as_str(),
        "file1.txt\nfile2.txt\nfile3.txt\nfile4.txt"
    );
}

#[test]
fn test_add_to_directory_with_trailing_newline() {
    let existing = "file1.txt\nfile2.txt\n";
    let result = add_to_directory(existing, "file3.txt");
    assert!(result.is_some());
    let new_dir = result.unwrap();
    assert_eq!(new_dir.as_str(), "file1.txt\nfile2.txt\nfile3.txt");
}

#[test]
fn test_remove_from_directory() {
    let existing = "file1.txt\nfile2.txt\nfile3.txt";

    // Remove middle entry
    let dir_after_remove = remove_from_directory(existing, "file2.txt");
    assert_eq!(dir_after_remove.as_str(), "file1.txt\nfile3.txt");

    // Remove first entry
    let dir_remove_first = remove_from_directory(existing, "file1.txt");
    assert_eq!(dir_remove_first.as_str(), "file2.txt\nfile3.txt");

    // Remove last entry
    let dir_remove_last = remove_from_directory(existing, "file3.txt");
    assert_eq!(dir_remove_last.as_str(), "file1.txt\nfile2.txt");

    // Remove non-existent entry (should return unchanged index)
    let dir_non_existent = remove_from_directory(existing, "nonexistent.txt");
    assert_eq!(dir_non_existent.as_str(), "file1.txt\nfile2.txt\nfile3.txt");
}

#[test]
fn test_remove_from_directory_single_entry() {
    let existing = "only_one.txt";
    let dir_after = remove_from_directory(existing, "only_one.txt");
    assert!(dir_after.is_empty());
}
