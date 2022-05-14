//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::Uint8Array;
use wasm_bindgen_test::*;
use wasm_reed_solomon_erasure::encode;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn encode_test() {
    /*
    let data_shards = [
        js_sys::Uint8Array::from(&[72, 101]),
        js_sys::Uint8Array::from([108, 108]),
        js_sys::Uint8Array::from([111, 32]),
        js_sys::Uint8Array::from([87, 111]),
        js_sys::Uint8Array::from([114, 108]),
        js_sys::Uint8Array::from([100, 0]),
        js_sys::Uint8Array::from([0, 0]),
        js_sys::Uint8Array::from([0, 0]),
    ];
     */
    unsafe {
        let data_shards = [
            Uint8Array::new(&Uint8Array::view(&[72, 101])),
            Uint8Array::new(&Uint8Array::view(&[108, 108])),
            Uint8Array::new(&Uint8Array::view(&[111, 32])),
            Uint8Array::new(&Uint8Array::view(&[87, 111])),
            Uint8Array::new(&Uint8Array::view(&[114, 108])),
            Uint8Array::new(&Uint8Array::view(&[100, 0])),
            Uint8Array::new(&Uint8Array::view(&[0, 0])),
            Uint8Array::new(&Uint8Array::view(&[0, 0])),
        ];
        let vu8a: Vec<Uint8Array> = encode(data_shards.to_vec(), 2);
        let r: Vec<Vec<u8>> = vec![];
        let vu8av8: Vec<Vec<u8>> = vu8a.iter().map(|v| v.to_vec()).collect();
        assert_eq!(r, vu8av8);
    }
}
