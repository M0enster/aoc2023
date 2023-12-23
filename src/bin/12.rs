advent_of_code::solution!(12);

fn dfs(
    sections: &Vec<char>,
    counts: &Vec<u64>,
    dp: &mut Vec<Vec<Vec<Option<u64>>>>,
    sec_i: usize,
    cnt_i: usize,
    cur_cnt: u64,
) -> u64 {
    if sec_i >= sections.len() {
        if cnt_i >= counts.len()
            || (cnt_i + 1 == counts.len() && cur_cnt == counts[cnt_i])
        {
            return 1;
        } else {
            return 0;
        }
    }

    if cnt_i < counts.len() && dp[sec_i][cnt_i][cur_cnt as usize].is_some() {
        return dp[sec_i][cnt_i][cur_cnt as usize].unwrap();
    }

    if cnt_i >= counts.len() {
        if sections[sec_i] == '#' {
            return 0;
        } else {
            return dfs(sections, counts, dp, sec_i + 1, cnt_i, cur_cnt);
        }
    }

    if sections[sec_i] == '.' {
        if cur_cnt == 0 {
            return dfs(sections, counts, dp, sec_i + 1, cnt_i, cur_cnt);
        } else if cur_cnt > 0 && counts[cnt_i] - cur_cnt != 0 {
            return 0;
        } else {
            return dfs(sections, counts, dp, sec_i + 1, cnt_i + 1, 0);
        }
    }
    if sections[sec_i] == '#' && cur_cnt == counts[cnt_i] {
        return 0;
    }
    if sections[sec_i] == '?' && cur_cnt == counts[cnt_i] {
        return dfs(sections, counts, dp, sec_i + 1, cnt_i + 1, 0);
    }

    let mut res = 0;

    res += dfs(sections, counts, dp, sec_i + 1, cnt_i, cur_cnt + 1);

    if sections[sec_i] == '?' && cur_cnt == 0 {
        res += dfs(sections, counts, dp, sec_i + 1, cnt_i, cur_cnt);
    }
    dp[sec_i][cnt_i][cur_cnt as usize] = Some(res);
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let res: u64 = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let sections = parts.next().unwrap().chars().collect::<Vec<char>>();
            let counts = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let max_value: usize = *counts.iter().max().unwrap_or(&0) as usize;
            let mut dp: Vec<Vec<Vec<Option<u64>>>> =
                vec![
                    vec![vec![None; max_value + 1]; counts.len() + 1];
                    sections.len() + 1
                ];
            dfs(&sections, &counts, &mut dp, 0, 0, 0)
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let res: u64 = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let sections = parts.next().unwrap().chars().collect::<Vec<char>>();
            let mut expanded_sections = sections.clone();
            for _ in 0..4 {
                expanded_sections.push('?');
                expanded_sections.extend(sections.clone());
            }
            let counts = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let mut expanded_counts = counts.clone();
            for _ in 0..4 {
                expanded_counts.extend(counts.clone());
            }
            let max_value: usize = *counts.iter().max().unwrap_or(&0) as usize;
            let mut dp: Vec<Vec<Vec<Option<u64>>>> =
                vec![
                    vec![vec![None; max_value + 1]; expanded_counts.len() + 1];
                    expanded_sections.len() + 1
                ];
            dfs(&expanded_sections, &expanded_counts, &mut dp, 0, 0, 0)
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_one_2() {
        assert_eq!(part_one(".??..??.?.?##. 1,1,3"), Some(8));
    }

    #[test]
    fn test_part_one_3() {
        assert_eq!(part_one(".??..??.?.?##. 1,2,3"), Some(2));
    }

    #[test]
    fn test_part_one_4() {
        assert_eq!(part_one("?###???????? 3,2,1"), Some(10));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_part_two_2() {
        assert_eq!(part_two(".??..??...?##. 1,1,3"), Some(16384));
    }
}
