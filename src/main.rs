use std::str::FromStr;

#[derive(Debug)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum IsbnError {
    InvalidLength,
    InvalidCheckDigit,
}

#[derive(Debug)]
struct Error {
    error: IsbnError,
}

impl From<IsbnError> for Error {
    fn from(error: IsbnError) -> Self {
        Error { error }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error {
            IsbnError::InvalidLength => write!(f, "Invalid length"),
            IsbnError::InvalidCheckDigit => write!(f, "Invalid check digit"),
        }
    }
}

impl std::error::Error for Error {}

impl FromStr for Isbn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.to_string();
        let digits: Vec<u8> = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        if digits.len() != 13 {
            return Err(Error::from(IsbnError::InvalidLength));
        }

        let check = calculate_check_digit(&digits[0..12]);
        if check != digits[12] {
            return Err(Error::from(IsbnError::InvalidCheckDigit));
        }

        Ok(Isbn { raw, digits })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut sum = 0_u8;
    digits.iter().enumerate().for_each(|(index, digit)| {
        sum += digit * {
            match index % 2 {
                0 => 1,
                _ => 3,
            }
        };
    });

    let check = 10 - (sum % 10);
    match check {
        10 => 0,
        _ => check,
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}

#[test]
fn rust_in_action_is_invalid() {
    let result: Result<Isbn, Error> = "978-3-16-148410-1".parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().error, IsbnError::InvalidCheckDigit);
}

#[test]
fn rust_in_action_is_invalid_length() {
    let result: Result<Isbn, Error> = "978-3-16-148410".parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().error, IsbnError::InvalidLength);
}
