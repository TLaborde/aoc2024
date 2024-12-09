advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut s_positions = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'X' {
                for &(dx, dy) in &[
                    (0, 1),
                    (1, 0),
                    (1, 1),
                    (1, -1),
                    (0, -1),
                    (-1, 0),
                    (-1, -1),
                    (-1, 1),
                ] {
                    if let Some('M') = grid
                        .get((y as i32 + dy) as usize)
                        .and_then(|row| row.get((x as i32 + dx) as usize))
                    {
                        if let Some('A') = grid
                            .get((y as i32 + 2 * dy) as usize)
                            .and_then(|row| row.get((x as i32 + 2 * dx) as usize))
                        {
                            if let Some('S') = grid
                                .get((y as i32 + 3 * dy) as usize)
                                .and_then(|row| row.get((x as i32 + 3 * dx) as usize))
                            {
                                s_positions += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(s_positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut found = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'A' {
                let corners: Vec<Option<&char>> = vec![
                    grid.get(y.wrapping_add(1))
                        .and_then(|r| r.get(x.wrapping_add(1))),
                    grid.get(y.wrapping_add(1))
                        .and_then(|r| r.get(x.wrapping_sub(1))),
                    grid.get(y.wrapping_sub(1))
                        .and_then(|r| r.get(x.wrapping_add(1))),
                    grid.get(y.wrapping_sub(1))
                        .and_then(|r| r.get(x.wrapping_sub(1))),
                ];

                if corners.iter().all(Option::is_some) {
                    let corners: Vec<&char> = corners.into_iter().map(Option::unwrap).collect();
                    if (corners[0] == &'M' && corners[3] == &'S'
                        || corners[0] == &'S' && corners[3] == &'M')
                        && (corners[1] == &'M' && corners[2] == &'S'
                            || corners[1] == &'S' && corners[2] == &'M')
                    {
                        found += 1;
                    }
                }
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
