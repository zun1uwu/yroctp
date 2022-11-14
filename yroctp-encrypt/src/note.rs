use std::{fs::File, io::Write, thread, time::Duration};

pub fn init(note: &str, path: &String) {
    let mut create_file = File::create(path).unwrap();
    create_file.write_all(note.as_bytes()).unwrap();
    thread::sleep(Duration::from_secs(2));
}
