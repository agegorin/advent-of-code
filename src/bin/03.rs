advent_of_code::solution!(3);

fn get_joltage(input: &str, length: usize) -> u64 {
    let mut max_element: (usize, u64) = (0, 0);
    let full_length = input.len();
    let enumerated_numbers = input
        .get(..(full_length - length + 1))
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .enumerate();

    for (i, number) in enumerated_numbers.clone() {
        if number > max_element.1 {
            max_element = (i, number);
            if number == 9 {
                break;
            }
        }
    }

    let mut add_part = 0;
    if length > 1 {
        add_part = get_joltage(input.get((max_element.0 + 1)..).unwrap(), length - 1);
    }

    max_element.1 * 10u64.pow(length as u32 - 1) + add_part
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| get_joltage(line, 2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| get_joltage(line, 12)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
