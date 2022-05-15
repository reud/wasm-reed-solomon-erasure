extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;

mod utils;

use js_sys::{Uint32Array, Uint8Array};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn encode(data_shards: Vec<Uint8Array>, parity_shard: usize) -> Vec<Uint8Array> {
    let r = ReedSolomon::new(data_shards.len(), parity_shard).unwrap();
    let mut v = data_shards
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<u8>>>();
    for _ in 0..parity_shard {
        let vv: Vec<u8> = vec![0; v[0].len()];
        v.push(vv);
    }
    let mut data = v.clone();
    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    r.encode(&mut data).unwrap();
    let shards: Vec<Vec<u8>> = data.clone();
    unsafe {
        return shards
            .iter()
            .map(|shard| Uint8Array::new(&Uint8Array::view(&shard)))
            .collect();
    }
}

#[wasm_bindgen]
pub fn reconstruct(
    corrupted_shards: Vec<Uint8Array>,
    parity_shard: usize,
    dead_shard_indexes: Uint32Array,
) -> Vec<Uint8Array> {
    let mut corrupted_shards_usable: Vec<Option<Vec<u8>>> = corrupted_shards
        .iter()
        .map(|v| v.to_vec())
        .map(Some)
        .collect();
    for idx in dead_shard_indexes.to_vec() {
        corrupted_shards_usable[idx as usize] = None;
    }

    let data_shards_cnt = corrupted_shards_usable.len() - parity_shard;
    let r = ReedSolomon::new(data_shards_cnt, parity_shard).unwrap();

    r.reconstruct(&mut *corrupted_shards_usable).unwrap();

    unsafe {
        return corrupted_shards_usable
            .iter()
            .map(|shard| Uint8Array::new(&Uint8Array::view(&(shard.clone().unwrap()))))
            .collect();
    }
}
