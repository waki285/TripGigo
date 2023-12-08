//use wasm_bindgen::prelude::*;
use des::{Des, cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray}};

//#[wasm_bindgen]
pub fn main() {
    println!("Hello, world!");
    //println!("do something");
    let key = GenericArray::from_slice("23".as_bytes());
    let cipher = Des::new(key);
    let mut bytes: Vec<_> = "12345678"
        .to_string()
        .into_bytes();
    let mut u8s: Vec<GenericArray<u8, _>> = bytes
        .chunks_mut(16)
        .map(|f| GenericArray::from_slice(f).clone())
        .collect();
    cipher.encrypt_blocks(&mut u8s);
    dbg!(u8s.to_vec());
}
