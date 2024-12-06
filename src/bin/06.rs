use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    //parse input as 2d array of chars
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //find the coordinate of the starting point, which is somewhere in the grid
    let mut position = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .unwrap();

    println!("{} {}", position.0, position.1);
    // first direction is up
    let mut direction = (0, -1);
    // until we reach the end of the grid, keep moving
    // create a hashset to store the visited coordinates
    let mut visited = std::collections::HashSet::new();
    // add the starting point to the hashset
    visited.insert(position);
    loop {
        let new_x = position.0 as i32 + direction.0;
        let new_y = position.1 as i32 + direction.1;
        if new_x < 0 || new_y < 0 || new_x > (grid[0].len() -1).try_into().unwrap() || new_y > (grid.len()-1).try_into().unwrap() {
            println!("out od bounds {} {} {} {}", new_x, new_y, grid[0].len(), grid.len());
            break;
        }
        // get the next character
        let next = grid[new_y as usize][new_x as usize];

        if next == '#' {
            // rotate the direction 90 degrees clockwise
            direction = (-direction.1, direction.0);
        } else {
            position.0 = new_x as usize;
            position.1 = new_y as usize;
            visited.insert(position);
        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    //parse input as 2d array of chars
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //find the coordinate of the starting point, which is somewhere in the grid
    let initial_position = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .unwrap();
    let initial_direction = (0, -1);
    let mut position = initial_position;
    // first direction is up
    let mut direction = initial_direction;
    // until we reach the end of the grid, keep moving
    // create a hashset to store the visited coordinates
    let mut possible_obstactle = std::collections::HashSet::new();
    loop {
        let new_x = position.0 as i32 + direction.0;
        let new_y = position.1 as i32 + direction.1;
        if new_x < 0 || new_y < 0 || new_x > (grid[0].len() -1).try_into().unwrap() || new_y > (grid.len()-1).try_into().unwrap() {
            break;
        }
        // get the next character
        let next = grid[new_y as usize][new_x as usize];

        if next == '#' {
            // rotate the direction 90 degrees clockwise
            direction = (-direction.1, direction.0);
        } else {
            position.0 = new_x as usize;
            position.1 = new_y as usize;
            possible_obstactle.insert(position);
        }
    }

    //for each possible obstacle, try to move around it
    let mut valid_obstacles = 0;
    'obs_loop: for obstacle in possible_obstactle {
        let mut visited: HashSet<(usize, usize, i32, i32)> = std::collections::HashSet::new();
        let mut position = initial_position;
        let mut direction = initial_direction;
        visited.insert((position.0, position.1, direction.0, direction.1));
        loop {
            let new_x = position.0 as i32 + direction.0;
            let new_y = position.1 as i32 + direction.1;
            if new_x < 0 || new_y < 0 || new_x > (grid[0].len() -1).try_into().unwrap() || new_y > (grid.len()-1).try_into().unwrap() {
                continue 'obs_loop; // out of bound
            }
            let next = grid[new_y as usize][new_x as usize];
            if next == '#' || (new_x == obstacle.0 as i32 && new_y == obstacle.1 as i32){
                direction = (-direction.1, direction.0);
            } else {
                position.0 = new_x as usize;
                position.1 = new_y as usize;
                //if position and direction exists in the visited set, we have visited this position before
                if visited.contains(&(position.0, position.1, direction.0, direction.1)) {
                    valid_obstacles += 1;
                    continue 'obs_loop;
                }
                visited.insert((position.0, position.1, direction.0, direction.1));
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
