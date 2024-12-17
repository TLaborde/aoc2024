use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'E')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    (grid, start, end)
}

fn bfs(
    grid: &[Vec<char>],
    start: (i32, i32),
    initial_direction: (i32, i32),
) -> HashMap<((i32, i32), (i32, i32)), i32> {
    let mut dp = HashMap::new();
    dp.insert((start, initial_direction), 0);
    let mut queue = VecDeque::new();
    queue.push_back((start, initial_direction));

    while let Some((pos, direction)) = queue.pop_front() {
        for &(i, j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (pos.0 + i, pos.1 + j);
            if new_pos.0 < 0
                || new_pos.0 >= grid.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= grid[0].len() as i32
                || grid[new_pos.0 as usize][new_pos.1 as usize] == '#'
            {
                continue;
            }
            let new_direction = (i, j);
            let new_steps = dp[&(pos, direction)]
                + 1
                + 1000 * (direction.0 * new_direction.1 - direction.1 * new_direction.0).abs();
            if !dp.contains_key(&(new_pos, new_direction))
                || new_steps < dp[&(new_pos, new_direction)]
            {
                dp.insert((new_pos, new_direction), new_steps);
                queue.push_back((new_pos, new_direction));
            }
            let new_steps = dp[&(pos, direction)]
                + 1000 * (direction.0 * new_direction.1 - direction.1 * new_direction.0).abs();
            if !dp.contains_key(&(pos, new_direction))
                || new_steps < dp[&(pos, new_direction)]
            {
                dp.insert((pos, new_direction), new_steps);
            }
        }
    }
    dp
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(input);
    let initial_direction = (0, 1);
    let dp = bfs(&grid, start, initial_direction);

    dp.iter()
        .filter(|&(&(pos, _), _)| pos == end)
        .map(|(_, &steps)| steps)
        .min()
        .map(|steps| steps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(input);
    let initial_direction = (0, 1);

    let dp_start = bfs(&grid, start, initial_direction);
    let dp_end = bfs(&grid, end, initial_direction);

    let total_steps = dp_start
        .iter()
        .filter_map(|(&(pos, direction), &steps)| {
            dp_end.get(&(pos, (-direction.0, -direction.1)))
                .map(|&steps2| steps + steps2)
        })
        .min()
        .unwrap_or(i32::MAX);

    let good_positions: HashSet<_> = dp_start
        .iter()
        .filter(|&(&(pos, direction), &steps)| {
            dp_end.contains_key(&(pos, (-direction.0, -direction.1)))
                && steps + dp_end[&(pos, (-direction.0, -direction.1))] == total_steps
        })
        .map(|(&(pos, _), _)| pos)
        .collect();

    Some(good_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
