use advent_of_code::intcode::{Memory, ParseMemoryError};
use std::str::FromStr;

advent_of_code::main!(2);

#[derive(Debug)]
struct GravityAssistProgram {
    memory: Memory,
}

impl GravityAssistProgram {
    fn execute(&self, noun: usize, verb: usize) -> usize {
        let mut memory = self.memory.clone();
        memory.set_register(1, noun);
        memory.set_register(2, verb);
        let closing_mem = memory.run();
        closing_mem.read_register(0)
    }

    fn find_noun_and_verb(&self, target: usize) -> Option<(usize, usize)> {
        for noun in 0..=100 {
            for verb in 0..=100 {
                if self.execute(noun, verb) == target {
                    return Some((noun, verb));
                }
            }
        }
        None
    }
}

impl FromStr for GravityAssistProgram {
    type Err = ParseMemoryError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let memory = text.parse()?;
        Ok(Self { memory })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(program) = input.parse::<GravityAssistProgram>() {
        Some(program.execute(12, 2))
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(program) = input.parse::<GravityAssistProgram>() {
        program
            .find_noun_and_verb(19690720)
            .map(|(noun, verb)| (100 * noun) + verb)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gravity_assist_program() {
        let parsed = advent_of_code::template::read_file("examples", 2)
            .parse::<GravityAssistProgram>()
            .unwrap();
        let expected: [usize; 12] = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        for (register, value) in expected.iter().enumerate() {
            assert_eq!(
                parsed.memory.read_register(register),
                *value,
                "register {register} contains value {value}"
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(100));
    }
}
