advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let a: Vec<&str> = line.splitn(2, ": ").collect();
                let b: Vec<&str> = a[1].splitn(2, " | ").collect();
                let win: Vec<u32> = b[0]
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let res = b[1].split_ascii_whitespace().fold(0, |acc, x| {
                    acc + (win.contains(&x.parse::<u32>().unwrap()) as u32)
                });
                if res > 0 {
                    2u32.pow(res - 1)
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans: Vec<u32> = vec![1; input.lines().count()];
    input.lines().enumerate().for_each(|(i, line)| {
        let a: Vec<&str> = line.splitn(2, ": ").collect();
        let b: Vec<&str> = a[1].splitn(2, " | ").collect();
        let win: Vec<u32> = b[0]
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let res = b[1].split_ascii_whitespace().fold(0, |acc, x| {
            acc + (win.contains(&x.parse::<u32>().unwrap()) as usize)
        });
        for j in 1..=res {
            ans[i + j] += ans[i];
        }
    });
    Some(ans.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
