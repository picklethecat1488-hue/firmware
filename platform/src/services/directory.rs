//! Shared directory index and key management utilities.
//!
//! Exposes helper functions to parse, add to, and remove files from the
//! persistent directory index file (`.dir`).

/// The fixed size key length in bytes for filename mapping.
pub const KEY_SIZE: usize = 32;

/// The maximum size of the directory index listing buffer and string in bytes.
pub const DIR_BUF_SIZE: usize = 2048;

/// Hashes a string filename into a fixed 32-byte key.
pub fn string_to_key(s: &str) -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    let bytes = s.as_bytes();
    let len = bytes.len().min(KEY_SIZE);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

/// Helper to parse a newline-separated directory listing, check if a file exists,
/// and append it if not present.
///
/// Returns `Some(new_dir)` if a change was made and the directory needs to be stored,
/// or `None` if the file was already in the directory.
pub fn add_to_directory(
    existing_dir: &str,
    filename: &str,
) -> Option<heapless::String<DIR_BUF_SIZE>> {
    let mut found = false;
    for entry in existing_dir.split('\n') {
        if entry == filename {
            found = true;
            break;
        }
    }

    if !found {
        let mut new_dir = heapless::String::<DIR_BUF_SIZE>::new();
        let _ = new_dir.push_str(existing_dir);
        if !new_dir.is_empty() && !new_dir.ends_with('\n') {
            let _ = new_dir.push('\n');
        }
        let _ = new_dir.push_str(filename);
        Some(new_dir)
    } else {
        None
    }
}

/// Helper to remove a file from a newline-separated directory listing.
///
/// Returns the updated directory listing.
pub fn remove_from_directory(existing_dir: &str, filename: &str) -> heapless::String<DIR_BUF_SIZE> {
    let mut new_dir = heapless::String::<DIR_BUF_SIZE>::new();
    for entry in existing_dir.split('\n') {
        if entry.is_empty() || entry == filename {
            continue;
        }
        if !new_dir.is_empty() {
            let _ = new_dir.push('\n');
        }
        let _ = new_dir.push_str(entry);
    }
    new_dir
}
