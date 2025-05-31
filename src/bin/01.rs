advent_of_code::solution!(1);

fn fuel_required(mass: u32, recursive: bool) -> u32 {
    let fuel = (mass / 3).saturating_sub(2);
    if recursive && fuel > 0 {
        fuel + fuel_required(fuel, true)
    } else {
        fuel
    }
}

fn total_fuel_required(input: &str, recursive: bool) -> Option<u32> {
    let (total, errors) = input.lines().fold((0, 0), |(total, errors), line| {
        line.parse().map_or_else(
            |_| (total, errors + 1),
            |mass| (total + fuel_required(mass, recursive), errors),
        )
    });
    if errors > 0 {
        None
    } else {
        Some(total)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    total_fuel_required(input, false)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    total_fuel_required(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12, false), 2);
        assert_eq!(fuel_required(14, false), 2);
        assert_eq!(fuel_required(1969, false), 654);
        assert_eq!(fuel_required(100_756, false), 33_583);
    }

    #[test]
    fn test_fuel_required_recursive() {
        assert_eq!(fuel_required(12, true), 2);
        assert_eq!(fuel_required(14, true), 2);
        assert_eq!(fuel_required(1969, true), 966);
        assert_eq!(fuel_required(100_756, true), 50_346);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34_241));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51_316));
    }
}
