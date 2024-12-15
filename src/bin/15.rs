advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    // read input, make of 2d array of character for all lines except the last 2
    // last line are the direction of the robot

    let lines: Vec<&str> = input.lines().collect();
    let mut grid = lines
        .iter()
        .take_while(|&&line| !line.is_empty())
        .map(|&line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let directions = lines.last().unwrap().chars().collect::<Vec<_>>();

    // find the starting point of the robot, it's the @ character
    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '@').map(|x| (x, y)))
        .unwrap();

    for d in directions {
        let direction = match d {
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => panic!("Invalid direction"),
        };

        let mut boxes = Vec::new();
        let mut next = (robot.0 as i32 + direction.0, robot.1 as i32 + direction.1);
        loop {
            let next_content = grid[next.1 as usize][next.0 as usize];
            if next_content == '#' {
                break;
            }
            if next_content == 'O' {
                boxes.push(next);
                next = (next.0 + direction.0, next.1 + direction.1);
                continue;
            }
            // we can move the robot
            if next_content == '.' {
                if !boxes.is_empty() {
                    grid[next.1 as usize][next.0 as usize] = 'O';
                    grid[robot.1][robot.0] = '.';
                    robot = (boxes[0].0 as usize as usize, boxes[0].1 as usize);
                    grid[robot.1][robot.0] = '@';
                } else {
                    grid[robot.1][robot.0] = '.';
                    robot = (next.0 as usize, next.1 as usize);
                    grid[robot.1][robot.0] = '@';
                }
                break;
            }
        }
    }
    // print final grid
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }

    let sum = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == 'O' {
                    Some(((y as u32) * 100) + (x as u32))
                } else {
                    None
                }
            })
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // read input, make of 2d array of character for all lines except the last 2
    // last line are the direction of the robot

    let lines: Vec<&str> = input.lines().collect();
    // parse the grid, depending on the character, insert the character twice
    let mut grid = lines
        .iter()
        .take_while(|&&line| !line.is_empty())
        .map(|&line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' | '.' => vec![c, c],
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    _ => panic!("Invalid direction"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }

    let directions = lines.last().unwrap().chars().collect::<Vec<_>>();

    // find the starting point of the robot, it's the @ character
    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '@')
                .map(|x| (x as i32, y as i32))
        })
        .unwrap();

    for d in directions {
        let direction = match d {
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => panic!("Invalid direction"),
        };
        println!("Direction: {:?}", d);
        let mut box_moves: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let mut nexts = Vec::new();
        nexts.push((robot.0 + direction.0, robot.1 + direction.1));
        loop {
            let next_contents: Vec<char> = nexts
                .iter()
                .map(|next| grid[next.1 as usize][next.0 as usize])
                .collect();
            println!("next content: {:?}", next_contents);
            if next_contents.contains(&'#') {
                break;
            }

            // we can move the robot
            // if all next contnes are '.' we can move
            if next_contents.iter().all(|&c| c == '.') {
                // move all the boxes, starting from the last one
                for (from, to) in box_moves.iter().rev() {
                    grid[to.1 as usize][to.0 as usize] = grid[from.1 as usize][from.0 as usize];
                    // if no box will move to the from position, we put a . in from
                    if !box_moves.iter().any(|(_, to)| to == from) {
                        grid[from.1 as usize][from.0 as usize] = '.';
                    }
                    println!("Moving box from {:?} to {:?}", from, to);
                }
                grid[robot.1 as usize][robot.0 as usize] = '.';
                robot = (robot.0 as i32 + direction.0, robot.1 as i32 + direction.1);
                grid[robot.1 as usize][robot.0 as usize] = '@';
                break;
            }
            let mut new_nexts = Vec::new();
            for next in &nexts {
                let next_content = grid[next.1 as usize][next.0 as usize];
                if next_content == '.' {
                    continue;
                }
                if d == '>' {
                    let next_next = (next.0 + 1, next.1);
                    box_moves.push((*next, next_next));
                    let next_next_next = (next_next.0 + 1, next_next.1);
                    box_moves.push((next_next, next_next_next));
                    new_nexts.push(next_next_next);
                }
                if d == '<' {
                    let next_next = (next.0 - 1, next.1);
                    box_moves.push((*next, next_next));
                    let next_next_next = (next_next.0 - 1, next_next.1);
                    box_moves.push((next_next, next_next_next));
                    new_nexts.push(next_next_next);
                }
                if d == '^' {
                    let second_half = if next_content == '[' {
                        (next.0 + 1, next.1)
                    } else {
                        (next.0 - 1, next.1)
                    };
                    let next_next = (next.0, next.1 - 1);
                    let next_second_half = (second_half.0, second_half.1 - 1);
                    if !box_moves.contains(&(*next, next_next)) {
                        box_moves.push((*next, next_next));
                    }
                    if !box_moves.contains(&(second_half, next_second_half)) {
                        box_moves.push((second_half, next_second_half));
                    }
                    if !new_nexts.contains(&next_second_half) {
                        new_nexts.push(next_second_half);
                    }
                    if !new_nexts.contains(&next_next) {
                        new_nexts.push(next_next);
                    }
                }
                if d == 'v' {
                    let second_half = if next_content == '[' {
                        (next.0 + 1, next.1)
                    } else {
                        (next.0 - 1, next.1)
                    };
                    let next_next = (next.0, next.1 + 1);
                    let next_second_half = (second_half.0, second_half.1 + 1);
                    if !box_moves.contains(&(*next, next_next)) {
                        box_moves.push((*next, next_next));
                    }
                    if !box_moves.contains(&(second_half, next_second_half)) {
                        box_moves.push((second_half, next_second_half));
                    }
                    if !new_nexts.contains(&next_second_half) {
                        new_nexts.push(next_second_half);
                    }
                    if !new_nexts.contains(&next_next) {
                        new_nexts.push(next_next);
                    }
                }
            }
            nexts = new_nexts;
        }
    }

    let sum = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '[' {
                    Some(((y as u32) * 100) + (x as u32))
                } else {
                    None
                }
            })
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
