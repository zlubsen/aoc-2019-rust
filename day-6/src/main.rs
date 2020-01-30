use std::{env, fs};
use regex::Regex;
use std::collections::HashMap;
use std::borrow::BorrowMut;
use syntax::util::map_in_place::MapInPlace;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Reading input file: {}", path);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");
}

#[derive(Debug)]
struct Galaxy<'s> {
    planets : HashMap<&'s str, TreeNode<'s>>,
}

impl <'s> Galaxy<'s> {
    fn new() -> Galaxy<'static> {
        Galaxy {
            planets : HashMap::new(),
        }
    }

    fn build_orbits_tree(&mut self, input : &'s str) {
        let lines : Vec<&'s str>= input.split("\n").collect();

        for line in &lines {
            let orbit : Vec<&str> = line.split(")").collect();
            let planet = self.find_or_create_planet(orbit.get(0).unwrap());
            let satellite = self.find_or_create_planet(orbit.get(1).unwrap());
            planet.borrow_mut().add_satellite(satellite);
        }
    }

    fn find_or_create_planet(&mut self, key : &'s str) -> &TreeNode {
        self.planets.entry(key).or_insert(
            TreeNode {
                name: String::from(key),
                satellites: vec![],
                depth: 0,
            }
        )
    }

    fn add_planet(&mut self, key : &'s str) {
        self.planets.insert(key, TreeNode {
            name: String::from(key),
            satellites: vec![],
            depth: 0,
        });
    }

    fn get_planet(&self, key : &str) -> Option<&TreeNode> {
        self.planets.get(key)
    }

    fn set_tree_depths(node: &'s mut TreeNode<'s>, depth : u32) {
//        node.depth = depth;
//        for mut satellite in node.satellites {
//            Galaxy::set_tree_depths(satellite.borrow_mut(), depth + 1);
//        }
    }
}

#[derive(Debug)]
struct TreeNode<'a> {
    name : String,
    satellites : Vec<&'a TreeNode<'a>>,
    depth : u32,
}

impl <'a> TreeNode<'a> {
    fn add_satellite(&mut self, satellite : &'a TreeNode<'a>) {
        self.satellites.push(satellite);
    }

    fn set_depth(&mut self, depth : u32) {
        self.depth = depth;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Galaxy, TreeNode};

    #[test]
    fn test_inserting() {
        let mut galaxy = Galaxy::new();

        assert_eq!(galaxy.planets.len(), 0);
        galaxy.add_planet("AAA");
        galaxy.add_planet("BBB");
        assert_eq!(galaxy.planets.len(), 2);
        let pl_c: &TreeNode;
        {
            let pl_c = galaxy.find_or_create_planet("CCC");
        }
        assert_eq!(galaxy.planets.len(), 3);
        let pl_c2: &TreeNode;
        {
            pl_c2 = galaxy.find_or_create_planet("CCC");
        }
        assert_eq!(pl_c2.name, "CCC");
        assert_eq!(galaxy.planets.len(), 3);
    }

    #[test]
    fn test_orbits() {

//        assert_eq!(galaxy.get_planet("AAA").unwrap().depth, 0);
//        assert_eq!(galaxy.get_planet("BBB").unwrap().depth, 0);
//        assert_eq!(galaxy.get_planet("CCC").is_none(), true);

//        assert_eq!(orbits, vec![(planets.get("AAA").unwrap(),planets.get("BBB").unwrap())]);

//        assert_eq!(parse_input("zon)aarde\naarde)maan"), vec![("zon", "aarde"),("aarde","maan")]);
    }
}