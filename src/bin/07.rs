advent_of_code::solution!(7);

fn tl(c: char, b: bool) -> char {
    match c {
        'T' => '\x0a',
        'J' => {
            if b {
                '\x01'
            } else {
                '\x0b'
            }
        }
        'Q' => '\x0c',
        'K' => '\x0d',
        'A' => '\x0e',
        _ => ((c as u8) - b'0') as char,
    }
}

fn get_val(cards: &[char]) -> u32 {
    let mut cnt = [0; 15];
    cards.iter().for_each(|c| {
        cnt[*c as usize] += 1;
    });
    cnt.sort_unstable_by(|a, b| b.cmp(a));
    if cnt[0] == 5 {
        6
    } else if cnt[0] == 4 {
        5
    } else if cnt[0] == 3 && cnt[1] == 2 {
        4
    } else if cnt[0] == 3 {
        3
    } else if cnt[0] == 2 && cnt[1] == 2 {
        2
    } else if cnt[0] == 2 {
        1
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let cards = parts
                .next()
                .unwrap()
                .chars()
                .map(|x| tl(x, false))
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap();
            (get_val(&cards), cards, bid)
        })
        .collect::<Vec<_>>();
    lines.sort_unstable_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let res = lines
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, (_, _, bid))| {
            acc + ((i + 1) as u64) * bid.parse::<u64>().unwrap()
        });
    Some(res)
}

fn get_val_j(cards: &[char]) -> u32 {
    let mut cnt = [0; 15];
    let mut j = 0;
    cards.iter().for_each(|c| {
        let n = *c as usize;
        if n == 1 {
            j += 1;
        } else {
            cnt[*c as usize] += 1;
        }
    });
    cnt.sort_unstable_by(|a, b| b.cmp(a));
    cnt[0] += j;
    if cnt[0] == 5 {
        6
    } else if cnt[0] == 4 {
        5
    } else if cnt[0] == 3 && cnt[1] == 2 {
        4
    } else if cnt[0] == 3 {
        3
    } else if cnt[0] == 2 && cnt[1] == 2 {
        2
    } else if cnt[0] == 2 {
        1
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let cards = parts
                .next()
                .unwrap()
                .chars()
                .map(|x| tl(x, true))
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap();
            (get_val_j(&cards), cards, bid)
        })
        .collect::<Vec<_>>();
    lines.sort_unstable_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let res = lines
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, (_, _, bid))| {
            acc + ((i + 1) as u64) * bid.parse::<u64>().unwrap()
        });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
