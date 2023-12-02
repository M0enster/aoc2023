advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let a: Vec<&str> = line.split(": ").collect();
                let res = a[1].split("; ").fold(true, |abb, b| {
                    abb & b.split(", ").fold(true, |acc, c| {
                        let d: Vec<&str> = c.split_ascii_whitespace().collect();
                        acc & match d.get(1) {
                            Some(&"red") => 12 >= d[0].parse::<u32>().unwrap(),
                            Some(&"green") => 13 >= d[0].parse::<u32>().unwrap(),
                            Some(&"blue") => 14 >= d[0].parse::<u32>().unwrap(),
                            _ => false,
                        }
                    })
                });
                if res {
                    a[0].split_ascii_whitespace()
                        .collect::<Vec<&str>>()
                        .get(1)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut mm: [u32; 3] = [0; 3];
                let a: Vec<&str> = line.split(": ").collect();
                a[1].split("; ").for_each(|b| {
                    b.split(", ").for_each(|c| {
                        let d: Vec<&str> = c.split_ascii_whitespace().collect();
                        match d.get(1) {
                            Some(&"red") => {
                                mm[0] = std::cmp::max(mm[0], d[0].parse::<u32>().unwrap())
                            }
                            Some(&"green") => {
                                mm[1] = std::cmp::max(mm[1], d[0].parse::<u32>().unwrap())
                            }
                            Some(&"blue") => {
                                mm[2] = std::cmp::max(mm[2], d[0].parse::<u32>().unwrap())
                            }
                            _ => (),
                        };
                    });
                });
                mm[0] * mm[1] * mm[2]
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
