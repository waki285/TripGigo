mod func;

use once_cell::sync::Lazy;
use std::panic;
use std::sync::{Arc, RwLock};

use crate::func::{randobet, get_trip};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    pub fn found(trip: &str, key: &str);
    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

static STOP_FLAG: Lazy<Arc<RwLock<bool>>> = Lazy::new(|| Arc::new(RwLock::new(false)));


/*
#[allow(deprecated)]
    let digest = pwhash::unix_crypt::hash_with("23",
    "12345678").unwrap();
 */

#[wasm_bindgen]
pub fn init_panic_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn trip_search(word: &str) {
    //    std::thread::spawn(|| {
    let mut i = 0;
    let mut key = String::new();
    loop {
        let h = STOP_FLAG.read().unwrap();
        if *h {
            break;
        }
        drop(h);

        if i == 0 {
            i = 1;
            key = randobet(2, Some("./"));
        }

        if i == 2000 {
            i = 1;
            key = randobet(2, Some("./"));
        }
        let key_1 = randobet(5, Some("./"));
        let key_2 = randobet(1, Some("./"));

        let final_key = format!("{}{}{}", key_2, key, key_1);

        let salt = &final_key[1..3];

        let trip = get_trip(salt, &final_key);

        let trip = &trip[trip.len() - 10..];
        
        if trip.starts_with(word) {
            found(trip, &final_key);
            break;
        }
        log(&format!("{} {}", trip, final_key));

        i += 1;
    }
    //    });
}

#[wasm_bindgen]
pub fn stop() {
    let mut h = STOP_FLAG.write().unwrap();
    *h = true;
}
