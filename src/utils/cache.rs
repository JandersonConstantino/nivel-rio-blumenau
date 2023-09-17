use std::fs;

const TEMPFILE: &str = "/tmp/nivel-rio-blumenau";

pub fn save_cache_file(content: String) {
    let _ = fs::write(TEMPFILE, content);
}

pub fn cache_file_exists() -> bool {
    match fs::read(TEMPFILE) {
        Err(_) => false,
        Ok(_) => true,
    }
}

pub fn get_cache_file() -> String {
    if !cache_file_exists() {
        panic!("Temp file not exists!");
    }

    fs::read_to_string(TEMPFILE).unwrap()
}