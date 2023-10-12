#[mockall_double::double]
use crate::core::Logger;
use crate::services::NivelItem;
use chrono::{Duration, NaiveDateTime};

use std::ops::Sub;

pub struct DisplayRiverInfo;

impl DisplayRiverInfo {
    pub fn display_river_info(item: &NivelItem, last_level: &Option<f32>) {
        Logger::print(&format!(
            "{} - {} metros {}",
            DisplayRiverInfo::to_readable_date(item.datetime.clone()),
            DisplayRiverInfo::format_level(item.level),
            DisplayRiverInfo::get_variation(last_level, &item.level)
        ));
    }

    fn format_level(level: f32) -> String {
        format!("{number:.prec$}", number = level, prec = 2)
    }

    fn get_variation(last_level: &Option<f32>, current_level: &f32) -> String {
        if last_level.is_none() {
            return String::from("");
        }

        if last_level.unwrap() == 0f32 {
            return String::from("");
        }

        let variation = current_level.sub(last_level.unwrap());

        if variation == 0f32 {
            return String::from("");
        }

        if variation > 0f32 {
            return format!("(+{number:.prec$}m)", number = variation, prec = 2);
        }

        format!("({number:.prec$}m)", number = variation, prec = 2)
    }

    fn to_readable_date(date: String) -> String {
        match NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%dT%H:%M:%SZ") {
            Ok(value) => (value - Duration::hours(3))
                .format("%d/%m/%Y %H:%M:%S")
                .to_string(),
            Err(_) => date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_variation_tests {
        use super::*;

        #[test]
        fn should_return_empty_string_when_last_level_is_none() {
            assert_eq!(
                DisplayRiverInfo::get_variation(&None, &1f32),
                "".to_string()
            );
        }

        #[test]
        fn should_return_empty_string_when_last_level_is_zero() {
            assert_eq!(
                DisplayRiverInfo::get_variation(&Some(0f32), &1f32),
                "".to_string()
            );
        }

        #[test]
        fn should_return_empty_string_when_variation_is_zero() {
            let current_level = &1f32;
            let last_level = &Some(current_level.clone());

            assert_eq!(
                DisplayRiverInfo::get_variation(last_level, current_level),
                "".to_string()
            );
        }

        #[test]
        fn should_return_formatted_string_when_variation_is_positive() {
            let last_level = &Some(9f32);
            let current_level = &10f32;

            assert_eq!(
                DisplayRiverInfo::get_variation(last_level, current_level),
                "(+1.00m)".to_string()
            );
        }

        #[test]
        fn should_return_formatted_string_when_variation_is_negative() {
            let last_level = &Some(7f32);
            let current_level = &3f32;

            assert_eq!(
                DisplayRiverInfo::get_variation(last_level, current_level),
                "(-4.00m)".to_string()
            );
        }
    }

    mod format_level {
        use super::*;

        #[test]
        fn should_keep_float_precision_in_two() {
            assert_eq!(DisplayRiverInfo::format_level(3.552f32), "3.55");
        }

        #[test]
        fn should_keep_float_precision_in_two_case_2() {
            assert_eq!(DisplayRiverInfo::format_level(2.555f32), "2.56");
        }

        #[test]
        fn should_keep_float_precision_in_two_case_3() {
            assert_eq!(DisplayRiverInfo::format_level(1f32), "1.00");
        }
    }

    mod to_readable_date {
        use super::*;

        #[test]
        fn should_return_not_formatted_value_when_date_is_in_unexpected_format() {
            assert_eq!(
                DisplayRiverInfo::to_readable_date("16/11/2023".to_string()),
                "16/11/2023".to_string()
            );
        }

        #[test]
        fn should_return_not_formatted_value_less_three_bcze_brazilian_timezone() {
            assert_eq!(
                DisplayRiverInfo::to_readable_date("2023-10-11T12:00:04Z".to_string()),
                "11/10/2023 09:00:04".to_string()
            );
        }

        #[test]
        fn should_return_not_formatted_value_less_three_bcze_brazilian_timezone_test_case_2() {
            assert_eq!(
                DisplayRiverInfo::to_readable_date("2023-10-29T17:00:00Z".to_string()),
                "29/10/2023 14:00:00".to_string()
            );
        }
    }

    mod display_river_info {
        use mockall::predicate::eq;

        use super::*;
        use crate::core::MockLogger;

        #[test]
        fn should_print_formatted_value() {
            let ctx = MockLogger::print_context();

            ctx.expect()
                .once()
                .with(eq("11/10/2023 17:00:03 - 4.00 metros "))
                .returning(|_| ());

            DisplayRiverInfo::display_river_info(
                &NivelItem {
                    datetime: String::from("2023-10-11T20:00:03Z"),
                    level: 4f32,
                },
                &Some(0f32),
            );
        }
    }
}
