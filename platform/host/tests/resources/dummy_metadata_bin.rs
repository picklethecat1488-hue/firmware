//! Dummy host binary test fixture embedding `PROJECT_METADATA` and `PROJECT_METADATA_ELF` symbols.

#[used]
#[no_mangle]
pub static PROJECT_METADATA: [u8; include_bytes!(env!("TEST_PROJECT_METADATA_PATH")).len()] =
    *include_bytes!(env!("TEST_PROJECT_METADATA_PATH"));

#[used]
#[no_mangle]
pub static PROJECT_METADATA_ELF: [u8; include_bytes!(env!("TEST_PROJECT_METADATA_ELF_PATH"))
    .len()] = *include_bytes!(env!("TEST_PROJECT_METADATA_ELF_PATH"));

fn main() {}
