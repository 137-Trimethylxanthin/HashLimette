#[allow(non_snake_case)]
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};


const DEFAULT_KEY:&str = "UGlzc2JvbWJlcjppbm5lbjMxNDE1OWdOdEZpRmdsQ05pTmVqRkZSZ2ZHRHZKSXVUQ3ZFTmJSZHVuR25F";

pub fn encrypt(variable:&str, key:Option<&str>) -> String {
    let key = key.unwrap_or(DEFAULT_KEY);
    let mut encrypted = String::new();
    for (i, c) in variable.chars().enumerate() {
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let encrypted_char = (c as u8 ^ key_char as u8) as char;
        encrypted.push(encrypted_char);
    }
    base64_encode(&encrypted)
}

pub fn decrypt(variable:&str, key:Option<&str>) -> String {
    let key = key.unwrap_or(DEFAULT_KEY);
    let encrypted = base64_decode(variable);
    let mut decrypted = String::new();
    for (i, c) in encrypted.chars().enumerate() {
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let decrypted_char = (c as u8 ^ key_char as u8) as char;
        decrypted.push(decrypted_char);
    }
    decrypted
}

pub fn base64_encode(variable:&str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(variable)
}

pub fn base64_decode(variable:&str) -> String {
    general_purpose::STANDARD_NO_PAD.decode(variable.as_bytes()).ok().map(|v| v.into_iter().map(|c| c as char).collect()).unwrap_or_else(|| String::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encrypt_decrypt() {
        let variable = "Hello, world!";
        let key = "secret";
        let encrypted = encrypt(variable, Some(key));
        let decrypted = decrypt(&encrypted, Some(key));
        assert_eq!(variable, decrypted);
    }

    #[test]
    fn test_encrypt_different_chars() {
        let variable = "!@#$%^&*()_+=/.,<>?;:'\"[]{}\\|`~";
        let key = "secret";
        let encrypted = encrypt(variable, Some(key));
        let decrypted = decrypt(&encrypted, Some(key));
        assert_eq!(variable, decrypted);
    }

    #[test]
    fn test_base64_encode_decode() {
        let variable = "Hello, world!";
        let encoded = base64_encode(variable);
        let decoded = base64_decode(&encoded);
        assert_eq!(variable, decoded);
    }

    #[test]
    fn test_base64_encode_decode_different_chars() {
        let variable = "!@#$%^&*()_+=/.,<>?;:'\"[]{}\\|`~";
        let encoded = base64_encode(variable);
        let decoded = base64_decode(&encoded);
        assert_eq!(variable, decoded);
    }

}

