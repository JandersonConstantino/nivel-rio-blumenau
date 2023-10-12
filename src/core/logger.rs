pub struct Logger;

impl Logger {
    pub fn print(value: &str) {
        println!("{}", value);
    }

    pub fn panic(value: &str) {
        panic!("{}", value);
    }
}
