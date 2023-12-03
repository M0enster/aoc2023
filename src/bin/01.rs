advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .filter_map(|line| {
            let f = line.chars().find(|c| c.is_numeric())?.to_digit(10)?;
            let l = line.chars().rev().find(|c| c.is_numeric())?.to_digit(10)?;
            Some(f * 10 + l)
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sl = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum = input
        .lines()
        .filter_map(|line| {
            let mut f = 0;
            let mut l = 0;

            for (i, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    f = c.to_digit(10)?;
                    break;
                } else if let Some(x) = sl.iter().position(|&x| line[i..].starts_with(x)) {
                    f = x as u32 + 1;
                    break;
                }
            }
            for (i, c) in line.chars().rev().enumerate() {
                if c.is_numeric() {
                    l = c.to_digit(10)?;
                    break;
                } else if let Some(x) = sl
                    .iter()
                    .position(|&x| line[line.len() - i - 1..].starts_with(x))
                {
                    l = x as u32 + 1;
                    break;
                }
            }
            Some(f * 10 + l)
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
