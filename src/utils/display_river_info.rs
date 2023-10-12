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
