use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(6);

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let initial_position = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .unwrap();

    (grid, initial_position)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, mut position) = parse_input(input);
    let mut direction = (0, -1);
    let mut visited = HashSet::new();
    visited.insert(position);

    loop {
        let new_x = position.0 as i32 + direction.0;
        let new_y = position.1 as i32 + direction.1;
        if new_x < 0 || new_y < 0 || new_x >= grid[0].len() as i32 || new_y >= grid.len() as i32 {
            break;
        }

        let next = grid[new_y as usize][new_x as usize];
        if next == '#' {
            direction = (-direction.1, direction.0);
        } else {
            position = (new_x as usize, new_y as usize);
            visited.insert(position);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, initial_position) = parse_input(input);
    let initial_direction = (0, -1);
    let mut position = initial_position;
    let mut direction = initial_direction;
    let mut possible_obstacles = HashSet::new();

    loop {
        let new_x = position.0 as isize + direction.0;
        let new_y = position.1 as isize + direction.1;
        if new_x < 0 || new_y < 0 || new_x >= grid[0].len() as isize || new_y >= grid.len() as isize {
            break;
        }

        let next = grid[new_y as usize][new_x as usize];
        if next == '#' {
            direction = (-direction.1, direction.0);
        } else {
            position = (new_x as usize, new_y as usize);
            possible_obstacles.insert(position);
        }
    }

    let mut valid_obstacles = 0;
    'obs_loop: for &obstacle in &possible_obstacles {
        let mut visited = HashSet::new();
        let mut position = initial_position;
        let mut direction = initial_direction;
        visited.insert((position, direction));

        loop {

            let new_x = position.0 as isize + direction.0;
            let new_y = position.1 as isize + direction.1;
            if new_x < 0 || new_y < 0 || new_x >= grid[0].len() as isize || new_y >= grid.len() as isize {
                continue 'obs_loop;
            }

            let next = grid[new_y as usize][new_x as usize];
            if next == '#' || (new_x == obstacle.0 as isize && new_y == obstacle.1 as isize) {
                direction = (-direction.1, direction.0);
            } else {
                position = (new_x as usize, new_y as usize);
                if visited.contains(&(position, direction)) {
                    valid_obstacles += 1;
                    continue 'obs_loop;
                }
                visited.insert((position, direction));
            }
        }
    }
    Some(valid_obstacles)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
