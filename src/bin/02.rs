use std::str::FromStr;

const INTCODE_MEMORY_SIZE: usize = 130;

#[derive(Debug, PartialEq)]
enum IntCodeInstruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Halt,
}

impl IntCodeInstruction {
    fn get_output_position(&self) -> Option<usize> {
        match self {
            IntCodeInstruction::Add(_, _, x) | IntCodeInstruction::Multiply(_, _, x) => Some(*x),
            IntCodeInstruction::Halt => None,
        }
    }

    fn get_output_value(&self, program: &IntCodeProgram) -> usize {
        match self {
            IntCodeInstruction::Add(a, b, _) => {
                program.read_register(*a) + program.read_register(*b)
            }
            IntCodeInstruction::Multiply(a, b, _) => {
                program.read_register(*a) * program.read_register(*b)
            }
            IntCodeInstruction::Halt => 0,
        }
    }

    fn get_position_change(&self) -> usize {
        if let IntCodeInstruction::Halt = self {
            0
        } else {
            4
        }
    }
}

struct IntCodeProgram {
    registers: [usize; INTCODE_MEMORY_SIZE],
    position: usize,
    terminated: bool,
}

impl IntCodeProgram {
    fn new() -> Self {
        let mut registers = [0; INTCODE_MEMORY_SIZE];
        registers[0] = 99;
        IntCodeProgram {
            registers,
            position: 0,
            terminated: false,
        }
    }

    fn read_register(&self, pos: usize) -> usize {
        if pos >= INTCODE_MEMORY_SIZE {
            0
        } else {
            self.registers[pos]
        }
    }

    fn read_instruction(&self, pos: usize) -> IntCodeInstruction {
        match self.read_register(pos) {
            1 => IntCodeInstruction::Add(
                self.read_register(pos + 1),
                self.read_register(pos + 2),
                self.read_register(pos + 3),
            ),
            2 => IntCodeInstruction::Multiply(
                self.read_register(pos + 1),
                self.read_register(pos + 2),
                self.read_register(pos + 3),
            ),
            _ => IntCodeInstruction::Halt,
        }
    }

    fn step(&mut self) {
        let instruction = self.read_instruction(self.position);

        if let Some(pos) = instruction.get_output_position() {
            self.registers[pos] = instruction.get_output_value(self);
        }

        self.position += instruction.get_position_change();

        if let IntCodeInstruction::Halt = instruction {
            self.terminated = true;
        }
    }

    fn run(&mut self) {
        while !self.terminated {
            self.step();
        }
    }
}

struct ParseIntCodeProgramError;

impl FromStr for IntCodeProgram {
    type Err = ParseIntCodeProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut program = Self::new();
        for (register, value_str) in s.trim().split(',').enumerate() {
            let value = value_str.parse().map_err(|_| ParseIntCodeProgramError)?;
            program.registers[register] = value;
        }
        Ok(program)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(mut program) = input.parse::<IntCodeProgram>() {
        program.registers[1] = 12;
        program.registers[2] = 2;
        program.run();
        Some(program.read_register(0).try_into().unwrap_or(0))
    } else {
        None
    }
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        if let Ok(program) = "1,9,10,3,2,3,11,0,99,30,40,50".parse::<IntCodeProgram>() {
            assert_eq!(
                program.registers[0..12],
                [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
            );
            assert_eq!(program.position, 0);
            assert_eq!(program.terminated, false);
        } else {
            assert!(false);
        }
    }

    fn first_test_program() -> IntCodeProgram {
        let mut program = IntCodeProgram::new();
        for (ix, item) in [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
            .iter()
            .enumerate()
        {
            program.registers[ix] = *item;
        }
        program
    }

    #[test]
    fn test_first_test_program_factory() {
        let program = first_test_program();
        assert_eq!(program.position, 0);
        assert_eq!(
            program.registers[0..12],
            [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(program.terminated, false);
    }

    #[test]
    fn test_read_addition_instruction() {
        let program = first_test_program();
        assert_eq!(
            program.read_instruction(0),
            IntCodeInstruction::Add(9, 10, 3)
        );
    }

    #[test]
    fn test_addition_step() {
        let mut program = first_test_program();
        program.step();
        assert_eq!(program.terminated, false);
        assert_eq!(program.position, 4);
        assert_eq!(
            program.registers[0..12],
            [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_multiply_step() {
        let mut program = first_test_program();
        program.position = 4;
        program.step();
        assert_eq!(program.terminated, false);
        assert_eq!(program.position, 8);
        assert_eq!(
            program.registers[0..12],
            [150, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_halt_step() {
        let mut program = first_test_program();
        program.position = 8;
        program.step();
        assert_eq!(program.terminated, true);
        assert_eq!(program.position, 8);
        assert_eq!(
            program.registers[0..12],
            [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run_first_test_program() {
        let mut program = first_test_program();
        program.run();
        assert_eq!(program.terminated, true);
        assert_eq!(program.position, 8);
        assert_eq!(
            program.registers[0..12],
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(90));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }
}
