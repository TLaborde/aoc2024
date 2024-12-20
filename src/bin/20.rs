advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    // calculate shortest distance from any point from the start
    // calculate shortest distance from any point to the end
    // for any pair of obstacles find if there is a path shorter than 20
    // if that's the case
    let (grid, start, end) = parse_input(input);
    let reference_path_length =
        find_shortest_path(&grid, start, end, Some(i32::MAX)).unwrap() as i32;
    let obstacles: std::collections::HashSet<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == '#' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    let dist_from_start = dfs(&obstacles, start);
    let dist_from_end = dfs(&obstacles, end);
    let mut valid_shortcuts = std::collections::HashSet::new();
    //for each pair of cell in the grid that are not obstacles
    for (i1, j1) in (0..grid.len() as i32).flat_map(|i| (0..grid[0].len() as i32).map(move |j| (i, j))) {
        if grid[i1 as usize][j1 as usize] == '#' {
            continue;
        }
        for (i2, j2) in (0..grid.len() as i32).flat_map(|i| (0..grid[0].len() as i32).map(move |j| (i, j))) {
            if grid[i2 as usize][j2 as usize] == '#' {
                continue;
            }
            if (i1, j1) == (i2, j2) {
                continue;
            }
            // if manahttan distance is greater than 19, continue
            let manathan_distance = (i1 - i2).abs() + (j1 - j2).abs();
            if manathan_distance > 2 {
                continue;
            }
            if !dist_from_start.contains_key(&(i1, j1)) || !dist_from_end.contains_key(&(i2, j2)) {
                continue;
            }
            if dist_from_start.get(&(i1, j1)).unwrap() + dist_from_end.get(&(i2, j2)).unwrap() > reference_path_length {
                continue;
            }
            let total_length = dist_from_start.get(&(i1, j1)).unwrap() + dist_from_end.get(&(i2, j2)).unwrap() + manathan_distance;
            if total_length <= reference_path_length - 100 {
                //println!("found shortcut from {:?} to {:?} with length cut by {}", (i1, j1), (i2, j2), reference_path_length - total_length);
                valid_shortcuts.insert(((i1, j1), (i2, j2)));
            }
        }
    }
    Some(valid_shortcuts.len() as u32)
}

fn find_shortest_path(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
    max_path_length: Option<i32>,
) -> Option<usize> {
    let max_path_length = max_path_length.unwrap_or(i32::MAX);
    let mut visited = std::collections::HashSet::new();
    let mut queue: std::collections::VecDeque<((i32, i32), usize)> =
        std::collections::VecDeque::new();
    queue.push_back((start, 0));
    while let Some((current, distance)) = queue.pop_front() {
        if current == end {
            return Some(distance);
        }
        if visited.contains(&current) || distance as i32 > max_path_length {
            continue;
        }
        visited.insert(current);
        let (x, y): (i32, i32) = current;
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for &neighbor in &neighbors {
            if neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= grid.len() as i32
                || neighbor.1 >= grid[0].len() as i32
            {
                continue;
            }
            if grid[neighbor.0 as usize][neighbor.1 as usize] == '#' {
                continue;
            }
            queue.push_back((neighbor, distance + 1));
        }
    }
    Some(0)
}


pub fn part_two(input: &str) -> Option<u32> {
    // calculate shortest distance from any point from the start
    // calculate shortest distance from any point to the end
    // for any pair of obstacles find if there is a path shorter than 20
    // if that's the case
    let (grid, start, end) = parse_input(input);
    let reference_path_length =
        find_shortest_path(&grid, start, end, Some(i32::MAX)).unwrap() as i32;
    let obstacles: std::collections::HashSet<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == '#' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    let dist_from_start = dfs(&obstacles, start);
    let dist_from_end = dfs(&obstacles, end);
    let mut valid_shortcuts = std::collections::HashSet::new();
    //for each pair of cell in the grid that are not obstacles
    for (i1, j1) in (0..grid.len() as i32).flat_map(|i| (0..grid[0].len() as i32).map(move |j| (i, j))) {
        if grid[i1 as usize][j1 as usize] == '#' {
            continue;
        }
        for (i2, j2) in (0..grid.len() as i32).flat_map(|i| (0..grid[0].len() as i32).map(move |j| (i, j))) {
            if grid[i2 as usize][j2 as usize] == '#' {
                continue;
            }
            if (i1, j1) == (i2, j2) {
                continue;
            }
            // if manahttan distance is greater than 19, continue
            let manathan_distance = (i1 - i2).abs() + (j1 - j2).abs();
            if manathan_distance > 20 {
                continue;
            }
            if !dist_from_start.contains_key(&(i1, j1)) || !dist_from_end.contains_key(&(i2, j2)) {
                continue;
            }
            if dist_from_start.get(&(i1, j1)).unwrap() + dist_from_end.get(&(i2, j2)).unwrap() > reference_path_length {
                continue;
            }
            let total_length = dist_from_start.get(&(i1, j1)).unwrap() + dist_from_end.get(&(i2, j2)).unwrap() + manathan_distance;
            if total_length <= reference_path_length - 100 {
                //println!("found shortcut from {:?} to {:?} with length cut by {}", (i1, j1), (i2, j2), reference_path_length - total_length);
                valid_shortcuts.insert(((i1, j1), (i2, j2)));
            }
        }
    }
    Some(valid_shortcuts.len() as u32)
}

fn dfs(
    obstacles: &std::collections::HashSet<(i32, i32)>,
    start: (i32, i32),
) -> std::collections::HashMap<(i32, i32), i32> {
    let mut visited = std::collections::HashSet::new();
    let mut stack: Vec<((i32, i32), i32)> = Vec::new();
    let mut distances = std::collections::HashMap::new();
    stack.push((start, 0));
    while let Some((current, distance)) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        distances.insert(current, distance);
        if obstacles.contains(&current) {
            continue;
        }
        let (x, y): (i32, i32) = current;
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for &neighbor in &neighbors {
            if !visited.contains(&neighbor) {
                stack.push((neighbor, distance + 1));
            }
        }
    }
    distances
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
