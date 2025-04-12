use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

const SPECIAL_CHARS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?/";
const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";

pub fn generate_random_password() -> String {
    let mut rng = thread_rng();
    
    let mut password = String::with_capacity(20);
    
    password.push(SPECIAL_CHARS.chars().nth(rng.gen_range(0..SPECIAL_CHARS.len())).unwrap());
    password.push(LOWERCASE_CHARS.chars().nth(rng.gen_range(0..LOWERCASE_CHARS.len())).unwrap());
    password.push(UPPERCASE_CHARS.chars().nth(rng.gen_range(0..UPPERCASE_CHARS.len())).unwrap());
    password.push(NUMBERS.chars().nth(rng.gen_range(0..NUMBERS.len())).unwrap());
    
    let all_chars = format!("{}{}{}{}", SPECIAL_CHARS, LOWERCASE_CHARS, UPPERCASE_CHARS, NUMBERS);
    
    for _ in 0..16 {
        let idx = rng.gen_range(0..all_chars.len());
        password.push(all_chars.chars().nth(idx).unwrap());
    }
    
    let mut password_chars: Vec<char> = password.chars().collect();
    password_chars.shuffle(&mut rng);
    
    password_chars.into_iter().collect()
}

pub fn is_password_valid(password: &str) -> bool {
    if password.len() < 15 {
        return false;
    }
    
    let has_lowercase = password.chars().any(|c| LOWERCASE_CHARS.contains(c));
    let has_uppercase = password.chars().any(|c| UPPERCASE_CHARS.contains(c));
    let has_number = password.chars().any(|c| NUMBERS.contains(c));
    let has_special = password.chars().any(|c| SPECIAL_CHARS.contains(c));
    
    has_lowercase && has_uppercase && has_number && has_special
}