use std::cell;

advent_of_code::solution!(12);
pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut fence_map = std::collections::HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let mut fence_count = 0;
            if y == 0 || grid[y - 1][x] != cell {
                fence_count += 1;
            }
            if y == grid.len() - 1 || grid[y + 1][x] != cell {
                fence_count += 1;
            }
            if x == 0 || grid[y][x - 1] != cell {
                fence_count += 1;
            }
            if x == row.len() - 1 || grid[y][x + 1] != cell {
                fence_count += 1;
            }
            fence_map.insert((x, y), fence_count);
        }
    }

    let visited = cell::RefCell::new(std::collections::HashSet::new());
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if !visited.borrow().contains(&(x, y)) {
                let mut stack = vec![(x, y)];
                let mut area = 0;
                let mut fences = 0;

                while let Some((x, y)) = stack.pop() {
                    if !visited.borrow().contains(&(x, y)) {
                        visited.borrow_mut().insert((x, y));
                        area += 1;
                        fences += fence_map[&(x, y)];
                        let cell = grid[y][x];

                        if x > 0 && grid[y][x - 1] == cell {
                            stack.push((x - 1, y));
                        }
                        if x < row.len() - 1 && grid[y][x + 1] == cell {
                            stack.push((x + 1, y));
                        }
                        if y > 0 && grid[y - 1][x] == cell {
                            stack.push((x, y - 1));
                        }
                        if y < grid.len() - 1 && grid[y + 1][x] == cell {
                            stack.push((x, y + 1));
                        }
                    }
                }
                sum += area * fences;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let visited = cell::RefCell::new(std::collections::HashSet::new());
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if !visited.borrow().contains(&(x, y)) {
                let mut stack = vec![(x, y)];
                let mut subset = std::collections::HashSet::new();
                let mut corners = 0;

                while let Some((x, y)) = stack.pop() {
                    if !visited.borrow().contains(&(x, y)) {
                        visited.borrow_mut().insert((x, y));
                        subset.insert((x, y));

                        if x > 0 && grid[y][x - 1] == cell {
                            stack.push((x - 1, y));
                        }
                        if x < row.len() - 1 && grid[y][x + 1] == cell {
                            stack.push((x + 1, y));
                        }
                        if y > 0 && grid[y - 1][x] == cell {
                            stack.push((x, y - 1));
                        }
                        if y < grid.len() - 1 && grid[y + 1][x] == cell {
                            stack.push((x, y + 1));
                        }
                    }
                }

                for &(x, y) in &subset {
                    let mut missing_neighbors = 0;
                    if x == 0 || !subset.contains(&(x - 1, y)) {
                        missing_neighbors += 1;
                    }
                    if x == row.len() - 1 || !subset.contains(&(x + 1, y)) {
                        missing_neighbors += 1;
                    }
                    if y == 0 || !subset.contains(&(x, y - 1)) {
                        missing_neighbors += 1;
                    }
                    if y == grid.len() - 1 || !subset.contains(&(x, y + 1)) {
                        missing_neighbors += 1;
                    }

                    let corners_add = match missing_neighbors {
                        4 => 4,
                        3 => 2,
                        2 => {
                            if (x > 0 && subset.contains(&(x - 1, y)) && x < row.len() - 1 && subset.contains(&(x + 1, y)))
                                || (y > 0 && subset.contains(&(x, y - 1)) && y < grid.len() - 1 && subset.contains(&(x, y + 1)))
                            {
                                0
                            } else {
                                1
                            }
                        }
                        _ => 0,
                    };
                    corners += corners_add;

                    if x > 0 && y > 0 && !subset.contains(&(x - 1, y - 1)) && subset.contains(&(x, y - 1)) && subset.contains(&(x - 1, y)) {
                        corners += 1;
                    }
                    if x < row.len() - 1 && y < grid.len() - 1 && !subset.contains(&(x + 1, y + 1)) && subset.contains(&(x, y + 1)) && subset.contains(&(x + 1, y)) {
                        corners += 1;
                    }
                    if x > 0 && y < grid.len() - 1 && !subset.contains(&(x - 1, y + 1)) && subset.contains(&(x, y + 1)) && subset.contains(&(x - 1, y)) {
                        corners += 1;
                    }
                    if x < row.len() - 1 && y > 0 && !subset.contains(&(x + 1, y - 1)) && subset.contains(&(x, y - 1)) && subset.contains(&(x + 1, y)) {
                        corners += 1;
                    }
                }

                sum += subset.len() * corners;
            }
        }
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
