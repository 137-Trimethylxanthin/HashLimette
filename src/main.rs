use base64::{Engine as _, engine::{self, general_purpose}, alphabet};


const DEFAULT_KEY:&str = "UGlzc2JvbWJlcjppbm5lbjMxNDE1OWdOdEZpRmdsQ05pTmVqRkZSZ2ZHRHZKSXVUQ3ZFTmJSZHVuR25F";

pub fn encrypt(variable: &str, key: Option<&str>) -> String {
    let mut result = String::new();
    let mut key_chars = key.unwrap_or(DEFAULT_KEY).chars().cycle();
    for c in variable.chars() {
        let key_char = key_chars.next().unwrap();
        let key_char_num = key_char.to_digit(36).unwrap();
        let c_num = c.to_digit(36).unwrap();
        let new_char_num = (c_num + key_char_num) % 36;
        let new_char = std::char::from_digit(new_char_num, 36).unwrap();
        result.push(new_char);
    }
    result
}


pub fn decrypt(variable: &str, key: Option<&str>) -> String {
    let mut result = String::new();
    let mut key_chars = key.unwrap_or(DEFAULT_KEY).chars().cycle();
    for c in variable.chars() {
        let key_char = key_chars.next().unwrap();
        let key_char_num = key_char.to_digit(36).unwrap();
        let c_num = c.to_digit(36).unwrap();
        let new_char_num = (c_num + 36 - key_char_num) % 36;
        let new_char = std::char::from_digit(new_char_num, 36).unwrap();
        result.push(new_char);
    }
    result
}

pub fn base64_encode(variable:&str) -> String {
    general_purpose::STANDARD.encode(variable)
}

pub fn base64_decode(variable:&str) -> String {
    general_purpose::STANDARD.decode(variable).unwrap().into_iter().map(|c| c as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let variable = "test";
        let key = None;
        assert_eq!(encrypt(variable, key), "dsqd");
    }

    #[test]
    fn test_decrypt() {
        let variable = "dsqd";
        let key = None;
        assert_eq!(decrypt(variable, key), "test");
    }

    #[test]
    fn test_base64_encode() {
        let variable = "test";
        assert_eq!(base64_encode(variable), "dGVzdA==");
    }

    #[test]
    fn test_base64_decode() {
        let variable = "dGVzdA==";
        assert_eq!(base64_decode(variable), "test");
    }
}

fn main(){
    println!("{}",encrypt("test", None));
    println!("{}",decrypt("nuds", None));
}