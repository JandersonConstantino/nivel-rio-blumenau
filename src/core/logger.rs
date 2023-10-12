#[cfg(test)]
use mockall::automock;

pub struct Logger;

#[cfg_attr(test, automock)]
impl Logger {
    pub fn print(value: &str) {
        println!("{}", value);
    }

    pub fn panic(value: &str) {
        panic!("{}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "foo")]
    fn should_panic_when_logger_panic_called() {
        Logger::panic("foo");
    }

    #[test]
    fn should_not_panic_when_logger_print_is_called() {
        Logger::print("foo");
    }
}
