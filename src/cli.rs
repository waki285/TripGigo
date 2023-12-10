mod func;

use std::{io::{stdout, Write}, sync::{Arc, RwLock}};

use crate::func::{get_trip, randobet};

fn main() {
    println!("Hello!");
    print!("What is your word do you want? > ");
    stdout().flush().unwrap();
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    if word.len() > 10 {
        println!("Word is too long!");
        return;
    }
    let mut threads = 1;
    print!("How many threads do you want? > ");
    stdout().flush().unwrap();
    let mut threads_str = String::new();
    std::io::stdin().read_line(&mut threads_str).unwrap();
    let threads_str = threads_str.trim();
    if threads_str.len() > 0 {
        threads = threads_str.parse::<usize>().unwrap();
    }

    let start_time = std::time::Instant::now();

    let stop_flag = Arc::new(RwLock::new(false));

    for t in 0..threads {
        let word = word.clone();
        let word = word.trim();
        let word = word.to_string();
        let stop_flag = Arc::clone(&stop_flag);
        std::thread::spawn(move || {
            let mut i: usize = 0;
            let mut key = String::new();
            loop {
                let h = stop_flag.read().unwrap();
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

                if trip.starts_with(&word) {
                    let mut h = stop_flag.write().unwrap();
                    *h = true;
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    println!("Found!");
                    println!("Trip: {}", trip);
                    println!("Key: #{}00", final_key);
                    println!("Time: {}ms", start_time.elapsed().as_millis());
                    std::process::exit(0);
                }
                println!(
                    "{}",
                    format!("[{}] Tested {}. But failed with {}", t, final_key, trip)
                );

                i += 1;
            }
        });
    }
    loop {}
}
