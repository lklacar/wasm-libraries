extern crate console_error_panic_hook;

use std::io;
use std::io::prelude::*;

use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_random_array(length: u32, seed: u64) -> js_sys::Array {
    let mut random_generator: Hc128Rng = Hc128Rng::seed_from_u64(seed);
    let result = js_sys::Array::new_with_length(length);
    (0..length).for_each(|i| result.set(i, JsValue::from(random_generator.next_u32())));
    return result;
}


#[wasm_bindgen]
pub fn compress(mut buf: &[u8]) -> js_sys::Array {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(buf);
    let compressed_bytes = encoder.finish().unwrap();

    let result = js_sys::Array::new_with_length(compressed_bytes.len() as u32);
    (0..compressed_bytes.len()).for_each(|i| result.set(i as u32, JsValue::from(compressed_bytes[i])));

    return result;
}

#[wasm_bindgen]
pub fn decompress(mut buf: &[u8], size: usize) -> js_sys::Array {
    set_panic_hook();
    let mut d = ZlibDecoder::new(buf);
    let mut buffer_result = vec![0; size];
    d.read(&mut buffer_result).unwrap();

    let result = js_sys::Array::new_with_length(buffer_result.len() as u32);
    (0..buffer_result.len()).for_each(|i| result.set(i as u32, JsValue::from(buffer_result[i])));

    return result;
}

#[wasm_bindgen]
pub fn encrypt(mut buf: &[u8], mut key: &[u8]) -> js_sys::Array {
    set_panic_hook();
    let mut res = vec![0 as u8; buf.len()];

    for i in 0..buf.len() {
        res[i] = buf[i] ^ key[i];
    }

    let result = js_sys::Array::new_with_length(res.len() as u32);
    (0..res.len()).for_each(|i| result.set(i as u32, JsValue::from(res[i])));

    return result;
}
