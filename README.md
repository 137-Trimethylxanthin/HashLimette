# HashLimette

## was a:


Simple bad coded Password Hasher in go i wrote.
Its my first programm in go but i made the wrong thing.
i wantet somthing to store password and show them again but i made idk how a password hasher.
i think its really bad coded so feel free to write feedback.


## now is:

a cargo package for rust to encrypt your code so if you decompile it no strings will be shown.

if you download the binary to encrypt your strings beforhand.

## how to use the binary

```bash
./hashlimette -e value -k key # encrypt , key is optional
./hashlimette -d value -k key # decrypt, key is optional
./hashlimette -bd value # base64 decode
./hashlimette -be value # base64 encode
./hashlimette -h # help
```
use the binary to first encrypt your strings and then in your code decrypt them with the function decrypt().

## how to use it in your code

```rust
use hashlimette::decrypt;
const key: &str = "your key"; // if you dont use a key in the binary you dont need to use one here the default key wil be used
fn main() {
    let encrypted = "your encrypted string";
    let decrypted = decrypt(encrypted, Some(key)));
    println!("{}", decrypted);

    // if no key is used:
    let decrypted = decrypt(encrypted, None);
}
```