advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    //parse input, split on whitespace, cast as u64
    let stones = input.split_whitespace()
    .map(|n| n.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();
// create an empty hastable to match stone to next step
    let mut stone_map: std::collections::HashMap<(u64,u64), u64> = std::collections::HashMap::new();
    let mut total = 0;
    for stone in stones.iter() {
        total += recur_stone_count(*stone, 25, &mut stone_map);
    }
    Some(total)
}

fn recur_stone_count(stone: u64 ,blinks: u64, stone_map: & mut std::collections::HashMap<(u64,u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if stone_map.contains_key(&(stone, blinks)) {
        return stone_map[&(stone, blinks)];
    }
    let mut return_value = 1;
    if stone == 0 {
        return recur_stone_count(1, blinks -1, stone_map);
    }
    if ((stone as f64).log10().floor() as u64 + 1) % 2 == 0 {
        let num_digits_to_split = ((stone as f64).log10().floor() as u64 + 1) / 2;
        let left_part = stone / (10_usize.pow(num_digits_to_split as u32) as u64);
        let right_part = stone % (10_usize.pow(num_digits_to_split as u32) as u64);
        return_value = recur_stone_count(left_part, blinks -1 , stone_map) + recur_stone_count(right_part, blinks -1, stone_map);
    } else {
        return_value = recur_stone_count(stone*2024, blinks -1, stone_map);
    }
    stone_map.insert((stone, blinks), return_value);
    return return_value;
}


pub fn part_two(input: &str) -> Option<u64> {
    //parse input, split on whitespace, cast as u64
    let stones = input.split_whitespace()
    .map(|n| n.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();
// create an empty hastable to match stone to next step
    let mut stone_map: std::collections::HashMap<(u64,u64), u64> = std::collections::HashMap::new();
    let mut total = 0;
    for stone in stones.iter() {
        total += recur_stone_count(*stone, 75, &mut stone_map);
    }
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
