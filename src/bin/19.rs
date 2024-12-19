use std::collections::{hash_set, HashSet, HashMap};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    // first line of input is a comma and space serapated list of patterns
    let mut iter = input.lines();
    let patterns = iter.next().unwrap().split(", ").collect::<Vec<&str>>();
    // sorts patterns by length then by lexicographic order
    let patterns = patterns.iter().map(|&x| x).collect::<Vec<&str>>();
    let mut known_failures: HashSet<String> = HashSet::new();
    iter.next(); // Skip the empty line
    let designs = iter.collect::<Vec<&str>>();
    let mut count = 0;
    for design in designs {
        if check_design(design, &patterns, &mut known_failures) {
            count += 1;
        }
    }
    Some(count)
}

pub fn check_design(
    design: &str,
    patterns: &Vec<&str>,
    known_failures: &mut HashSet<String>,
) -> bool {
    // check if the design starts with a pattern, if so recurse on the rest of the design
    if known_failures.contains(&(design.to_string())) {
        return false;
    }
    for pattern in patterns {
        if design == *pattern {
            return true;
        }
        if design.starts_with(pattern) {
            if check_design(&design[pattern.len()..], patterns, known_failures) {
                return true;
            }
        } else {
        }
    }
    known_failures.insert(design.to_string());
    false
}

pub fn count_design(
    design: &str,
    patterns: &Vec<&str>,
    known_failures: &mut HashSet<String>,
    knows_successes: &mut HashMap<String, u64>,
) -> u64 {
    // check if the design starts with a pattern, if so recurse on the rest of the design
    if known_failures.contains(&(design.to_string())) {
        return 0;
    }
    if let Some(&count) = knows_successes.get(design) {
        return count;
    }
    let mut count = 0;
    for pattern in patterns {
        if design == *pattern {
            count += 1;
        }
        if design.starts_with(pattern) {
            count += count_design(&design[pattern.len()..], patterns, known_failures,knows_successes);
        }
    }
    if count == 0 {
        known_failures.insert(design.to_string());
    } else {
        knows_successes.insert(design.to_string(), count);
    }
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    // first line of input is a comma and space serapated list of patterns
    let mut iter = input.lines();
    let patterns = iter.next().unwrap().split(", ").collect::<Vec<&str>>();
    // sorts patterns by length then by lexicographic order
    let patterns = patterns.iter().map(|&x| x).collect::<Vec<&str>>();
    let mut known_failures: HashSet<String> = HashSet::new();
    let mut knows_successes: HashMap<String, u64> = HashMap::new();
    iter.next(); // Skip the empty line
    let designs = iter.collect::<Vec<&str>>();
    let mut count: u64 = 0;
    for design in designs {
        count += count_design(design, &patterns, &mut known_failures, &mut knows_successes);
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
