use rand::{thread_rng, seq::SliceRandom};

pub fn randobet(len: usize, additional_chars: Option<&str>) -> String {
    let mut rng = thread_rng();
    let chars: Vec<char> = format!(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}",
        additional_chars.unwrap_or("")
    )
    .chars()
    .collect();
    let random_string: String = (0..len).map(|_| *chars.choose(&mut rng).unwrap()).collect();
    random_string
}

pub fn get_trip(salt: &str, pass: &str) -> String {
    #[allow(deprecated)]
    pwhash::unix_crypt::hash_with(salt, pass).unwrap()
}
