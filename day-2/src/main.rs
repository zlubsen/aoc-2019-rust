use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Reading input file: {}", path);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");

    let original_memory : Vec<i32> = parse_instructions(contents);

    // Part One
    let mut program_memory = original_memory.clone();
    correct_program(&mut program_memory);
    let answer_1 = run_program(program_memory)[0];
    println!("Part One: Position 0 contains {}", answer_1);

    // Part Two
    for noun in 0..99 {
        for verb in 0..99 {
            let mut program_memory = original_memory.clone();
            alter_program(&mut program_memory, noun, verb);
            let result_memory = run_program(program_memory);
            let result = result_memory[0];
            if result == 19690720 {
                let answer_2 = (100 * result_memory[1]) + result_memory[2];
                println!("Part two: noun = {}, verb = {} and answer = {}", noun, verb, answer_2);
            }
        }
    }
}

fn parse_instructions(contents : String) -> Vec<i32> {
    contents.split(",").filter_map(|w| w.parse().ok()).collect()
}

fn correct_program(instr : &mut Vec<i32>) -> () {
    instr[1] = 12;
    instr[2] = 2;
}

fn alter_program(instr : &mut Vec<i32>, i : i32, j : i32) -> () {
    instr[1] = i;
    instr[2] = j;
}

fn run_program(instr : Vec<i32>) -> Vec<i32> {
    let mut program = Program {
        instructions : instr,
        pc : 0,
        finished : false,
    };

    while !program.finished {
        program.do_operation();
    }
    program.instructions
}

struct Program {
    instructions : Vec<i32>,
    pc : usize,
    finished : bool
}

impl Program {
    fn next_operation(&mut self) -> &Self {
        self.pc = self.pc + 4;
        self
    }

    fn do_operation(&mut self) -> &Self {
        match self.instructions.get(self.pc).unwrap() {
            1 => self.op_add(),
            2 => self.op_mult(),
            99 => self.op_exit(),
            _ => ()
        }

        if ! self.finished {
            self.next_operation();
        }
        self
    }

    fn op_add(&mut self) {
        let op1 = self.instructions.get(*self.instructions.get(self.pc + 1).unwrap() as usize).unwrap();
        let op2 = self.instructions.get(*self.instructions.get(self.pc + 2).unwrap() as usize).unwrap();
        let destination = *self.instructions.get(self.pc + 3).unwrap() as usize;
        self.instructions[destination] = op1 + op2;
    }

    fn op_mult(&mut self) {
        let op1 = self.instructions.get(*self.instructions.get(self.pc + 1).unwrap() as usize).unwrap();
        let op2 = self.instructions.get(*self.instructions.get(self.pc + 2).unwrap() as usize).unwrap();
        let destination = *self.instructions.get(self.pc + 3).unwrap() as usize;
        self.instructions[destination] = op1 * op2;
    }

    fn op_exit(&mut self) {
        self.finished = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::run_program;
    use crate::parse_instructions;

    #[test]
    fn test_one() {
        assert_eq!(run_program(parse_instructions(String::from("1,0,0,0,99"))), vec![2,0,0,0,99]);
    }

    #[test]
    fn test_two() {
        assert_eq!(run_program(parse_instructions(String::from("2,3,0,3,99"))), vec![2,3,0,6,99]);
    }

    #[test]
    fn test_three() {
        assert_eq!(run_program(parse_instructions(String::from("2,4,4,5,99,0"))), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_four() {
        assert_eq!(run_program(parse_instructions(String::from("1,1,1,4,99,5,6,0,99"))), vec![30,1,1,4,2,5,6,0,99]);
    }

}