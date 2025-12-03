advent_of_code::solution!(2);

fn get_values(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|v| {
            let pair: Vec<u64> = v.split('-').map(|n| n.parse::<u64>().unwrap()).collect();
            (pair[0], pair[1])
        })
        .collect()
}

fn is_valid(number: u64) -> bool {
    let string_number = number.to_string();
    let length = string_number.len();
    if (length % 2) == 1 {
        return true;
    }
    let half_length = length / 2;
    let right_half = string_number.get(half_length..).unwrap();
    let right_number = right_half.parse::<u64>().unwrap();

    number != right_number + right_number * 10u64.pow(half_length as u32)
}

fn is_valid2(number: u64) -> bool {
    let string_number = number.to_string();
    let length = string_number.len();
    for sub_length in 1..=(length / 2) {
        if length.is_multiple_of(sub_length) {
            let substring = string_number.get(..sub_length).unwrap();

            let mut is_same = true;
            for index in 0..length / sub_length {
                is_same &= string_number
                    .get(index * sub_length..(index + 1) * sub_length)
                    .unwrap()
                    == substring;
            }

            if is_same {
                return false;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;

    let values = get_values(input);
    for (left, right) in values {
        for number in left..=right {
            if !is_valid(number) {
                result += number;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;

    let values = get_values(input);
    for (left, right) in values {
        for number in left..=right {
            if !is_valid2(number) {
                result += number;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
