advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut cols = vec![true; lines[0].len()];
    let mut rows_y = 0;
    let mut pos: Vec<[usize; 2]> = Vec::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        let mut rows = true;
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                pos.push([x, y + rows_y]);
                cols[x] = false;
                rows = false;
            }
        });
        rows_y += rows as usize;
    });
    pos.sort_unstable();
    let mut cols_x = 0;
    let mut j = 0;
    pos = pos
        .iter()
        .map(|[x, y]| {
            while j < *x {
                if cols[j] {
                    cols_x += 1;
                }
                j += 1;
            }
            [*x + cols_x, *y]
        })
        .collect();
    let mut sum = 0;
    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            sum += (pos[i][0] as i32 - pos[j][0] as i32).abs()
                + (pos[i][1] as i32 - pos[j][1] as i32).abs();
        }
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    #[cfg(test)]
    let expansion = 10 - 1;
    #[cfg(not(test))]
    let expansion = 1000000 - 1;

    let lines = input.lines().collect::<Vec<_>>();
    let mut cols = vec![true; lines[0].len()];
    let mut rows_y = 0;
    let mut pos: Vec<[usize; 2]> = Vec::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        let mut rows = true;
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                pos.push([x, y + rows_y]);
                cols[x] = false;
                rows = false;
            }
        });
        rows_y += if rows { expansion } else { 0 };
    });
    pos.sort_unstable();
    let mut cols_x = 0;
    let mut j = 0;
    pos = pos
        .iter()
        .map(|[x, y]| {
            while j < *x {
                if cols[j] {
                    cols_x += expansion;
                }
                j += 1;
            }
            [*x + cols_x, *y]
        })
        .collect();
    let mut sum = 0;
    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            sum += (pos[i][0] as i64 - pos[j][0] as i64).abs()
                + (pos[i][1] as i64 - pos[j][1] as i64).abs();
        }
    }
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
