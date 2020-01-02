use std::{env, fs};

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Reading input file: {}", path);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");

    let lines : Vec<&str> = contents.split("\n").collect();
    let mut wires : Vec<Vec<Move>> = Vec::new();
    for line in &lines {
        wires.push(line.split(",").map(|s| Move::from_string(s)).collect());
    }

//    println!("line:\n\t{:?}", lines[0]);
//    println!("wire:\n\t{:?}", wires[0]);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction : Direction,
    amount : i32,
}

impl Move {
    fn from_string(input_string : &str) -> Move {
        let direction = match input_string.chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Ai... parse error for a direction")
        };
        let amount : i32 = input_string[1..].parse().ok().unwrap();
        let mv = Move {
            direction,
            amount,
        };
        mv
    }
}

