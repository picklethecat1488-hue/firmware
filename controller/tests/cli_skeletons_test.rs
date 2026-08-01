//! Verifies that all generated CLI handler skeletons compile cleanly.
#![allow(unused_variables, dead_code, clippy::match_single_binding)]

include!(concat!(env!("OUT_DIR"), "/cli_skeletons_test.rs"));
