use std::collections::hash_map;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<usize> {
    // parse input each line as usize
    let lines = input.lines();
    let mut sum = 0;
    let mut result = 0;
    // make hashmap to store previous results
    let mut previous_results = hash_map::HashMap::new();
    for line in lines {
        let mut secret = line.parse::<usize>().unwrap();
        for _ in 0..2000 {
            if let Some(&value) = previous_results.get(&secret) {
                secret = value;
                continue;
            }
            let previous = secret;
            result = secret * 64;
            secret = mix(result, secret);
            secret = prune(secret);
            result = secret / 32;
            secret = mix(result, secret);
            secret = prune(secret);
            result = secret * 2048;
            secret = mix(result, secret);
            secret = prune(secret);
            previous_results.insert(previous, secret);
        }
        sum += secret;
    }
    Some(sum)
}

pub fn prune( secret: usize) -> usize {
    secret % 16777216
}

pub fn mix (value: usize, secret: usize) -> usize {
    value ^ secret
}

pub fn part_two(input: &str) -> Option<usize> {
        // parse input each line as usize
        let lines = input.lines();
        let mut result = 0;
        let mut all_monkey_sequences = hash_map::HashMap::new();
        // make hashmap to store previous results
        for line in lines {
            let mut secret = line.parse::<usize>().unwrap();
            let mut five_previous:Vec<isize> = vec![];
            let mut monkey_sequences = hash_map::HashMap::new();
            for _ in 0..2000  {

                result = secret * 64;
                secret = mix(result, secret);
                secret = prune(secret);
                result = secret / 32;
                secret = mix(result, secret);
                secret = prune(secret);
                result = secret * 2048;
                secret = mix(result, secret);
                secret = prune(secret);

                five_previous.push((secret% 10) as isize);
                if five_previous.len()== 5 {
                    let var:(isize,isize,isize,isize) = (five_previous[1] - five_previous[0], five_previous[2] - five_previous[1], five_previous[3] - five_previous[2], five_previous[4] - five_previous[3]);
                    // if sequence is not in hashmap, add it
                    if !monkey_sequences.contains_key(&var) {
                        monkey_sequences.insert(var, secret% 10);
                    }
                    five_previous.remove(0);
                }
            }

            for (key, value) in monkey_sequences.iter() {
                if !all_monkey_sequences.contains_key(key) {
                    all_monkey_sequences.insert(key.clone(), value.clone());
                } else {
                    let val = all_monkey_sequences.get_mut(key).unwrap();
                    *val = *val + value;
                }
            }

        }
        // find key with max value
        let mut max = 0;
        let mut max_key = (0, 0, 0, 0);
        for (key, value) in all_monkey_sequences.iter() {
            if *value > max {
                max = *value;
                max_key = key.clone();
            }
        }
        // print all monkey sequences

        println!("max_key: {:?}", max_key);
        Some(max)
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
