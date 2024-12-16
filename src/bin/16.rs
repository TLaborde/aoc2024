use std::collections::HashMap;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    // parse input as a 2d vector of chars
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // find the start S and end E
    let start = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let end = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'E')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    // first direction is facing east
    let direction = (0, 1);
    // find the shortest path from S to E
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(((start, direction), 0));
    let mut visited = std::collections::HashSet::new();
    visited.insert(start);
    let mut total_steps = 0;
    while let Some(((pos, direction), steps)) = queue.pop_front() {
        if pos == end {
            total_steps = steps;
            break;
        }
        for (i, j) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_pos = (pos.0 as i32 + i, pos.1 as i32 + j);
            if new_pos.0 < 0
                || new_pos.0 >= input.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= input[0].len() as i32
            {
                continue;
            }
            if visited.contains(&new_pos) {
                continue;
            }
            if input[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                continue;
            }
            let new_direction = (*i, *j);
            if direction == new_direction {
                queue.push_front(((new_pos, new_direction), steps + 1));
            } else {
                // add 1000 to step for each 90 degree turn
                // compare new_direction with direction to know how much we turned
                let new_steps = steps
                    + 1
                    + 1000 * (direction.0 * new_direction.1 - direction.1 * new_direction.0).abs();
                queue.push_back(((new_pos, new_direction), new_steps));
            }
            visited.insert(new_pos);
        }
    }
    Some(total_steps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse input as a 2d vector of chars
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // find the start S and end E
    let start = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let end = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'E')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    // first direction is facing east
    let direction = (0, 1);

    let mut dp = HashMap::new();
    dp.insert((start, direction), 0);
    let mut queue = std::collections::VecDeque::new();

    queue.push_back((start, direction));
    while let Some((pos, direction)) = queue.pop_front() {
        for (i, j) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_pos = (pos.0 as i32 + i, pos.1 as i32 + j);
            if new_pos.0 < 0
                || new_pos.0 >= input.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= input[0].len() as i32
            {
                continue;
            }
            if input[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                continue;
            }
            let new_direction = (*i, *j);
            let new_steps = dp[&(pos, direction)]
                + 1
                + 1000 * (direction.0 * new_direction.1 - direction.1 * new_direction.0).abs();
            if !dp.contains_key(&(new_pos, new_direction))
                || new_steps < dp[&(new_pos, new_direction)]
            {
                dp.insert((new_pos, new_direction), new_steps);
                queue.push_back((new_pos, new_direction));
            }
        }
    }

    // do the same from the end
    let mut dp2 = HashMap::new();
    let mut queue = std::collections::VecDeque::new();
    for (i, j) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        println!("{:?}", (end, (*i, *j)));
        dp2.insert((end, (*i, *j)), 0);
        queue.push_back((end, (*i, *j)));
    }

    while let Some((pos, direction)) = queue.pop_front() {
        for (i, j) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_pos = (pos.0 as i32 + i, pos.1 as i32 + j);
            if new_pos.0 < 0
                || new_pos.0 >= input.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= input[0].len() as i32
            {
                continue;
            }
            if input[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                continue;
            }
            let new_direction = (*i, *j);
            let new_steps = dp2[&(pos, direction)]
                + 1
                + 1000 * (direction.0 * new_direction.1 - direction.1 * new_direction.0).abs();
            if !dp2.contains_key(&(new_pos, new_direction))
                || new_steps < dp2[&(new_pos, new_direction)]
            {
                dp2.insert((new_pos, new_direction), new_steps);
                queue.push_back((new_pos, new_direction));
            }
            let new_steps = dp2[&(pos, direction)]
            + 1000 * (direction.0 * new_direction.1 - direction.1 * new_direction.0).abs();
            if !dp2.contains_key(&(pos, new_direction))
                || new_steps < dp2[&(pos, new_direction)] {
                dp2.insert((pos, new_direction), new_steps);
                }
        }
    }

    // count all the combinations of paths that are equal to the shortest path
    let mut good_position = std::collections::HashSet::new();
    // find smallest total steps for matching coordiante, set total steps to + infinity

    let mut total_steps = i32::MAX;
    for (pos, steps) in &dp {
        if let Some(&steps2) = dp2.get(&(pos.0, (-pos.1.0, -pos.1.1))) {
            //println!("{} {} {}", steps + steps2, steps, steps2);
            total_steps = total_steps.min(steps + steps2);
        } else {
            println!("missing for {} {}", pos.0.0, pos.0.1);
        }
    }
    // print total steps
    println!("{:?}", total_steps);
    for (pos, steps) in dp {
        if dp2.contains_key(&(pos.0, (-pos.1.0, -pos.1.1))) && steps + dp2[&(pos.0, (-pos.1.0, -pos.1.1))] == total_steps {
            good_position.insert(pos.0);
        }
    }

    // draw the grid, mark cells that are part of the shortest path
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if good_position.contains(&(i as i32, j as i32)) {
                print!("X");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }

    Some(good_position.len() as u32 + 1)
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
