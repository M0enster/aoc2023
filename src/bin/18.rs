advent_of_code::solution!(18);

enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Direction::U,
            'D' => Direction::D,
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid direction: {}", c),
        }
    }
    fn from_char_digit(c: char) -> Self {
        match c {
            '0' => Direction::R,
            '1' => Direction::D,
            '2' => Direction::L,
            '3' => Direction::U,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

type Color = String;

struct Dig {
    direction: Direction,
    length: usize,
    color: Color,
}

impl Dig {
    fn new(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let direction =
            Direction::from_char(parts.next().unwrap().chars().next().unwrap());
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap().to_string();
        Self {
            direction,
            length,
            color,
        }
    }
    fn get_next_coords(&self, x: usize, y: usize) -> (usize, usize) {
        match self.direction {
            Direction::U => (x, y - self.length),
            Direction::D => (x, y + self.length),
            Direction::L => (x - self.length, y),
            Direction::R => (x + self.length, y),
        }
    }
    fn update_from_color(&mut self) {
        let chars = self.color.chars().collect::<Vec<char>>();
        self.direction = Direction::from_char_digit(chars[7]);
        let hex = self.color.chars().skip(2).take(5).collect::<String>();
        self.length = usize::from_str_radix(&hex, 16).unwrap_or_default();
    }
}

struct Plan {
    digs: Vec<Dig>,
}

impl Plan {
    fn new(input: &str) -> Self {
        Plan {
            digs: input.lines().map(|line| Dig::new(line)).collect(),
        }
    }
    fn get_bounds(&self) -> (usize, usize, usize, usize) {
        let mut min_x = std::i64::MAX;
        let mut max_x = std::i64::MIN;
        let mut min_y = std::i64::MAX;
        let mut max_y = std::i64::MIN;
        let (mut x, mut y) = (0i64, 0i64);
        for dig in &self.digs {
            match dig.direction {
                Direction::U => y -= dig.length as i64,
                Direction::D => y += dig.length as i64,
                Direction::L => x -= dig.length as i64,
                Direction::R => x += dig.length as i64,
            }
            min_x = std::cmp::min(min_x, x);
            max_x = std::cmp::max(max_x, x);
            min_y = std::cmp::min(min_y, y);
            max_y = std::cmp::max(max_y, y);
        }
        let start_pos = (min_x * -1, min_y * -1);
        let grid_size = (max_x - min_x + 1, max_y - min_y + 1);
        (
            start_pos.0 as usize,
            start_pos.1 as usize,
            grid_size.0 as usize,
            grid_size.1 as usize,
        )
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec!['.'; width]; height],
        }
    }
    fn execute_plan(&mut self, plan: &Plan, start_x: usize, start_y: usize) {
        let mut x = start_x;
        let mut y = start_y;
        for dig in &plan.digs {
            let (next_x, next_y) = dig.get_next_coords(x, y);
            for i in std::cmp::min(x, next_x)..=std::cmp::max(x, next_x) {
                self.grid[y][i] = '#';
            }
            for j in std::cmp::min(y, next_y)..=std::cmp::max(y, next_y) {
                self.grid[j][x] = '#';
            }
            x = next_x;
            y = next_y;
        }
    }
    fn flood_fill(&mut self, x: usize, y: usize) -> usize {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        let mut cnt = 0;
        while let Some((x, y)) = queue.pop_front() {
            if self.grid[y][x] == '.' {
                cnt += 1;
                self.grid[y][x] = ',';
                if x > 0 {
                    queue.push_back((x - 1, y));
                }
                if x < self.width - 1 {
                    queue.push_back((x + 1, y));
                }
                if y > 0 {
                    queue.push_back((x, y - 1));
                }
                if y < self.height - 1 {
                    queue.push_back((x, y + 1));
                }
            }
        }
        cnt
    }
    fn count_trench(&mut self) -> usize {
        let mut count = 0;
        for i in 0..self.width {
            if self.grid[0][i] == '.' {
                count += self.flood_fill(i, 0);
            }
            if self.grid[self.height - 1][i] == '.' {
                count += self.flood_fill(i, self.height - 1);
            }
        }
        for j in 0..self.height {
            if self.grid[j][0] == '.' {
                count += self.flood_fill(0, j);
            }
            if self.grid[j][self.width - 1] == '.' {
                count += self.flood_fill(self.width - 1, j);
            }
        }
        self.width * self.height - count
    }
    pub fn print(&self) {
        for row in &self.grid {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let plan = Plan::new(input);
    let (start_x, start_y, grid_width, grid_height) = plan.get_bounds();
    let mut grid = Grid::new(grid_width as usize, grid_height as usize);
    grid.execute_plan(&plan, start_x, start_y);
    Some(grid.count_trench() as u32)
}

fn calc_area(points: &[[i64; 2]]) -> u64 {
    let mut res = 0;
    let mut inner = 0;
    let mut outer = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        res += points[i][0] * points[j][1];
        res -= points[j][0] * points[i][1];
    }
    println!("res: {}", res);
    res >>= 1;
    res as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut plan = Plan::new(input);
    // plan.digs.iter_mut().for_each(|dig| dig.update_from_color());
    let (start_x, start_y, _, _) = plan.get_bounds();
    let (mut x, mut y) = (start_x + 1, start_y + 1);
    let mut points: Vec<[i64; 2]> = plan
        .digs
        .iter()
        .map(|dig| {
            (x, y) = dig.get_next_coords(x, y);
            [x as i64, y as i64]
        })
        .collect();
    points.reverse();
    Some(calc_area(&points))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }

    #[test]
    fn test_part_two_2() {
        let input = "R 1 #\nD 1 #\nL 1 #\nU 1 #\n";
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_3() {
        let input = "R 2 #\nD 2 #\nL 2 #\nU 2 #\n";
        let result = part_two(input);
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_4() {
        let input = "R 7 (#70c710)
        D 6 (#0dc571)
        L 2 (#5713f0)
        D 1 (#d2c081)
        R 2 (#59c680)
        D 3 (#411b91)
        L 6 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 3 (#caa171)
        R 2 (#7807d2)
        U 2 (#a77fa3)
        L 2 (#015232)
        U 3 (#7a21e3)";
        let result = part_two(input);
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_calc_area() {
        let points = [[1, 1], [2, 1], [2, 2], [1, 2]];
        assert_eq!(calc_area(&points), 4);
        // let points = [[1, 1], [3, 1], [3, 3], [1, 3]];
        // assert_eq!(calc_area(&points), 4);
    }
}
