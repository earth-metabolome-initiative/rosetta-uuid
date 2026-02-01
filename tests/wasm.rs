//! Tests for Wasm target
#![cfg(target_arch = "wasm32")]

use rosetta_uuid::Uuid;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_v4_generation() {
    let uuid = Uuid::new_v4();
    assert_eq!(uuid.get_version(), Some(uuid::Version::Random));
}

#[wasm_bindgen_test]
fn test_v7_generation() {
    let uuid = Uuid::utc_v7();
    // V7 is time-based, so not nil
    assert_eq!(uuid.get_version(), Some(uuid::Version::SortRand));
}
