use super::*;

#[test]
fn valid_adder() {
    assert_eq!(adder(2, 2), 4);
}

#[test]
fn valid_search_pattern() {
    let pattern = String::from("python");
    let path = PathBuf::from("src/utils/fun/test.txt");
    assert!(search_pattern(pattern, path).is_ok());
}

#[test]
fn invalid_search_pattern() {
    let pattern = String::from("python");
    let path = PathBuf::from("not_valid.tx");
    assert!(search_pattern(pattern, path).is_err());
}

#[test]
fn valid_progress_bar_with_sleep() {
    progress_bar_with_sleep(15 as u64);
}

#[test]
fn valid_adder_large_numbers() {
    assert_eq!(adder(1000000, 2000000), 3000000);
}

#[test]
fn invalid_adder_negative_numbers() {
    assert_ne!(adder(-6, 10), 5);
}

#[test]
fn invalid_adder_bad_params() {
    // this is done to ensure test will compile and run
    let result = std::panic::catch_unwind(|| {
        let invalid_param = "invalid".parse::<i64>().unwrap();
        adder(1, invalid_param);
    });

    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn invalid_adder_max_i64() {
    adder(i64::MAX, 10);
}
