use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    let var = &args[2];
    let variant : i32 = var.parse().unwrap();

    println!("Reading input file: {}", path);
    println!("Calculating variant {}", variant);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");

    let values : Vec<i32>;
    if variant == 1 {
        values = contents.split("\n").filter_map(|w| w.parse().ok()).map(|v: i32| (v / 3) - 2).collect();
    } else if variant == 2 {
        values = contents.split("\n").filter_map(|w| w.parse().ok()).map(|v : i32| calculate_fuel(&v)).collect();
    } else {
        println!("Unknown variant supplied.");
        exit(0);
    }
    let total : i32 = values.iter().sum();

    println!("Sum is {}", total);
}

fn calculate_fuel(mass : &i32) -> i32 {
    if *mass > 0 {
        let fuel : i32 = *mass / 3 - 2;
        if fuel >= 1 {
            fuel + calculate_fuel(&fuel)
        } else {
            0
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_fuel;

    #[test]
    fn test_14() {
        assert_eq!(calculate_fuel(&14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(calculate_fuel(&1969), 966);
    }

    #[test]
    fn test_100756() {
        assert_eq!(calculate_fuel(&100756), 50346);
    }
}