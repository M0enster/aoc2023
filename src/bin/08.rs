use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    lines.next();
    let mut map = HashMap::new();
    lines.for_each(|l| {
        let parts: Vec<&str> =
            l.split(&[' ', '=', '(', ')', ','][..]).collect();
        let parts: Vec<&str> =
            parts.into_iter().filter(|s| !s.is_empty()).collect();

        let v = parts[0];
        let l = parts[1];
        let r = parts[2];
        map.insert(v, [l, r]);
    });
    let mut current = "AAA";
    let mut i: usize = 0;
    let mut res = 0;
    while current != "ZZZ" {
        current = map[&current][instructions[i]];
        i = (i + 1) % instructions.len();
        res += 1;
    }
    Some(res)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    lines.next();
    let mut map = HashMap::new();
    let mut nodes: Vec<&str> = Vec::new();
    lines.for_each(|l| {
        let parts: Vec<&str> =
            l.split(&[' ', '=', '(', ')', ','][..]).collect();
        let parts: Vec<&str> =
            parts.into_iter().filter(|s| !s.is_empty()).collect();

        let v = parts[0];
        let l = parts[1];
        let r = parts[2];
        map.insert(v, [l, r]);
        if v.ends_with('A') {
            nodes.push(v);
        }
    });
    let mut dist: Vec<u64> = Vec::new();
    for node in &nodes {
        let mut current = *node;
        let mut j: usize = 0;
        let mut cnt = 0;
        while !current.ends_with('Z') {
            cnt += 1;
            current = map[&current][instructions[j]];
            j = (j + 1) % instructions.len();
        }
        dist.push(cnt);
    }
    let res = dist.iter().fold(1, |acc, x| lcm(acc, *x));
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
