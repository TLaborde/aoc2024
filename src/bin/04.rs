advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // make the input into a 2d array of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // make a list of all the "X" characters position
    let mut x_positions: Vec<(usize, usize)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'X' {
                x_positions.push((x, y));
            }
        }
    }
    // for each x_position, find the one with a "M" in one of the 8 direct adjacent position
    let mut m_positions: Vec<(usize, usize, i32, i32)> = Vec::new();
    for (x, y) in x_positions {
        for &(dx, dy) in &[(0, 1), (1, 0), (1, 1), (1, -1), (0, -1), (-1, 0), (-1, -1), (-1, 1)] {
            if let Some(cell) = grid.get((y as i32 + dy) as usize).and_then(|row| row.get((x as i32 + dx) as usize)) {
                if *cell == 'M' {
                    m_positions.push(((x as i32 + dx) as usize,(y as i32 + dy) as usize, dx, dy));
                }
            }
        }
    }
    let mut a_positions: Vec<(usize, usize, i32, i32)> = Vec::new();
    for (x, y, dx, dy) in m_positions {
        let new_y = (y as i32 + dy) as usize;
        let new_x = (x as i32+ dx) as usize;
            if let Some(cell) = grid.get(new_y).and_then(|row| row.get(new_x)) {
                if *cell == 'A' {
                    a_positions.push((new_x, new_y, dx, dy));
                }
            }

    }
    let mut s_positions: Vec<(usize, usize, i32, i32)> = Vec::new();
    for (x, y, dx, dy) in a_positions {
        let new_y = (y as i32 + dy) as usize;
        let new_x = (x as i32+ dx) as usize;
        if new_y < grid.len() && new_x < grid[new_y].len() {
            if let Some(cell) = grid.get(new_y).and_then(|row| row.get(new_x)) {
                if *cell == 'S' {
                    s_positions.push((x, y, dx, dy));
                }
            }
        }
    }
    Some(s_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // make the input into a 2d array of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // make a list of all the "X" characters position
    let mut a_positions: Vec<(usize, usize)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'A' {
                a_positions.push((x, y));
            }
        }
    }
    // fora each a position, get the values in the 4 corners adjacent to it
    let mut found = 0;
    for (x, y) in a_positions {
        let mut corners: Vec<(&char)> = Vec::new();
        for &(dx, dy) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            if let Some(cell) = grid.get((y as i32 + dy) as usize).and_then(|row| row.get((x as i32 + dx) as usize)) {
                corners.push(cell);
            }
        }
        // check if the opposite corners are M and S
        if corners.len() ==4 && (corners[0] == &'M' && corners[3] == &'S' || corners[0] == &'S' && corners[3] == &'M') && (corners[1] == &'M' && corners[2] == &'S' || corners[1] == &'S' && corners[2] == &'M') {
            found += 1;
        }
    }
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
