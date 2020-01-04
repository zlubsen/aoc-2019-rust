use std::str::FromStr;

fn main() {
    let range_from = 145852;
    let range_to = 616942;

    let mut valid_pwds = 0;

    for i in range_from..=range_to {
        if is_within_range(i, range_from, range_to)
            && is_six_digit(i)
            && has_two_adjacent(i)
            && is_increasing(i) {
            valid_pwds += 1;
        }
    }
    println!("There are {} combinations valid", valid_pwds);
}

fn is_six_digit(number : i32) -> bool {
    number >= 100000 && number <= 999999
}

fn is_within_range(number : i32, range_from : i32, range_to : i32) -> bool {
    number >= range_from && number <= range_to
}

fn has_two_adjacent(number : i32) -> bool {
    let text = number.to_string();
    let mut prev_c : Option<char> = Option::None;
    for c in text.chars() {
        if prev_c.is_none() {
            prev_c = Option::Some(c);
        } else {
            if prev_c.unwrap() == c {
                return true;
            } else {
                prev_c = Option::Some(c);
            }
        }
    }

    false
}

fn is_increasing(number : i32) -> bool {
    let text = number.to_string();
    let mut prev_c = 0;
    for c in text.chars() {
        let val_c = i32::from_str(c.to_string().as_str()).unwrap();
        if val_c >= prev_c {
            prev_c = val_c;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{has_two_adjacent, is_six_digit, is_within_range, is_increasing};

    #[test]
    fn test_has_two_adjacent() {
        assert_eq!(has_two_adjacent(111111), true);
        assert_eq!(has_two_adjacent(123345), true);
        assert_eq!(has_two_adjacent(123456), false);
        assert_eq!(has_two_adjacent(123789), false);
    }

    #[test]
    fn test_is_six_digit() {
        assert_eq!(is_six_digit(123456), true);
        assert_eq!(is_six_digit(12345), false);
        assert_eq!(is_six_digit(1234567), false);
    }

    #[test]
    fn test_is_within_range() {
        assert_eq!(is_within_range(150000, 145852, 616942), true);
        assert_eq!(is_within_range(123456, 145852, 616942), false);
    }

    #[test]
    fn test_is_increasing() {
        assert_eq!(is_increasing(123456), true);
        assert_eq!(is_increasing(111123), true);
        assert_eq!(is_increasing(135679), true);
        assert_eq!(is_increasing(223450), false);
    }
}