advent_of_code::solution!(15);

fn hash(v: u32) -> u32 {
    v * 17 % 256
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.chars().fold(0, |acc: u32, c| hash(acc + c as u32)))
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v: Vec<Vec<(Vec<char>, usize)>> = vec![vec![]; 256];
    input.trim_end_matches('\n').split(',').for_each(|s| {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut fl: usize = 0;
        if s[s.len() - 1].is_numeric() {
            fl = s.pop().unwrap().to_digit(10).unwrap() as usize;
        }
        let op = s.pop().unwrap();
        let hash = s.iter().fold(0, |acc, c| hash(acc + *c as u32)) as usize;
        let i = v[hash].iter().position(|(w, _)| {
            w.len() == s.len() && w.iter().zip(s.iter()).all(|(a, b)| a == b)
        });

        if let Some(i) = i {
            if op == '=' {
                v[hash][i].1 = fl;
            } else {
                v[hash].remove(i);
            }
        } else if op == '=' {
            v[hash].push((s, fl));
        }
    });
    Some(v.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + v
            .iter()
            .enumerate()
            .fold(0, |acc, (j, (_, fl))| (i + 1) * (j + 1) * fl + acc)
            as u32
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
