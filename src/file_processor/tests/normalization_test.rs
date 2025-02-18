#[cfg(test)]
mod tests {
    use crate::file_processor::normalization::{
        is_normalized_date_pattern_match, normalize_file_name,
    };

    #[test]
    fn test_normalize_file_name() {
        assert_eq!(
            normalize_file_name("IMG-20240101-test.jpg"),
            "IMG_20240101_test.jpg"
        );
        assert_eq!(normalize_file_name("no_dashes_here"), "no_dashes_here");
    }

    #[test]
    fn test_is_normalized_date_pattern_match() {
        assert!(is_normalized_date_pattern_match("IMG_20240101_sample.jpg"));
        assert!(is_normalized_date_pattern_match(
            "PANO_20221111_panorama.png"
        ));

        assert!(!is_normalized_date_pattern_match("VID_20231231.mp4")); // No underscore after date
        assert!(!is_normalized_date_pattern_match("image_20240101.jpg")); // Wrong prefix
        assert!(!is_normalized_date_pattern_match("IMG-20240101.jpg")); // Hyphen instead of underscore
        assert!(!is_normalized_date_pattern_match("IMG_2024010A_test.png")); // Invalid date
    }
}
