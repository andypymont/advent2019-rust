struct ParseMassesError;

fn parse_masses(input: &str) -> Result<Vec<u32>, ParseMassesError> {
    let mut masses = Vec::new();

    for line in input.lines() {
        let mass = line.parse::<u32>().map_err(|_| ParseMassesError)?;
        masses.push(mass);
    }

    Ok(masses)
}

fn required_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn required_fuel_recursive(mass: u32) -> u32 {
    let fuel = (mass / 3).saturating_sub(2);

    match fuel {
        0 => 0,
        _ => fuel + required_fuel_recursive(fuel),
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    match parse_masses(input) {
        Ok(masses) => Some(masses.iter().map(|f| required_fuel(*f)).sum()),
        Err(_) => None,
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    match parse_masses(input) {
        Ok(masses) => Some(masses.iter().map(|f| required_fuel_recursive(*f)).sum()),
        Err(_) => None,
    }
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_12_requires_2() {
        assert_eq!(required_fuel(12), 2);
    }

    #[test]
    fn test_mass_14_requires_2() {
        assert_eq!(required_fuel(14), 2);
    }

    #[test]
    fn test_mass_1969_requires_654() {
        assert_eq!(required_fuel(1969), 654);
    }

    #[test]
    fn test_mass_100756_requires_33583() {
        assert_eq!(required_fuel(100_756), 33_583);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(34_241));
    }

    #[test]
    fn test_mass_14_requires_2_recursively() {
        assert_eq!(required_fuel_recursive(14), 2);
    }

    #[test]
    fn test_mass_1969_requires_966() {
        assert_eq!(required_fuel_recursive(1969), 966);
    }

    #[test]
    fn test_mass_100756_requires_50346() {
        assert_eq!(required_fuel_recursive(100_756), 50_346);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(51_316));
    }
}
