use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn generate_paths(grid: &[Vec<u32>], max_step: u32) -> Vec<Vec<(usize, usize)>> {
    let mut paths = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                paths.push(vec![(x, y)]);
            }
        }
    }

    for step in 1..=max_step {
        let mut new_paths = Vec::new();
        for steps in &paths {
            let (x, y) = steps[step as usize - 1];
            if y > 0 && grid[y - 1][x] == step {
                let mut new_steps = steps.clone();
                new_steps.push((x, y - 1));
                new_paths.push(new_steps);
            }
            if x > 0 && grid[y][x - 1] == step {
                let mut new_steps = steps.clone();
                new_steps.push((x - 1, y));
                new_paths.push(new_steps);
            }
            if x < grid[y].len() - 1 && grid[y][x + 1] == step {
                let mut new_steps = steps.clone();
                new_steps.push((x + 1, y));
                new_paths.push(new_steps);
            }
            if y < grid.len() - 1 && grid[y + 1][x] == step {
                let mut new_steps = steps.clone();
                new_steps.push((x, y + 1));
                new_paths.push(new_steps);
            }
        }
        paths = new_paths;
    }

    paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let paths = generate_paths(&grid, 9);
    let unique_paths: HashSet<_> = paths
        .iter()
        .map(|steps| (steps[0], steps[steps.len() - 1]))
        .collect();
    Some(unique_paths.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let paths = generate_paths(&grid, 9);
    Some(paths.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
