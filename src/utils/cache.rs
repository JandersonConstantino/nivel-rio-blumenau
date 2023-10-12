use std::fs;

use crate::core::Logger;

const TEMPFILE: &str = "/tmp/nivel-rio-blumenau";

pub struct Cache;

impl Cache {
    pub fn save(content: String) {
        let _ = fs::write(TEMPFILE, content);
    }

    pub fn exists() -> bool {
        fs::read(TEMPFILE).is_ok()
    }

    pub fn get() -> String {
        if !Cache::exists() {
            Logger::panic("Temp file not exists!");
        }

        fs::read_to_string(TEMPFILE).unwrap()
    }
}
