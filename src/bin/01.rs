advent_of_code::solution!(1);

const START_POSITION: i64 = 50;

fn up_to(number: i64) -> (i64, usize) {
    let mut counter = 0;
    let mut value = number;

    while value < 0 {
        value += 100;
        counter += 1;
    }

    (value, counter)
}

fn down_to(number: i64) -> (i64, usize) {
    let mut counter = 0;
    let mut value = number;

    while value >= 100 {
        value -= 100;
        counter += 1;
    }

    (value, counter)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut counter = 0;

    input.lines().fold(START_POSITION, |acc, line| {
        let (letter, number_string) = line.split_at(1);
        let number = number_string.parse::<i64>().unwrap();
        let result = match letter {
            "R" => (acc + number) % 100,
            "L" => (acc - number) % 100,
            _ => panic!(),
        };
        if result == 0 {
            counter += 1;
        }
        result
    });

    Some(counter)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut counter = 0;

    input.lines().fold(START_POSITION, |acc, line| {
        let (letter, number_string) = line.split_at(1);
        let number = number_string.parse::<i64>().unwrap();
        let (result, rounds) = match letter {
            "R" => down_to(acc + number),
            "L" => up_to(acc - number),
            _ => panic!(),
        };
        counter += rounds;
        result
    });

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
