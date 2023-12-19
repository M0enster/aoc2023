advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines();
    let res: i64 = lines
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut lasts: Vec<i64> = Vec::new();
            lasts.push(nums.last().unwrap().clone());
            while nums.len() > 1 {
                let mut a_z: bool = true;
                nums = nums
                    .windows(2)
                    .map(|pair| {
                        let x = pair[1] - pair[0];
                        a_z = a_z && (x == 0);
                        x
                    })
                    .collect();
                if a_z {
                    break;
                }
                lasts.push(nums.last().unwrap().clone());
            }
            lasts.iter().fold(0, |a, b| a + b)
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.lines();
    let res: i64 = lines
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut firsts: Vec<i64> = Vec::new();
            firsts.push(nums.first().unwrap().clone());
            while nums.len() > 1 {
                let mut a_z: bool = true;
                nums = nums
                    .windows(2)
                    .map(|pair| {
                        let x = pair[1] - pair[0];
                        a_z = a_z && (x == 0);
                        x
                    })
                    .collect();
                if a_z {
                    break;
                }
                firsts.push(nums.first().unwrap().clone());
            }
            firsts.iter().rev().fold(0, |a, b| b - a)
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
