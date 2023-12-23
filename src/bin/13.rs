advent_of_code::solution!(13);

fn vertical_match(pattern: &Vec<Vec<char>>, start: usize) -> bool {
    for i in 0..pattern.len() {
        for (l, r) in ((0..start).rev()).zip(start..pattern[0].len()) {
            if pattern[i][l] != pattern[i][r] {
                return false;
            }
        }
    }
    true
}

fn horizontal_match(pattern: &Vec<Vec<char>>, start: usize) -> bool {
    for (l, r) in ((0..start).rev()).zip(start..pattern.len()) {
        for j in 0..pattern[0].len() {
            if pattern[l][j] != pattern[r][j] {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let pattern: Vec<Vec<char>> = pattern
                    .lines()
                    .map(|line| line.chars().collect())
                    .collect();
                for i in 1..pattern[0].len() {
                    match vertical_match(&pattern, i) {
                        true => {
                            return i as u32;
                        }
                        false => {}
                    }
                }
                for i in 1..pattern.len() {
                    match horizontal_match(&pattern, i) {
                        true => {
                            return i as u32 * 100;
                        }
                        false => {}
                    }
                }
                0
            })
            .sum(),
    )
}

fn vertical_match_two(pattern: &Vec<Vec<char>>, start: usize) -> bool {
    let mut smudge = false;
    for i in 0..pattern.len() {
        for (l, r) in ((0..start).rev()).zip(start..pattern[0].len()) {
            if pattern[i][l] != pattern[i][r] {
                if smudge {
                    return false;
                } else {
                    smudge = true;
                }
            }
        }
    }
    smudge
}

fn horizontal_match_two(pattern: &Vec<Vec<char>>, start: usize) -> bool {
    let mut smudge = false;
    for (l, r) in ((0..start).rev()).zip(start..pattern.len()) {
        for j in 0..pattern[0].len() {
            if pattern[l][j] != pattern[r][j] {
                if smudge {
                    return false;
                } else {
                    smudge = true;
                }
            }
        }
    }
    smudge
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let pattern: Vec<Vec<char>> = pattern
                    .lines()
                    .map(|line| line.chars().collect())
                    .collect();
                for i in 1..pattern[0].len() {
                    match vertical_match_two(&pattern, i) {
                        true => {
                            return i as u32;
                        }
                        false => {}
                    }
                }
                for i in 1..pattern.len() {
                    match horizontal_match_two(&pattern, i) {
                        true => {
                            return i as u32 * 100;
                        }
                        false => {}
                    }
                }
                0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
