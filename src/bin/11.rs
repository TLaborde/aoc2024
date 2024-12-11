advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut stone_map = std::collections::HashMap::new();
    let total: u64 = stones.iter()
        .map(|&stone| recur_stone_count(stone, 25, &mut stone_map))
        .sum();

    Some(total)
}

fn recur_stone_count(stone: u64, blinks: u64, stone_map: &mut std::collections::HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(&count) = stone_map.get(&(stone, blinks)) {
        return count;
    }

    let return_value = if stone == 0 {
        recur_stone_count(1, blinks - 1, stone_map)
    } else if (stone as f64).log10().floor() as u64 % 2 == 1 {
        let num_digits_to_split = ((stone as f64).log10().floor() as u64 + 1) / 2;
        let divisor = 10_u64.pow(num_digits_to_split as u32);
        let left_part = stone / divisor;
        let right_part = stone % divisor;
        recur_stone_count(left_part, blinks - 1, stone_map) + recur_stone_count(right_part, blinks - 1, stone_map)
    } else {
        recur_stone_count(stone * 2024, blinks - 1, stone_map)
    };

    stone_map.insert((stone, blinks), return_value);
    return_value
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut stone_map = std::collections::HashMap::new();
    let total: u64 = stones.iter()
        .map(|&stone| recur_stone_count(stone, 75, &mut stone_map))
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
