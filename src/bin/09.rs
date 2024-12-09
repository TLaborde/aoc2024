advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut storage, mut free) = parse_input(input);
    let mut new_storage = Vec::new();
    let mut index = 0;
    let mut last_index = storage.len() - 1;

    while !storage.is_empty() {
        let current = storage.remove(0);
        for _ in 0..current.1 {
            new_storage.push(index);
        }
        index += 1;

        let free_storage = free.remove(0);
        let mut filled_free_storage = 0;
        while filled_free_storage < free_storage {
            if storage.is_empty() {
                break;
            }
            let mut l = storage.pop().unwrap();
            new_storage.push(last_index);
            filled_free_storage += 1;
            l.1 -= 1;
            if l.1 > 0 {
                storage.push(l);
            } else {
                last_index -= 1;
            }
        }
    }

    let sum: u64 = new_storage
        .iter()
        .enumerate()
        .map(|(i, &v)| i as u64 * v as u64)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut storage, mut free) = parse_input(input);
    let mut new_storage = Vec::new();
    let mut last_index = storage.len() - 1;
    let mut iteration_left = storage.len() as u32;

    while last_index > 0 {
        let last_value = storage[last_index];
        // check that we haven't already moved the value
        // there might be a nicer way to do this
        if last_value.0 > iteration_left {
            last_index -= 1;
            continue;
        }
        iteration_left -= 1;

        // try to find some free space
        // we only go as far as the last index, because we can only shift to the left
        let mut free_index = 0;
        while free_index < last_index && free[free_index] < last_value.1 {
            free_index += 1;
        }
        // we didn't find a place to move it
        if free_index == last_index {
            last_index -= 1;
            continue;
        }

        // we update the free space that we are filling
        free[free_index] -= last_value.1;
        // we add an empty free space to match where we insert the value in storage
        free.insert(free_index, 0);
        storage.insert(free_index + 1, last_value);
        // we remove the value that we moved
        storage.remove(last_index);

        // we have now possibly 2 spaces around where the value was, and the space left by the value, so we combine them into one contiguous space
        let mut space_around = free.remove(last_index);
        if last_index < free.len() {
            space_around += free.remove(last_index);
        }
        free.insert(last_index, space_around + last_value.1);
    }

    // generate the final storage for calculation
    for (i, &(index, count)) in storage.iter().enumerate() {
        new_storage.extend(vec![index; count as usize]);
        if i < free.len() {
            new_storage.extend(vec![0; free[i] as usize]);
        }
    }

    let sum: u64 = new_storage
        .iter()
        .enumerate()
        .map(|(i, &v)| i as u64 * v as u64)
        .sum();
    Some(sum)
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<u32>) {
    let numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut storage = Vec::new();
    let mut free = Vec::new();
    for (i, &num) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            storage.push(((i / 2) as u32, num));
        } else {
            free.push(num);
        }
    }
    (storage, free)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
