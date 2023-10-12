use crate::core::Logger;
use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn delete_temp_file() {
        let _ = fs::remove_file(TEMPFILE);
    }

    #[test]
    fn exists_should_return_false_when_cache_file_not_exists() {
        delete_temp_file();
        assert_eq!(Cache::exists(), false);
    }

    #[test]
    fn exists_should_return_true_when_cache_file_exists() {
        let _ = fs::write(TEMPFILE, "mock-content");
        assert_eq!(Cache::exists(), true);

        delete_temp_file();
    }

    #[test]
    #[should_panic]
    fn get_should_panic_when_file_not_exists() {
        delete_temp_file();
        let _ = Cache::get();
    }

    #[test]
    fn get_should_return_file_content_as_string() {
        let mocked_content = String::from("My mock content of tempfile");
        delete_temp_file();

        let _ = fs::write(TEMPFILE, &mocked_content);

        assert_eq!(Cache::get(), mocked_content);
    }

    #[test]
    fn get_save_content() {
        let mocked_content = String::from("My mock content of tempfile");
        delete_temp_file();

        Cache::save(mocked_content.clone());

        assert_eq!(Cache::get(), mocked_content);
    }
}
