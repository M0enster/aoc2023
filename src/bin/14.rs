use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for j in 0..grid[0].len() {
        let mut top = 0;
        for i in 0..grid.len() {
            match grid[i][j] {
                'O' => {
                    res += grid.len() - top;
                    top += 1;
                }
                '#' => {
                    top = i + 1;
                }
                _ => {}
            }
        }
    }
    Some(res as u32)
}

fn north(grid: &mut Vec<Vec<char>>) {
    for j in 0..grid[0].len() {
        let mut pos = 0;
        for i in 0..grid.len() {
            match grid[i][j] {
                'O' => {
                    if i != pos {
                        grid[pos][j] = 'O';
                        grid[i][j] = '.';
                    }
                    pos += 1;
                }
                '#' => {
                    pos = i + 1;
                }
                _ => {}
            }
        }
    }
}

fn south(grid: &mut Vec<Vec<char>>) {
    for j in 0..grid[0].len() {
        let mut pos = grid.len() - 1;
        for i in (0..grid.len()).rev() {
            match grid[i][j] {
                'O' => {
                    if i != pos {
                        grid[pos][j] = 'O';
                        grid[i][j] = '.';
                    }
                    pos -= if i > 0 { 1 } else { 0 };
                }
                '#' => {
                    pos = if i > 0 { i - 1 } else { 0 };
                }
                _ => {}
            }
        }
    }
}

fn west(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        let mut pos = 0;
        for j in 0..grid[0].len() {
            match grid[i][j] {
                'O' => {
                    if j != pos {
                        grid[i][pos] = 'O';
                        grid[i][j] = '.';
                    }
                    pos += 1;
                }
                '#' => {
                    pos = j + 1;
                }
                _ => {}
            }
        }
    }
}

fn east(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        let mut pos = grid[0].len() - 1;
        for j in (0..grid[0].len()).rev() {
            match grid[i][j] {
                'O' => {
                    if j != pos {
                        grid[i][pos] = 'O';
                        grid[i][j] = '.';
                    }
                    pos -= if j > 0 { 1 } else { 0 };
                }
                '#' => {
                    pos = if j > 0 { j - 1 } else { 0 };
                }
                _ => {}
            }
        }
    }
}

fn count(grid: &Vec<Vec<char>>) -> u32 {
    let mut res = 0;
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                res += grid.len() - i;
            }
        }
    }
    res as u32
}

type GridFunction = dyn Fn(&mut Vec<Vec<char>>);

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cycle_tot = 1000000000;

    let cycle: Vec<&GridFunction> = vec![&north, &west, &south, &east];

    let mut dp: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    for cnt in 0..cycle_tot {
        for f in &cycle {
            f(&mut grid);
        }
        if let Some(i) = dp.get(&grid) {
            for _ in 0..(cycle_tot - cnt - 1) % (cnt - i) {
                for f in &cycle {
                    f(&mut grid);
                }
            }
            break;
        } else {
            dp.insert(grid.clone(), cnt);
        }
    }

    Some(count(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
