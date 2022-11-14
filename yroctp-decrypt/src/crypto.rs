use libaes::Cipher;
use std::fs;

pub fn init(path: &String) {
    println!("Decrypting file: {}", path);

    let key = b"77892KGGBQL4Bg8tRshSk8DDEppFuwz7"; // Yes, the keys have to be hard-coded.
    let iv = b"hUbnysDRJN8Mkf52";
    let file_content = fs::read(path).unwrap();

    let cipher = Cipher::new_256(key);

    let decrypted = cipher.cbc_decrypt(iv, &file_content);
    fs::write(path, decrypted).unwrap();

    println!("{}", path.clone());
    println!("{}", path.clone().replace(".cpyt", ""));
    fs::rename(path, path.clone().replace(".cpyt", "")).unwrap();
}
