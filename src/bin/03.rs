advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut a: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum: u32 = 0;
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i][j] == '.' || a[i][j].is_numeric() {
                continue;
            }
            if i > 0 && j > 0 {
                sum += get_num(&mut a, i - 1, j - 1);
            }
            if i > 0 {
                sum += get_num(&mut a, i - 1, j);
                sum += get_num(&mut a, i - 1, j + 1);
            }
            if j > 0 {
                sum += get_num(&mut a, i, j - 1);
                sum += get_num(&mut a, i + 1, j - 1);
            }
            sum += get_num(&mut a, i, j + 1);
            sum += get_num(&mut a, i + 1, j);
            sum += get_num(&mut a, i + 1, j + 1);
        }
    }
    Some(sum)
}

fn get_num(a: &mut Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i >= a.len() || j >= a[i].len() || !a[i][j].is_numeric() {
        return 0;
    }
    let mut l = j;
    let mut r = j;
    while l > 0 && a[i][l - 1].is_numeric() {
        l -= 1;
    }
    while r + 1 < a[i].len() && a[i][r + 1].is_numeric() {
        r += 1;
    }
    let res = a[i][l..=r]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    for k in l..=r {
        a[i][k] = '.';
    }
    res
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut a: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum: u32 = 0;
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i][j] != '*' {
                continue;
            }
            let mut b: Vec<(usize, usize, usize)> = Vec::new();

            if i > 0 {
                if a[i - 1][j].is_numeric() {
                    get_num_one(&a, &mut b, i - 1, j);
                } else {
                    if j > 0 {
                        get_num_one(&a, &mut b, i - 1, j - 1);
                    }
                    get_num_one(&a, &mut b, i - 1, j + 1);
                }
            }
            if j > 0 {
                get_num_one(&a, &mut b, i, j - 1);
            }
            get_num_one(&a, &mut b, i, j + 1);

            if a[i + 1][j].is_numeric() {
                get_num_one(&a, &mut b, i + 1, j);
            } else {
                if j > 0 {
                    get_num_one(&a, &mut b, i + 1, j - 1);
                }
                get_num_one(&a, &mut b, i + 1, j + 1);
            }

            if b.len() == 2 {
                sum += a[b[0].0][b[0].1..=b[0].2]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
                    * a[b[1].0][b[1].1..=b[1].2]
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                for k in b[0].1..=b[0].2 {
                    a[b[0].0][k] = '.';
                }
                for k in b[1].1..=b[1].2 {
                    a[b[1].0][k] = '.';
                }
            }
        }
    }
    Some(sum)
}

fn get_num_one(a: &Vec<Vec<char>>, b: &mut Vec<(usize, usize, usize)>, i: usize, j: usize) {
    if let Some(q) = get_num_two(a, i, j) {
        b.push(q);
    }
}

fn get_num_two(a: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize, usize)> {
    if i >= a.len() || j >= a[i].len() || !a[i][j].is_numeric() {
        return None;
    }
    let mut l = j;
    let mut r = j;
    while l > 0 && a[i][l - 1].is_numeric() {
        l -= 1;
    }
    while r + 1 < a[i].len() && a[i][r + 1].is_numeric() {
        r += 1;
    }
    Some((i, l, r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
