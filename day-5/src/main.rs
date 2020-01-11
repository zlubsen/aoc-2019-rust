use std::{fs, env};
use std::collections::HashMap;
use std::borrow::Borrow;

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
    automaton.init();
    automaton.load(contents.as_str());
    automaton.run();
}

struct InstructionDef {
    opcode : i8,
    no_params : i8,
}

struct Instruction {
    opcode : i8,
    params : Vec<Parameter>,
}

#[derive(Debug)]
struct Parameter {
    param : i32,
    mode : ParameterMode,
}

#[derive(Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

struct Automaton {
    instruction_set : HashMap<i8, InstructionDef>,
    pc : usize,
    finished : bool,
    memory : Vec<i32>,
}

impl Automaton {

    fn init(&mut self) {
        self.instruction_set.insert(1, InstructionDef {opcode : 1, no_params : 3});
        self.instruction_set.insert(2, InstructionDef {opcode : 2, no_params : 3});
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
                param : *self.memory.get(self.pc + i).unwrap(),
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
        while op_extended.len() < 5 {
            op_extended.push('0');
        }
        let op_extended = op_extended.chars().rev().collect::<String>();
        let opcode = &op_extended[3..=4].parse::<i8>().ok().unwrap();

        let mut params = Vec::new();
        for i in 1..=(self.instruction_set.get(&opcode).unwrap().no_params as usize) {
            let index = 3-i;
            let param_mode = &op_extended[index..=index].parse().ok().unwrap();
            let param = *self.memory.get(self.pc + i).unwrap();

            params.push(Parameter {
                param,
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

    fn read_input(&self) -> i32 {
        1
    }

    fn do_operation(&mut self, instruction : &Instruction) -> &Self {
        match instruction.opcode {
            1 => self.op_add(instruction),
            2 => self.op_mult(instruction),
            3 => self.op_input(instruction),
            4 => self.op_output(instruction),
            99 => self.op_exit(),
            _ => (),
        }

        self.pc += self.get_increment_for_opcode(&instruction.opcode);

        self
    }

    fn get_param_value<'a>(&'a self, instr : &'a Instruction, param_index: usize) -> &'a i32 {
        match instr.params.get(param_index).unwrap().mode {
            ParameterMode::Position => self.memory.get(instr.params.get(param_index).unwrap().param as usize).unwrap(),
            ParameterMode::Immediate => instr.params.get(param_index).unwrap().param.borrow(),
        }
    }
    fn get_address_value<'a>(&'a self, instr : &'a Instruction, param_index: usize) -> &'a i32 {
        instr.params.get(param_index).unwrap().param.borrow()
    }

    fn get_increment_for_opcode(&self, opcode : &i8) -> usize {
        (self.instruction_set.get(opcode).unwrap().no_params + 1) as usize
    }

    fn op_add(&mut self, instr : &Instruction) {
        let result;
        let address;
        {
            let op1 = self.get_param_value(instr, 0);
            let op2 = self.get_param_value(instr, 1);
            address = *self.get_address_value(instr, 2) as usize;
            result = op1 + op2;
//            println!("add: opcode {} - op1 {} - op2 {} - address {} => {}", instr.opcode, op1, op2, address, result);
        }
        self.memory[address] = result;
    }

    fn op_mult(&mut self, instr : &Instruction) {
        let result;
        let address;
        {
            let op1 = self.get_param_value(instr, 0);
            let op2 = self.get_param_value(instr, 1);
            address = *self.get_address_value(instr, 2) as usize;
            result = op1 * op2;
//            println!("mul: opcode {} - op1 {} - op2 {} - address {} => {}", instr.opcode, op1, op2, address, result);
        }
        self.memory[address] = result;
    }

    fn op_input(&mut self, instr : &Instruction) {
        let address = *self.get_address_value(instr, 0) as usize;
        self.memory[address] = self.read_input();

//        println!("input: opcode {} - address {} => {}", instr.opcode, address, self.memory[address]);
    }

    fn op_output(&mut self, instr : &Instruction) {
        let address = *self.get_address_value(instr, 0) as usize;
        println!("{}", self.memory[address]);
    }

    fn op_exit(&mut self) {
        self.finished = true;
        println!("halt");
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

    #[test]
    fn test_negative_values() {
        let mut automaton = Automaton {
            instruction_set: HashMap::new(),
            pc: 0,
            finished: false,
            memory: vec![]
        };
        automaton.init();
        automaton.load("1101,100,-1,4,0");
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![1101,100,-1,4,99]);
    }

}