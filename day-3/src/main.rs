use std::{env, fs};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Reading input file: {}", path);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");

    println!("Got contents");
    let wires : Vec<Vec<Move>> = parse_input(&contents);

    println!("Parsed wires");
    let maps : Vec<Vec<(i32,i32)>> = generate_maps(&wires);

    println!("Generated maps");
    let intersections = find_intersections(&maps[0], &maps[1]);
    println!("Determined intersections");
    let smallest = find_smallest_manhattan(&intersections);
    println!("Smallest manhattan distance is {}", smallest);

    let nearest = find_nearest_intersection(&maps, &intersections);
    println!("Nearest intersection is {}", nearest);
//    println!("sizes: {} - {}", maps[0].len(), maps[1].len());
//    println!("intersections: {:?}", intersections )
}

fn parse_input(contents : &str) -> Vec<Vec<Move>> {
    let lines : Vec<&str> = contents.split("\n").collect();
    let mut wires : Vec<Vec<Move>> = Vec::new();
    for line in &lines {
        wires.push(line.split(",").map(|s| Move::from_string(s)).collect());
    }
    wires
}

fn generate_maps(wires: &Vec<Vec<Move>>) -> Vec<Vec<(i32,i32)>> {
    let mut maps : Vec<Vec<(i32,i32)>> = Vec::new();
    for wire in wires {
        let mut path : Vec<(i32,i32)> = Vec::new();
        let mut coord : (i32,i32) = (0,0);
        for mv in wire {
            for _ in 1..=mv.amount {
                match mv.direction {
                    Direction::Up => {coord.1 += 1},
                    Direction::Down => {coord.1 += -1}
                    Direction::Left => {coord.0 += -1}
                    Direction::Right => {coord.0 += 1}
                };
                path.push(coord);
            }
        }
        maps.push(path);
    }
    maps
}

fn find_intersections<'a>(a : &'a Vec<(i32,i32)>, b : &'a Vec<(i32,i32)>) -> HashSet<(i32,i32)> {
//    let set_one = HashSet::from_iter(a.iter().cloned());
//    let set_two = HashSet::from_iter(b.iter().cloned());
//    let set_one = HashSet::new().extend(a.iter());
//    let set_two = HashSet::new().extend(b.iter());
//    set_one.intersection(&set_two).collect()
    let mut intersect : HashSet<(i32,i32)> = HashSet::new();
    for i in a {
        print!(".");
        if b.contains(i) {
            intersect.insert((i.0,i.1));
        }
    }
    println!(".");
    intersect
}

fn find_smallest_manhattan(intersections : &HashSet<(i32,i32)>) -> i32 {
    let mut smallest : i32 = i32::max_value();
    for intersection in intersections {
        let dist = calc_manhattan(intersection);
        if dist <= smallest {
            smallest = dist;
        }
    }
    smallest
}

fn calc_manhattan(a : &(i32,i32)) -> i32 {
    a.0.abs() + a.1.abs()
}

fn find_nearest_intersection(maps : &Vec<Vec<(i32,i32)>>, intersections : &HashSet<(i32,i32)>) -> i32 {
    let mut nearest : usize = usize::max_value();
    for crossing in intersections {
        let len_one = maps[0].iter().position(|x| x == crossing).unwrap() + 1;
        let len_two = maps[1].iter().position(|x| x == crossing).unwrap() + 1;
        if len_one + len_two <= nearest {
            nearest = len_one + len_two;
        }
    }
    nearest as i32
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

#[cfg(test)]
mod tests {
    use crate::{parse_input, find_intersections, calc_manhattan, find_smallest_manhattan, find_nearest_intersection};
    use crate::generate_maps;

    #[test]
    fn test_manhattan() {
        assert_eq!(calc_manhattan(&(3,3)), 6);
    }

    #[test]
    fn test_one() {
        let input : String = String::from("R8,U5,L5,D3\nU7,R6,D4,L4");
        let maps = generate_maps(&parse_input(&input));
        let intersects = find_intersections(&maps[0], &maps[1]);
        let smallest = find_smallest_manhattan(&intersects);
        let nearest = find_nearest_intersection(&maps, &intersects);
        assert_eq!(maps[0].len(), 21);
        assert_eq!(maps[1].len(), 21);
        assert_eq!(intersects.len(), 2);
        assert_eq!(smallest, 6);
        assert_eq!(nearest, 30);
    }

    #[test]
    fn test_two() {
        let input : String = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let maps = generate_maps(&parse_input(&input));
        let intersects = find_intersections(&maps[0], &maps[1]);
        let smallest = find_smallest_manhattan(&intersects);
        let nearest = find_nearest_intersection(&maps, &intersects);
        assert_eq!(smallest, 159);
        assert_eq!(nearest, 610);
    }

    #[test]
    fn test_three() {
        let input : String = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let maps = generate_maps(&parse_input(&input));
        let intersects = find_intersections(&maps[0], &maps[1]);
        let smallest = find_smallest_manhattan(&intersects);
        let nearest = find_nearest_intersection(&maps, &intersects);
        assert_eq!(smallest, 135);
        assert_eq!(nearest, 410);
    }
}

