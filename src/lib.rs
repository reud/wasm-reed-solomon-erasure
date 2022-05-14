#[macro_use(shards)]
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;

mod utils;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-reed-solomon-erasure!");
}

#[wasm_bindgen]
pub fn encode(data_shards: Vec<Uint8Array>, parity_shard: usize) -> Vec<Uint8Array> {
    let r = ReedSolomon::new(data_shards.len(), parity_shard).unwrap();
    let mut v: Vec<Vec<u8>> = vec![];
    let iter = data_shards.iter();
    let dd = iter.map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>();
    v.extend(dd);
    for _ in 0..parity_shard {
        let vv: Vec<u8> = vec![0; v[0].len()];
        v.push(vv);
    }
    let mut data = v.clone();
    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    r.encode(&mut data).unwrap();
    let mut shards: Vec<Vec<u8>> = data.clone();
    let mut ret: Vec<Uint8Array> = vec![];
    unsafe {
        for shard in shards {
            let u8a_shared = Uint8Array::new(&Uint8Array::view(&shard));
            ret.push(u8a_shared);
        }
        return ret;
    }
}
