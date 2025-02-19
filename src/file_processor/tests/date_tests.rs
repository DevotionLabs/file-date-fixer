#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate};

    use crate::file_processor::date::{
        extract_date_from_normalized_file_name, is_future_date, parse_normalized_date,
    };

    #[test]
    fn test_extract_date_from_normalized_file_name_valid() {
        assert_eq!(
            extract_date_from_normalized_file_name("IMG_20240101_sample.jpg"),
            "20240101"
        );
        assert_eq!(
            extract_date_from_normalized_file_name("PANO_19991231_panorama.png"),
            "19991231"
        );
    }

    #[test]
    #[should_panic]
    fn test_extract_date_from_normalized_file_name_invalid_format() {
        extract_date_from_normalized_file_name("IMG-20240101.jpg"); // No underscores
    }

    #[test]
    fn test_parse_normalized_date_valid() {
        let parsed = parse_normalized_date("20240101").expect("Failed to parse valid date");
        assert_eq!(parsed, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    }

    #[test]
    fn test_parse_normalized_date_invalid_format() {
        assert!(parse_normalized_date("20XX0101").is_err()); // Non-numeric characters
        assert!(parse_normalized_date("2024-01-01").is_err()); // Hyphen format
        assert!(parse_normalized_date("20241301").is_err()); // Invalid month (13)
        assert!(parse_normalized_date("20240230").is_err()); // Invalid day (Feb 30)
    }

    #[test]
    fn test_is_future_date() {
        let future_date = Local::now().date_naive() + chrono::Duration::days(1);
        assert!(is_future_date(future_date));

        let today = Local::now().date_naive();
        assert!(!is_future_date(today));

        let past_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        assert!(!is_future_date(past_date));
    }
}
