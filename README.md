# HashLimette.


## now is:

a cargo package for rust to encrypt your code so if you decompile it no strings will be shown.

if you download the binary to encrypt your strings beforhand.

## how to use the binary

the binary can be downloaded via cargo
cargo install hash_limette

```bash
hash_limette -e value -k key # encrypt , key is optional
hash_limette -d value -k key # decrypt, key is optional
hash_limette -bd value # base64 decode
hash_limette -be value # base64 encode
hash_limette -h # help
```
use the binary to first encrypt your strings and then in your code decrypt them with the function decrypt().

## how to use it in your code

first of all you need your dependencies:
cargo add hash_limette

```toml
[dependencies]
hash_limette = "0.1.4"
```

then you can use it like this:
```rust
use hash_limette::decrypt;
const key: &str = "your key"; // if you dont use a key in the binary you dont need to use one here the default key wil be used
fn main() {
    let encrypted = "your encrypted string";
    let decrypted = decrypt(encrypted, Some(key)));
    println!("{}", decrypted);

    // if no key is used:
    let decrypted = decrypt(encrypted, None);
}
```
