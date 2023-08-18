use std::convert::TryFrom;
use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {
    let start_result = NaiveDate::parse_from_str(a, "%Y-%m-%d");
    return match start_result {
        Ok(date) => {
            let start = date;
            let end_result = NaiveDate::parse_from_str(b, "%Y-%m-%d");
            match end_result {
                Ok(date) => {
                    let end = date;
                    let duration = end.signed_duration_since(start);
                    i32::try_from(duration.num_weeks()).unwrap()
                },
                // If the end date is invalid, return 0
                Err(_) => 0,
            }
        },
        // If the start date is invalid, return 0
        Err(_) => 0,
    };
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}