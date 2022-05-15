//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::Uint32Array;
use js_sys::Uint8Array;
use wasm_bindgen_test::*;
use wasm_reed_solomon_erasure::{encode, reconstruct};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn encode_test() {
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
        let encoded: Vec<Uint8Array> = encode(data_shards.to_vec(), 2);
        let mut corrupted: Vec<Uint8Array> = encoded.clone();
        corrupted[1] = Uint8Array::new(&Uint8Array::view(&[0, 0]));

        let dead_shard_indexes = Uint32Array::new(&Uint32Array::view(&[1]));
        let restored: Vec<Uint8Array> = reconstruct(corrupted.clone(), 2, dead_shard_indexes);
        let vrestored: Vec<Vec<u8>> = restored.iter().map(|v| v.to_vec()).collect();
        let encoded: Vec<Vec<u8>> = encoded.iter().map(|v| v.to_vec()).collect();
        assert_eq!(encoded, vrestored);
    }
}
