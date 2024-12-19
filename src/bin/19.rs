use std::collections::{HashSet, HashMap};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let (patterns, designs) = parse_input(input);
    let mut known_failures = HashSet::new();
    let count = designs.iter().filter(|&&design| check_design(design, &patterns, &mut known_failures)).count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse_input(input);
    let mut known_failures = HashSet::new();
    let mut known_successes = HashMap::new();
    let count = designs.iter().map(|&design| count_design(design, &patterns, &mut known_failures, &mut known_successes)).sum();
    Some(count)
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    lines.next(); // Skip the empty line
    let designs = lines.collect::<Vec<&str>>();
    (patterns, designs)
}

pub fn check_design(design: &str, patterns: &[&str], known_failures: &mut HashSet<String>) -> bool {
    if known_failures.contains(design) {
        return false;
    }
    for &pattern in patterns {
        if design == pattern {
            return true;
        }
        if design.starts_with(pattern) {
            if check_design(&design[pattern.len()..], patterns, known_failures) {
                return true;
            }
        }
    }
    known_failures.insert(design.to_string());
    false
}

pub fn count_design(design: &str, patterns: &[&str], known_failures: &mut HashSet<String>, known_successes: &mut HashMap<String, u64>) -> u64 {
    if known_failures.contains(design) {
        return 0;
    }
    if let Some(&count) = known_successes.get(design) {
        return count;
    }
    let mut count = 0;
    for &pattern in patterns {
        if design == pattern {
            count += 1;
        }
        if design.starts_with(pattern) {
            count += count_design(&design[pattern.len()..], patterns, known_failures, known_successes);
        }
    }
    if count == 0 {
        known_failures.insert(design.to_string());
    } else {
        known_successes.insert(design.to_string(), count);
    }
    count
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
