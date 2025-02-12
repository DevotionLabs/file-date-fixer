use chrono::{Local, NaiveDate, ParseResult};

pub fn extract_date_from_normalized_file_name(file_name: &str) -> &str {
    file_name.split('_').nth(1).unwrap()
}

pub fn parse_normalized_date(date_str: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y%m%d")
}

pub fn is_future_date(parsed_date: NaiveDate) -> bool {
    parsed_date > Local::now().date_naive()
}
