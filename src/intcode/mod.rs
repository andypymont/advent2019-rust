use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Halt,
}

impl Instruction {
    #[must_use]
    pub fn get_output_register(&self) -> Option<usize> {
        match self {
            Instruction::Add(_, _, x) | Instruction::Multiply(_, _, x) => Some(*x),
            Instruction::Halt => None,
        }
    }

    #[must_use]
    pub fn get_output_value(&self, mem: &Memory) -> usize {
        match self {
            Instruction::Add(a, b, _) => mem.read_register(*a) + mem.read_register(*b),
            Instruction::Multiply(a, b, _) => mem.read_register(*a) * mem.read_register(*b),
            Instruction::Halt => 0,
        }
    }

    #[must_use]
    pub fn get_register_change(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) | Instruction::Multiply(_, _, _) => 4,
            Instruction::Halt => 0,
        }
    }
}

const MEMORY_SIZE: usize = 200;

#[derive(Copy, Clone, Debug)]
pub struct Memory([usize; MEMORY_SIZE]);

impl Memory {
    #[must_use]
    pub fn read_register(&self, pos: usize) -> usize {
        if pos > MEMORY_SIZE {
            0
        } else {
            self.0[pos]
        }
    }

    pub fn set_register(&mut self, pos: usize, value: usize) {
        self.0[pos] = value;
    }

    #[must_use]
    pub fn read_instruction(&self, pos: usize) -> Instruction {
        match self.read_register(pos) {
            1 => Instruction::Add(
                self.read_register(pos + 1),
                self.read_register(pos + 2),
                self.read_register(pos + 3),
            ),
            2 => Instruction::Multiply(
                self.read_register(pos + 1),
                self.read_register(pos + 2),
                self.read_register(pos + 3),
            ),
            _ => Instruction::Halt,
        }
    }

    #[must_use]
    pub fn run(&self) -> Self {
        let mut mem = *self;
        let mut pos = 0;
        loop {
            let instruction = mem.read_instruction(pos);
            if let Instruction::Halt = instruction {
                break;
            }
            pos += instruction.get_register_change();
            mem += instruction;
        }
        mem
    }
}

impl Add<Instruction> for Memory {
    type Output = Self;

    fn add(self, rhs: Instruction) -> Self::Output {
        let mut registers = self.0;
        if let Some(register) = rhs.get_output_register() {
            registers[register] = rhs.get_output_value(&self);
        }
        Memory(registers)
    }
}

impl AddAssign<Instruction> for Memory {
    fn add_assign(&mut self, rhs: Instruction) {
        *self = *self + rhs;
    }
}

#[derive(Debug)]
pub struct ParseMemoryError;

impl FromStr for Memory {
    type Err = ParseMemoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut registers = [0; MEMORY_SIZE];
        for (ix, value_str) in s.trim().split(',').enumerate() {
            let value = value_str.parse().map_err(|_| ParseMemoryError)?;
            registers[ix] = value;
        }

        Ok(Self(registers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_memory() -> Memory {
        let mut mem = [0; MEMORY_SIZE];
        for (ix, value) in [1, 2, 4, 0, 2, 1, 3, 3, 99].iter().enumerate() {
            mem[ix] = *value;
        }
        Memory(mem)
    }

    #[test]
    fn test_example_memory() {
        assert_eq!(example_memory().0[0..9], [1, 2, 4, 0, 2, 1, 3, 3, 99]);
    }

    #[test]
    fn test_parse_memory() {
        let parsed: Memory = "1,2,4,0,2,1,3,3,99"
            .parse()
            .expect("Could not parse example program");
        assert_eq!(parsed.0[0..9], [1, 2, 4, 0, 2, 1, 3, 3, 99])
    }

    #[test]
    fn test_read_register() {
        let mem = example_memory();
        assert_eq!(mem.read_register(0), 1);
        assert_eq!(mem.read_register(4), 2);
        assert_eq!(mem.read_register(8), 99);
        assert_eq!(mem.read_register(20), 0);
    }

    #[test]
    fn test_read_instruction_add() {
        let mem = example_memory();
        assert_eq!(mem.read_instruction(0), Instruction::Add(2, 4, 0));
    }

    #[test]
    fn test_read_instruction_multiply() {
        let mem = example_memory();
        assert_eq!(mem.read_instruction(4), Instruction::Multiply(1, 3, 3));
    }

    #[test]
    fn test_read_instruction_halt() {
        let mem = example_memory();
        assert_eq!(mem.read_instruction(8), Instruction::Halt);
    }

    #[test]
    fn test_execute_instruction_add() {
        let mem = example_memory();

        let add = Instruction::Add(0, 2, 0);
        let after = mem + add;
        assert_eq!(after.0[0..9], [5, 2, 4, 0, 2, 1, 3, 3, 99]);

        let add = Instruction::Add(4, 8, 1);
        let after = mem + add;
        assert_eq!(after.0[0..9], [1, 101, 4, 0, 2, 1, 3, 3, 99]);
    }

    #[test]
    fn test_execute_instruction_multiply() {
        let mem = example_memory();

        let mul = Instruction::Multiply(1, 2, 0);
        let after = mem + mul;
        assert_eq!(after.0[0..9], [8, 2, 4, 0, 2, 1, 3, 3, 99]);

        let mul = Instruction::Multiply(1, 3, 4);
        let after = mem + mul;
        assert_eq!(after.0[0..9], [1, 2, 4, 0, 0, 1, 3, 3, 99]);
    }

    #[test]
    fn test_get_register_change() {
        let add = Instruction::Add(1, 2, 3);
        assert_eq!(add.get_register_change(), 4);

        let mul = Instruction::Multiply(1, 2, 3);
        assert_eq!(mul.get_register_change(), 4);

        let halt = Instruction::Halt;
        assert_eq!(halt.get_register_change(), 0);
    }

    #[test]
    fn test_run_program() {
        let mem = example_memory();
        let after = mem.run();
        assert_eq!(after.0[0..9], [6, 2, 4, 0, 2, 1, 3, 3, 99]);
    }
}
