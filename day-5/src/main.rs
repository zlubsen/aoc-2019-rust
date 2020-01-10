use std::{fs, env};
use std::collections::HashMap;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Reading input file: {}", path);

    let contents = fs::read_to_string(path).expect("Failed to read contents of file");

    let mut automaton = Automaton {
        instruction_set: HashMap::new(),
        pc: 0,
        finished: false,
        memory: vec![]
    };
    automaton.load(contents.as_str());
}

struct InstructionDef {
    opcode : i8,
    no_params : i8,
}

struct InstructionSet {
    set : HashMap<i8, InstructionDef>,
}

struct Program {
//    instructions : Vec<Instruction>,
}

struct Instruction {
    opcode : i8,
    params : Vec<Parameter>,
}

struct Parameter {
    param : i8,
    mode : ParameterMode,
}

enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

struct Automaton {
//    operations : HashMap<i8, Operation>,
    instruction_set : HashMap<i8, InstructionDef>,
    pc : usize,
    finished : bool,
    memory : Vec<i32>,
}

impl Automaton {

    fn init(&mut self) {
        self.instruction_set.insert(1, InstructionDef {opcode : 1, no_params : 2});
        self.instruction_set.insert(2, InstructionDef {opcode : 2, no_params : 2});
        self.instruction_set.insert(3, InstructionDef {opcode : 3, no_params : 1});
        self.instruction_set.insert(4, InstructionDef {opcode : 4, no_params : 1});
        self.instruction_set.insert(99, InstructionDef {opcode : 99, no_params : 0});
    }

    fn load(&mut self, input : &str) {
        self.pc = 0;
        self.finished = false;
        self.memory = input.split(",").filter_map(|w| w.parse().ok()).collect();
    }

    fn decode(&mut self) -> Option<Instruction> {
        let val = self.memory.get(self.pc).unwrap();
        match val {
            1..=99 => Some(self.decode_default(val)),
            100..=99999 => Some(self.decode_extended(val)),
            _ => None
        }
    }

    fn decode_default(&self, val: &i32) -> Instruction {
        let opcode = *val as i8;
        let mut params = Vec::new();
        for i in 1..=(self.instruction_set.get(&opcode).unwrap().no_params as usize) {
            params.push(Parameter {
                param : *self.memory.get(self.pc + i).unwrap() as i8,
                mode : ParameterMode::Position,
            });
        };
        Instruction {
            opcode,
            params,
        }
    }

    fn decode_extended(&self, val: &i32) -> Instruction {
        let mut op_extended = val.to_string().chars().rev().collect::<String>();
        if op_extended.len() == 4 {
            op_extended.push('0');
        }
        let op_extended = op_extended.chars().rev().collect::<String>();
        let opcode = &op_extended[3..=4].parse::<i8>().ok().unwrap();
        let mut params = Vec::new();
        for i in 0..=(self.instruction_set.get(&opcode).unwrap().no_params as usize) {
            let index = 2-i;
            let param_mode = &op_extended[index..=index].parse().ok().unwrap();
            params.push(Parameter {
                param : *self.memory.get(self.pc + i).unwrap() as i8,
                mode : match param_mode {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
                    _ => panic!("unknown parameter mode"),
                },
            });
        };
        Instruction {
            opcode: *opcode,
            params,
        }
    }

    fn run(&mut self) {
        while !self.finished {
            let instruction = self.decode().unwrap();
            self.do_operation(&instruction);
        }
    }

    fn dump_memory(&self) -> &Vec<i32> {
        &self.memory
    }

    fn do_operation(&mut self, instruction : &Instruction) -> &Self {
        let pc_increment : usize;
        match instruction.opcode {
            1 => pc_increment = self.op_add(),
            2 => pc_increment = self.op_mult(),
            3 => pc_increment = self.op_input(),
            4 => pc_increment = self.op_output(),
            99 => pc_increment = self.op_exit(),
            _ => pc_increment = 0
        }

        self.pc += pc_increment;

        self
    }

    fn op_add(&mut self) -> usize {
        // TODO support immediate mode params
        let op1 = self.memory.get(*self.memory.get(self.pc + 1).unwrap() as usize).unwrap();
        let op2 = self.memory.get(*self.memory.get(self.pc + 2).unwrap() as usize).unwrap();
        let address = *self.memory.get(self.pc + 3).unwrap() as usize;
        self.memory[address] = op1 + op2;

        4
    }

    fn op_mult(&mut self) -> usize {
        // TODO support immediate mode params
        let op1 = self.memory.get(*self.memory.get(self.pc + 1).unwrap() as usize).unwrap();
        let op2 = self.memory.get(*self.memory.get(self.pc + 2).unwrap() as usize).unwrap();
        let address = *self.memory.get(self.pc + 3).unwrap() as usize;
        self.memory[address] = op1 * op2;

        4
    }

    fn op_input(&mut self) -> usize {
        // TODO support immediate mode params
        let address = self.memory.get(*self.memory.get(self.pc + 1).unwrap() as usize).unwrap();
        unimplemented!();

        2
    }

    fn op_output(&mut self) -> usize {
        // TODO support immediate mode params
        let address = self.memory.get(*self.memory.get(self.pc + 1).unwrap() as usize).unwrap();
        unimplemented!();

        2
    }

    fn op_exit(&mut self)  -> usize {
        self.finished = true;

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Automaton;
    use std::collections::HashMap;

    #[test]
    fn test_one() {
        let mut automaton = Automaton {
            instruction_set: HashMap::new(),
            pc: 0,
            finished: false,
            memory: vec![]
        };
        automaton.init();
        automaton.load("1,0,0,0,99");
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,0,0,0,99]);
    }

    #[test]
    fn test_two() {
        let mut automaton = Automaton {
            instruction_set: HashMap::new(),
            pc: 0,
            finished: false,
            memory: vec![]
        };
        automaton.init();
        automaton.load("2,3,0,3,99");
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,3,0,6,99]);
    }

    #[test]
    fn test_three() {
        let mut automaton = Automaton {
            instruction_set: HashMap::new(),
            pc: 0,
            finished: false,
            memory: vec![]
        };
        automaton.init();
        automaton.load("2,4,4,5,99,0");
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_four() {
        let mut automaton = Automaton {
            instruction_set: HashMap::new(),
            pc: 0,
            finished: false,
            memory: vec![]
        };
        automaton.init();
        automaton.load("1,1,1,4,99,5,6,0,99");
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn test_five() {
        let mut automaton = Automaton {
            instruction_set: HashMap::new(),
            pc: 0,
            finished: false,
            memory: vec![]
        };
        automaton.init();
        automaton.load("1002,4,3,4,33");
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![1002,4,3,4,99]);
    }

}