use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Reading input file: {}", path);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");

    let mut program = Program {
        instructions : contents.split(",").filter_map(|w| w.parse().ok()).collect(),
        pc : 0,
    };
}

struct Program {
    instructions : Vec<i32>,
    pc : i32
}

impl Program {
    fn next_operation(&mut self) -> &Self {
        self.pc = self.pc + 4;
        self
    }

    fn do_operation(&self) -> &Self {
        match self.instructions.get(self.pc).unwrap() {
            1 => self.op_add(),
            2 => self.op_mult(),
            99 => self.op_exit(),
        }
        self
    }

    fn op_add() {

    }

    fn op_mult() {
        
    }

    fn op_exit() {

    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one() {
        assert_eq!(1+1, 2);
    }

}