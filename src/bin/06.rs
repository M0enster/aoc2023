advent_of_code::solution!(6);

fn get_nums(input: &str) -> Vec<u32> {
    input
        .splitn(2, ':')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut section = input.lines();
    let times = get_nums(section.next()?);
    let distances = get_nums(section.next()?);
    let res = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            let mut cnt = 0;
            for i in 1..=*t {
                cnt += ((*t - i) * i > *d) as u32;
            }
            cnt
        })
        .reduce(|a, b| a * b)?;
    Some(res)
}

fn get_num(input: &str) -> u64 {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut section = input.lines();
    let time = get_num(section.next()?);
    let distance = get_num(section.next()?);
    let mut l = 1;
    while (time - l) * l <= distance {
        l += 1;
    }
    Some(time - l - l + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
