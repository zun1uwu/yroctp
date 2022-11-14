use std::{
    fs::{self, read_dir},
    path::Path,
    thread,
    time::Duration,
};

mod constants;
use constants::*;

mod crypto;

fn main() {
    println!("This program will decrypt and restore your files.\nDo not close this window under any circumstances.\n");

    let pathstring = format!(r#"C:\Users\{}\"#, whoami::username());
    scan_files(&pathstring);

    for letter in EXTERNAL_DRIVE_LETTERS {
        scan_external_drives(&format!(r#"{}:\"#, letter));
    }

    println!("All of your files are decrypted and restored. Have a nice day!\nThis window will close itself after 10 seconds.");
    thread::sleep(Duration::from_secs(10));
}

fn scan_files(pathstring: &String) {
    let path = Path::new(&pathstring);

    if path.exists() && !path.display().to_string().contains('.') {
        println!("{}", path.display());
        let read_dir = read_dir(path).unwrap();

        for entry in read_dir {
            let entrypath = entry.as_ref().unwrap().path();
            let entrystring = entrypath.display().to_string();

            if entrypath.is_dir() && !entrypath.is_symlink() {
                if entrystring.contains("AppData") {
                    scan_appdata(&entrystring);
                } else if entrystring.contains("Desktop")
                    || entrystring.contains("Documents")
                    || entrystring.contains("Downloads")
                    || entrystring.contains("Pictures")
                    || entrystring.contains("Videos")
                    || entrystring.contains("Music")
                {
                    scan_files(&entrystring);
                }
            } else if entrypath.is_file() && entrystring.ends_with(".cpyt") {
                crypto::init(&entrystring);
                println!("{}", &entrystring);
            }
        }
    }
}

fn scan_external_drives(dir: &String) {
    let path = Path::new(&dir);

    if path.exists() {
        println!("{}", path.display());
        let read_dir = fs::read_dir(path).unwrap();
        for entry in read_dir {
            let path = entry.as_ref().unwrap().path();
            let display = entry.as_ref().unwrap().path().display().to_string();
            if path.is_dir() {
                scan_external_drives(&display);
            } else if path.is_file() && path.ends_with(".cpyt") {
                crypto::init(&path.display().to_string());
            }
        }
    }
}

fn scan_appdata(dir: &String) {
    println!("{}", dir)
}
