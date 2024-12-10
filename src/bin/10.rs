use std::{collections::HashSet, path};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    // parse input as 2d grid if single digit numbers
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    // find all starting coordinates, where the value is 0
    let mut paths = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let mut start = Vec::new();
                start.push((x, y));
                paths.push(start);
            }
        }
    }
    // for 1 to 9..
    for step in 1..10 {
        let mut new_paths = Vec::new();
        // for each starting coordinate
        for steps in &mut paths {
            // get the last coordinate
            let (x, y) = steps[step - 1];
                // check if the value is surrounded by step
                 if y > 0 && grid[y - 1][x] == step as u32 {
                    let mut new_steps = steps.clone();
                    new_steps.push((x, y - 1));
                    new_paths.push(new_steps);
                }
                if x > 0 && grid[y][x - 1] == step as u32{
                    let mut new_steps = steps.clone();
                    new_steps.push((x - 1, y));
                    new_paths.push(new_steps);
                }
                if x < grid[y].len() - 1 && grid[y][x + 1] == step as u32 {
                    let mut new_steps = steps.clone();
                    new_steps.push((x + 1, y));
                    new_paths.push(new_steps);
                }
                if y < grid.len() - 1 && grid[y + 1][x] == step as u32{
                    let mut new_steps = steps.clone();
                    new_steps.push((x, y + 1));
                    new_paths.push(new_steps);
                }

        }
        paths = new_paths;

    }

    // return the number of paths with unique first and last value
    let unique_paths: HashSet<_> = paths
        .iter()
        .map(|steps| (steps[0], steps[steps.len() - 1]))
        .collect();
    let count = unique_paths.len() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    // find all starting coordinates, where the value is 0
    let mut paths = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let mut start = Vec::new();
                start.push((x, y));
                paths.push(start);
            }
        }
    }
    // for 1 to 9..
    for step in 1..10 {
        let mut new_paths = Vec::new();
        // for each starting coordinate
        for steps in &mut paths {
            // get the last coordinate
            let (x, y) = steps[step - 1];
                // check if the value is surrounded by step
                 if y > 0 && grid[y - 1][x] == step as u32 {
                    let mut new_steps = steps.clone();
                    new_steps.push((x, y - 1));
                    new_paths.push(new_steps);
                }
                if x > 0 && grid[y][x - 1] == step as u32{
                    let mut new_steps = steps.clone();
                    new_steps.push((x - 1, y));
                    new_paths.push(new_steps);
                }
                if x < grid[y].len() - 1 && grid[y][x + 1] == step as u32 {
                    let mut new_steps = steps.clone();
                    new_steps.push((x + 1, y));
                    new_paths.push(new_steps);
                }
                if y < grid.len() - 1 && grid[y + 1][x] == step as u32{
                    let mut new_steps = steps.clone();
                    new_steps.push((x, y + 1));
                    new_paths.push(new_steps);
                }

        }
        paths = new_paths;

    }
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
