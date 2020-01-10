use std::str::FromStr;
use regex::Regex;

fn main() {
    let range_from = 145852;
    let range_to = 616942;

    let mut valid_pwds_pt_1 = 0;
    let mut valid_pwds_pt_2 = 0;

    for i in range_from..=range_to {
        if i % 20000 == 0 { print!("."); }

        if is_within_range(i, range_from, range_to)
            && is_six_digit(i)
            && is_increasing(i) {
            if has_two_adjacent(i) {
                valid_pwds_pt_1 += 1;
                println!("\nFound adjacent!");
                if has_double(i) {
                    valid_pwds_pt_2 += 1;
                    print!("It's a double!");
                }
            }
        }
    }
    println!("There are {} combinations valid", valid_pwds_pt_1);
    println!("There are {} combinations for part 2", valid_pwds_pt_2);
}

fn is_six_digit(number : i32) -> bool {
    let re = Regex::new(r"^\d{6}$").unwrap();
    re.is_match(number.to_string().as_str())
}

fn is_within_range(number : i32, range_from : i32, range_to : i32) -> bool {
    number >= range_from && number <= range_to
}

fn has_two_adjacent(number : i32) -> bool {
    // bleh, rust regex does not support look-around and back-references...
//    let re = Regex::new(r"^\d*((\d)\2)\d*$").unwrap();
//    re.is_match(number.to_string().as_str())
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

fn has_double(number : i32) -> bool {
//    let re = Regex::new(r"([\d])\1{1}(?!\1)").unwrap();
//    println!("text: {}", text);
//    let matches = re.is_match(number.to_string().as_str());
//    println!("{}", matches);

    // fill a vec with counts of all adjacent characters
    let text = number.to_string();
    let mut curr_c: Option<char> = Option::None;
    let mut count : i32 = 0;
    let mut adjacents : Vec<(char, i32)> = Vec::new();
    for c in text.chars() {
        if curr_c.is_none() {
            curr_c = Option::Some(c);
            count += 1;
        } else {
            if curr_c.unwrap() == c {
                count += 1;
            } else {
                if count > 1 {
                    adjacents.push((curr_c.unwrap(), count));
                }
                curr_c = Option::Some(c);
                count = 1;
            }
        }
    }
    if count > 1 {
        adjacents.push((curr_c.unwrap(), count));
    }

    // filter any adjacents > 2
    let mut doubles : Vec<(char, i32)> = Vec::new();
    if adjacents.is_empty() {
        // when no adjacent items found
        return false
    } else {
        for tup in &adjacents {
            if tup.1 == 2 {
                doubles.push(*tup);
            }
        }
    }
    !doubles.is_empty()
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
    use crate::{has_two_adjacent, is_six_digit, is_within_range, is_increasing, has_double};

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

    #[test]
    fn test_has_double() {
        assert_eq!(has_double(112233), true);
        assert_eq!(has_double(123444), false);
        assert_eq!(has_double(111122), true);
    }
}