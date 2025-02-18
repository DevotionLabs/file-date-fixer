use chrono::{Local, NaiveDate};
use file_date_fixer::file_processor::directory::process_dir;
use std::fs::{self, File};
use tempfile::TempDir;

#[cfg(target_os = "linux")]
use file_date_fixer::file_metadata::linux::get_file_modification_date as get_file_metadata_date;

#[cfg(target_os = "windows")]
use my_project::file_metadata::windows::get_file_creation_date as get_file_metadata_date;

struct TestFile {
    name: &'static str,
    date: Option<NaiveDate>,
}

const VALID_TEST_FILES: &[TestFile] = &[
    TestFile {
        name: "IMG_20180401_191831.jpg",
        date: NaiveDate::from_ymd_opt(2018, 4, 1),
    },
    TestFile {
        name: "PANO_20180403_095317.jpg",
        date: NaiveDate::from_ymd_opt(2018, 4, 3),
    },
    TestFile {
        name: "IMG-20180405-WA0001.jpg",
        date: NaiveDate::from_ymd_opt(2018, 4, 5),
    },
    TestFile {
        name: "VID-20180406-WA0048.mp4",
        date: NaiveDate::from_ymd_opt(2018, 4, 6),
    },
    TestFile {
        name: "recursion/IMG-20180401-191831.jpg",
        date: NaiveDate::from_ymd_opt(2018, 4, 1),
    },
    TestFile {
        name: "recursion/PANO_20180403_095317.jpg",
        date: NaiveDate::from_ymd_opt(2018, 4, 3),
    },
];

const FUTURE_DATE_TEST_FILES: &[TestFile] = &[
    TestFile {
        name: "VID_59501227_060723_283.mp4",
        date: NaiveDate::from_ymd_opt(5950, 12, 27),
    },
    TestFile {
        name: "IMG-21001227-WA0001.jpg",
        date: NaiveDate::from_ymd_opt(2100, 12, 27),
    },
];

const RECURSION_TEST_FILE: TestFile = TestFile {
    name: "recursion/VID_59501227_060723_283.mp4",
    date: NaiveDate::from_ymd_opt(2011, 12, 27),
};

fn setup_test_dir(test_files: &[TestFile]) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    println!("Creating test directory: {:?}", temp_dir.path());

    for file in test_files {
        let file_path = temp_dir.path().join(file.name);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create subdirectory");
        }

        File::create(&file_path).expect("Failed to create test file");
    }

    temp_dir
}

#[test]
fn test_file_metadata_retrieval() {
    let test_dir = setup_test_dir(VALID_TEST_FILES);

    let test_file_path = test_dir.path().join(VALID_TEST_FILES[0].name);

    let file_date =
        get_file_metadata_date(&test_file_path).expect("Failed to retrieve file metadata date");

    let today = Local::now().date_naive();

    assert_eq!(
        file_date, today,
        "File metadata date does not match today's date"
    );
}

#[test]
fn test_file_metadata_update() {
    let test_dir = setup_test_dir(VALID_TEST_FILES);

    process_dir(&test_dir.path(), true);

    for test_file in VALID_TEST_FILES {
        let file_path = test_dir.path().join(test_file.name);

        let actual_date =
            get_file_metadata_date(&file_path).expect("Failed to retrieve file metadata date");

        if let Some(expected_date) = test_file.date {
            assert_eq!(
                actual_date, expected_date,
                "File {:?} has incorrect metadata date. Expected: {}, Got: {}",
                file_path, expected_date, actual_date
            );
        }
    }
}

#[test]
fn test_future_date_file() {
    let test_dir = setup_test_dir(FUTURE_DATE_TEST_FILES);

    process_dir(&test_dir.path(), true);

    for test_file in FUTURE_DATE_TEST_FILES {
        let file_path = test_dir.path().join(test_file.name);

        let actual_date =
            get_file_metadata_date(&file_path).expect("Failed to retrieve file metadata date");

        let expected_date = Local::now().date_naive();

        assert_eq!(
            actual_date, expected_date,
            "File {:?} with future date in its name should be set to today's date. Expected: {}, Got: {}",
            file_path, expected_date, actual_date
        );
    }
}

#[test]
fn test_non_recursive_metadata_update() {
    let test_dir = setup_test_dir(&[RECURSION_TEST_FILE]);

    let nested_file_path = test_dir.path().join(RECURSION_TEST_FILE.name);

    let original_date = get_file_metadata_date(&nested_file_path)
        .expect("Failed to retrieve original file metadata date");

    process_dir(&test_dir.path(), false); // No recursion

    let updated_date = get_file_metadata_date(&nested_file_path)
        .expect("Failed to retrieve updated file metadata date");

    assert_eq!(
        original_date, updated_date,
        "File {:?} inside a subdirectory should not be updated when recursive=false",
        nested_file_path
    );
}
