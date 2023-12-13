advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut section = input.split("\n\n");
    let mut f = section
        .next()?
        .splitn(2, ": ")
        .collect::<Vec<&str>>()
        .get(1)?
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut new_f = vec![0; f.len()];
    section.for_each(|s| {
        f.sort_unstable();
        new_f.clone_from_slice(&f);
        s.lines().skip(1).for_each(|l| {
            if let [a, b, c] = l
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()[..]
            {
                let mut i = f.partition_point(|&x| x < b);
                while i < f.len() && f[i] >= b && f[i] < b + c {
                    new_f[i] = a + (f[i] - b);
                    i += 1;
                }
            }
        });
        std::mem::swap(&mut f, &mut new_f);
    });
    Some(*f.iter().min()?)
}

fn pros_line(input: &str, f: &mut Vec<[u64; 2]>, new_f: &mut Vec<[u64; 2]>) {
    if let [a, l, c] = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()[..]
    {
        let r = l + c - 1;
        // println!(" {} {} {} {}", a, l, c, r);
        for i in (0..f.len()).rev() {
            if f[i][0] <= r && f[i][1] >= l {
                let new_l = std::cmp::max(f[i][0], l) + a - l;
                let new_r = std::cmp::min(f[i][1], r) + a - l;
                new_f.push([new_l, new_r]);
                if f[i][0] < l {
                    if f[i][1] > r {
                        f.push([r + 1, f[i][1]]);
                    }
                    f[i] = [f[i][0], l - 1];
                } else if f[i][1] > r {
                    f[i] = [r + 1, f[i][1]];
                } else {
                    f[i] = [0, 0];
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut section = input.split("\n\n");
    let mut bit = false;
    let mut f: Vec<[u64; 2]> = Vec::new();
    section
        .next()?
        .splitn(2, ": ")
        .collect::<Vec<&str>>()
        .get(1)?
        .split_whitespace()
        .for_each(|x| {
            if bit {
                f.last_mut().unwrap()[1] =
                    f.last().unwrap()[0] + x.parse::<u64>().unwrap() - 1;
            } else {
                f.push([x.parse::<u64>().unwrap(), 0]);
            }
            bit = !bit;
        });
    let mut new_f: Vec<[u64; 2]> = Vec::new();
    section.for_each(|s| {
        // println!("{:?}", f);
        s.lines()
            .skip(1)
            .for_each(|l| pros_line(l, &mut f, &mut new_f));
        f.iter().for_each(|x| {
            if x[1] > 0 {
                new_f.push(*x);
            }
        });
        std::mem::swap(&mut f, &mut new_f);
        new_f.clear();
    });
    // println!("{:?}", f);
    Some(f.iter().fold(std::u64::MAX, |a, x| std::cmp::min(a, x[0])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
