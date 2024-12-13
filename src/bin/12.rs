use std::cell;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    //parse input as 2d grid of characters
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // create hashmap to store for each coordinate the number of fence needed around
    let mut fence_map = std::collections::HashMap::new();
    // iterate over the grid
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = grid[y][x];
            // count the number of fence needed for each cell up down left right
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
                if x == grid[y].len() - 1 || grid[y][x + 1] != cell {
                    fence_count += 1;
                }
            fence_map.insert((x, y), fence_count);
        }
    }
    // create an empty hashset of visited coordinates
    let visited = cell::RefCell::new(std::collections::HashSet::new());
    // create a function to recursively visit all the cells of the same color
    // iterate over the grid
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // if the cell has not been visited
            if !visited.borrow().contains(&(x, y)) {
                // create a stack of coordinates to visit
                let mut stack = vec![(x, y)];
                // create a variable to store the number of cells visited
                let mut area = 0;
                let mut fences = 0;
                // while the stack is not empty
                while let Some((x, y)) = stack.pop() {
                    // if the cell has not been visited
                    if !visited.borrow().contains(&(x, y)) {
                        // mark the cell as visited
                        visited.borrow_mut().insert((x, y));
                        // increment the count
                        area += 1;
                        // get the number of fence needed for the cell
                        fences += fence_map.get(&(x, y)).unwrap();
                        let cell = grid[y][x];
                        // add the neighbors to the stack
                        if x > 0 && grid[y][x - 1] == cell{
                            stack.push((x - 1, y));
                        }
                        if x < grid[y].len() - 1 && grid[y][x+1] == cell{
                            stack.push((x + 1, y));
                        }
                        if y > 0 && grid[y-1][x] == cell{
                            stack.push((x, y - 1));
                        }
                        if y < grid.len() - 1 && grid[y +1][x] == cell{
                            stack.push((x, y + 1));
                        }
                    }
                }
                // print the area and fence count
                sum += area * fences;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    //parse input as 2d grid of characters
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // create an empty hashset of visited coordinates
    let visited = cell::RefCell::new(std::collections::HashSet::new());
    // create a function to recursively visit all the cells of the same color
    // iterate over the grid
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // if the cell has not been visited
            if !visited.borrow().contains(&(x, y)) {
                // create a stack of coordinates to visit
                let mut stack = vec![(x, y)];
                // create a variable to store the number of cells visited
                let mut subset = std::collections::HashSet::new();
                let mut corners = 0;
                // while the stack is not empty
                while let Some((x, y)) = stack.pop() {
                    // if the cell has not been visited
                    if !visited.borrow().contains(&(x, y)) {
                        // mark the cell as visited
                        visited.borrow_mut().insert((x, y));
                        // increment the count
                        subset.insert((x, y));
                        let cell = grid[y][x];
                        // add the neighbors to the stack
                        if x > 0 && grid[y][x - 1] == cell{
                            stack.push((x - 1, y));
                        }
                        if x < grid[y].len() - 1 && grid[y][x+1] == cell{
                            stack.push((x + 1, y));
                        }

                        if y > 0 && grid[y-1][x] == cell{
                            stack.push((x, y - 1));
                        }
                        if y < grid.len() - 1 && grid[y +1][x] == cell{
                            stack.push((x, y + 1));
                        }
                    }
                }
                for cell in subset.iter() {
                    let x = cell.0;
                    let y = cell.1;
                    let mut missing_neighbors = 0;
                    if x == 0 {
                        missing_neighbors += 1;
                    } else if x > 0 && !subset.contains(&(x - 1, y)) {
                        missing_neighbors += 1;
                    }

                    if x == grid[y].len() - 1 {
                        missing_neighbors += 1;
                    } else if x < grid[y].len() - 1 && !subset.contains(&(x + 1, y)) {
                        missing_neighbors += 1;
                    }

                    if y == 0 {
                        missing_neighbors += 1;
                    } else if y > 0 && !subset.contains(&(x, y - 1)) {
                        missing_neighbors += 1;
                    }

                    if y == grid.len() - 1 {
                        missing_neighbors += 1;
                    } else if y < grid.len() - 1 && !subset.contains(&(x, y + 1)) {
                        missing_neighbors += 1;
                    }
                    let corners_add = match missing_neighbors {
                        4 => 4,
                        3 => 2,
                        2 => if (x > 0 && subset.contains(&(x - 1, y)) && x < grid[y].len() - 1 && subset.contains(&(x + 1, y))) ||
                                (y > 0 && subset.contains(&(x, y - 1)) && y < grid.len() - 1 && subset.contains(&(x, y + 1)) ){ 0 } else {1},
                        _ => 0,
                    };
                    println!("x: {}, y: {}, missing_neighbors: {}, corners_add: {}", x, y, missing_neighbors, corners_add);
                    corners += corners_add;

                    if x > 0 && y > 0 && !subset.contains(&(x - 1, y - 1)) && subset.contains(&(x, y - 1)) && subset.contains(&(x - 1, y))  {
                        println!("x: {}, y: {}", x, y);
                        corners += 1;
                    }
                    if x < grid[y].len() - 1 &&  y < grid.len() - 1 &&!subset.contains(&(x + 1, y + 1)) && subset.contains(&(x, y + 1)) && subset.contains(&(x + 1, y))  {
                        println!("x: {}, y: {}", x, y);
                        corners += 1;
                    }
                    if x > 0 && y < grid.len() - 1 && !subset.contains(&(x - 1, y + 1)) && subset.contains(&(x, y + 1)) && subset.contains(&(x - 1, y))  {
                        println!("x: {}, y: {}", x, y);
                        corners += 1;
                    }
                    if x < grid[y].len() - 1 && y > 0 && !subset.contains(&(x + 1, y - 1)) && subset.contains(&(x, y - 1)) && subset.contains(&(x + 1, y))  {
                        println!("x: {}, y: {}", x, y);
                        corners += 1;
                    }
                }

                // print the subset len and corners count
                println!("subset: {:?}, corners: {}", subset, corners);
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
