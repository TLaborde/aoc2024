use core::num;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    // input is a single line, we go over each character and parse it as a digit
    let numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    // we save the alternating numbers into two vectors, one storage and one free space
    let mut storage = Vec::new();
    let mut free = Vec::new();
    for i in 0..numbers.len() {
        if i % 2 == 0 {
            storage.push(numbers[i]);
        } else {
            free.push(numbers[i]);
        }
    }
    let mut new_storage = Vec::new();
    let mut index = 0;
    let mut last_index = storage.len() - 1;
    while storage.len() > 0 {
        let current = storage.remove(0);
        for i in 0..current {
            new_storage.push(index);
        }
        index += 1;

        let free_storage = free.remove(0);
        let mut filled_free_storage = 0;
        while filled_free_storage < free_storage {
            if storage.len() == 0 {
                break;
            }
            let mut l = storage.pop().unwrap();
            new_storage.push(last_index);
            filled_free_storage += 1;
            l -= 1;
            if l > 0 {
                storage.push(l as u32);
            } else {
                last_index -= 1;
            }
        }
    }
    // in new storage, sum the product of the index and the value at that index
    let mut sum: u64 = 0;
    for i in 0..new_storage.len() {
        sum += i as u64 * (new_storage[i] as u64);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    // input is a single line, we go over each character and parse it as a digit
    let numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    // we save the alternating numbers into two vectors, one storage and one free space
    let mut storage = Vec::new();
    let mut free = Vec::new();
    for i in 0..numbers.len() {
        if i % 2 == 0 {
            storage.push((i / 2, numbers[i]));
        } else {
            free.push(numbers[i]);
        }
    }
    let mut new_storage = Vec::new();
    let mut index = 0;
    let mut last_index = storage.len() - 1;
    let mut iteration_left = storage.len();
    while last_index > 0 {
        // read the value in last index
        let mut last_value = storage[last_index];
        if last_value.0 > iteration_left {
            last_index -= 1;
            continue;
        }
        iteration_left -= 1;
        println!(
            "trying to move {:?}, current last index is {}",
            last_value, last_index
        );
        // find in free the index of the first value that is bigger or equal to the last value
        let mut free_index = 0;
        while free_index < last_index && free[free_index] < last_value.1 {
            free_index += 1;
        }
        if free_index == last_index {
            last_index -= 1;
            continue;
        }
        // update free storage
        free[free_index] -= last_value.1;
        // insert a zero free before the free_index
        free.insert(free_index, 0);
        // remove the last value from storage
        storage.remove(last_index);
        storage.insert(free_index + 1, (last_value.0, last_value.1));

        // add the free space block
        // get space before ahd after the last value
        let mut space_around = free.remove(last_index);
        if last_index < free.len() {
            space_around += free.remove(last_index);
        }
        free.insert(last_index, space_around + last_value.1);
    }
    let mut s = storage.clone();
    let mut f = free.clone();
    new_storage.clear();
    while s.len() > 0 {
        let current = s.remove(0);
        for i in 0..current.1 {
            new_storage.push(current.0);
        }
        let current = f.remove(0);
        for i in 0..current {
            new_storage.push(0);
        }
    }
    println!("{:?}", new_storage);
    let mut sum: u64 = 0;
    for i in 0..new_storage.len() {
        sum += i as u64 * (new_storage[i] as u64);
    }
    Some(sum)
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
