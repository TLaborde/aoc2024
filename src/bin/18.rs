
use std::collections::{hash_set, VecDeque};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    // read the first 1024 line of input and parse as couple of coordinate put in a hashset
    let obstacles = input
        .lines()
        .take(1024)
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<u32>().unwrap();
            let y = iter.next().unwrap().parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<std::collections::HashSet<(u32, u32)>>();

    let width = 71;
    let height = 71;
    let start = (0, 0);
    let end = (height - 1, width - 1);
    let mut visited = std::collections::HashSet::new();
    let mut queue = VecDeque::new();
    let mut distances = std::collections::HashMap::new();

    queue.push_back(start);
    distances.insert(start, 0);

    while let Some(current) = queue.pop_front() {
        if current == end {
            break;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let (x, y): (u32, u32) = current;
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for &neighbor in &neighbors {
            if neighbor.0 < height && neighbor.1 < width && !obstacles.contains(&neighbor) && !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                let distance = distances[&current] + 1;
                distances.entry(neighbor).or_insert(distance);
            }
        }
    }

    Some(distances[&end])

}

pub fn dfs_returning_path (obstacles: std::collections::HashSet<(u32, u32)>, start: (u32, u32), end: (u32, u32)) -> Option<Vec<(u32, u32)>> {
    let mut visited = std::collections::HashSet::new();
    let mut stack: Vec<((u32, u32),Vec<(u32, u32)>)> = Vec::new();
    let path:Vec<(u32, u32)> = Vec::new();

    stack.push((start, path));
    let height = end.0 + 1;
    let width = end.1 + 1;
    while let Some((current, previous_path)) = stack.pop() {
        if current == end {
            return Some(previous_path);
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let (x, y): (u32, u32) = current;
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for &neighbor in &neighbors {
            if neighbor.0 < height && neighbor.1 < width && !obstacles.contains(&neighbor) && !visited.contains(&neighbor) {
                let mut path = previous_path.clone();
                path.push(neighbor);
                stack.push((neighbor, path));
            }
        }
    }
    return Some(Vec::new());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut obstacles = input
        .lines()
        .take(1024)
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<u32>().unwrap();
            let y = iter.next().unwrap().parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<std::collections::HashSet<(u32, u32)>>();
    // make a list from all the obstacles after 1024
    let mut obstacles_list = input
        .lines()
        .skip(1024)
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<u32>().unwrap();
            let y = iter.next().unwrap().parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<Vec<(u32, u32)>>();

        let width = 71;
        let height = 71;
        let start = (0, 0);
        let end = (height - 1, width - 1);

        // find a path from start to end
        let mut path = dfs_returning_path(obstacles.clone(), start, end).unwrap();

        while obstacles_list.len() > 0 {
            // add new obstacle to the hashset until we find one that is on the path
            let obstacle = obstacles_list[0];
            obstacles.insert(obstacle);
            obstacles_list.remove(0);
            if path.contains(&obstacle) {
                // recalculate the path
                path = dfs_returning_path(obstacles.clone(), start, end).unwrap();
                // if path is none, we have found the solution
                if path.is_empty() {
                    println!("{:?} is blocking the way", obstacle);
                    return Some(obstacle.0);
                }
            }
        };
        return None;

}

