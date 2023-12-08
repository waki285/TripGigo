use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

/*
#[allow(deprecated)]
    let digest = pwhash::unix_crypt::hash_with("23",
    "12345678").unwrap();
 */

#[wasm_bindgen]
pub fn trip_search() {
    println!("Hello, world!");
    #[allow(deprecated)]
    let digest = pwhash::unix_crypt::hash_with("23",
    "12345678").unwrap();
    alert(&format!("digest: {}", digest));
}
