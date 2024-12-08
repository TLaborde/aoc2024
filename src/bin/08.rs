advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
    // get input width and height
    let width = grid[0].len();
    let height = grid.len();
    // go over the grid, and store in a hashtable the coordinate based on the value of the cell
    let mut antennas = std::collections::HashMap::new();
    let mut antenna_positions = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let cell = grid[y][x];
            if cell != '.' {
                antennas.entry(cell).or_insert_with(Vec::new).push((x, y));
                antenna_positions.push((x, y));
            }
        }
    }
    // new hashset to store the antinodes
    let mut antinodes = std::collections::HashSet::new();
    // for each key value...

    for (key, positions) in antennas.iter() {
        // for each pair of positions...
        for (x1, y1) in positions {
            for (x2, y2) in positions {
                // if the positions are the same, skip
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                println!("{} {} {} {}", x1, y1, x2, y2);
                // if the positions are not the same, calculate the antinode distance
                let new_x= *x1 as i32 - (*x2 as i32 - *x1 as i32);
                let new_y: i32 = *y1 as i32 - (*y2 as i32 - *y1 as i32);
                // if the antinode is within the grid, and not already in the antinode hashset, add it
                if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
                    let new_x1 = new_x as usize;
                    let new_y1 = new_y as usize;
                    //if !antenna_positions.contains(&(new_x1, new_y1))  {
                        antinodes.insert((new_x1, new_y1));
                        println!("{} {}", new_x1, new_y1);
                    //}
                }
            }
        }
    }
    println!("{:?}", antinodes);
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
    // get input width and height
    let width = grid[0].len();
    let height = grid.len();
    // go over the grid, and store in a hashtable the coordinate based on the value of the cell
    let mut antennas = std::collections::HashMap::new();
    let mut antenna_positions = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let cell = grid[y][x];
            if cell != '.' {
                antennas.entry(cell).or_insert_with(Vec::new).push((x, y));
                antenna_positions.push((x, y));
            }
        }
    }
    // new hashset to store the antinodes
    let mut antinodes = std::collections::HashSet::new();
    // for each key value...

    for (key, positions) in antennas.iter() {
        // for each pair of positions...
        for (x1, y1) in positions {
            for (x2, y2) in positions {
                // if the positions are the same, skip
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                antinodes.insert((*x1 as usize, *y1 as usize));
                // if the positions are not the same, calculate the antinode distance
                let mut new_x= *x1 as i32 - (*x2 as i32 - *x1 as i32);
                let mut new_y: i32 = *y1 as i32 - (*y2 as i32 - *y1 as i32);
                // if the antinode is within the grid, and not already in the antinode hashset, add it
                while new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
                    let new_x1 = new_x as usize;
                    let new_y1 = new_y as usize;
                    //if !antenna_positions.contains(&(new_x1, new_y1))  {
                        antinodes.insert((new_x1, new_y1));
                        println!("{} {}", new_x1, new_y1);
                    //}

                    new_x = new_x - (*x2 as i32 - *x1 as i32);
                    new_y = new_y - (*y2 as i32 - *y1 as i32);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
