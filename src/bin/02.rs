advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().for_each(|line| {
        let a: Vec<&str> = line.split(": ").collect();
        a[1].split("; ").for_each(|b| {
            b.split(", ").for_each(|c| {
                let d: Vec<&str> = c.split_ascii_whitespace().collect();
                match d.get(1) {
                    Some(&"red") => println!("red"),
                    Some(&"green") => println!("green"),
                    Some(&"blue") => println!("blue"),
                    _ => println!("not red"),
                }
            });
        });
    });
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
