advent_of_code::solution!(5);
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    // split input on the empty line
    let mut parts = input.split("\n\n");
    // for each line in the first part, split on | to get 2 integers
    let rules = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split("|");
        let page_before = parts.next().unwrap().parse::<u32>().unwrap();
        let page_after = parts.next().unwrap().parse::<u32>().unwrap();
        (page_before, page_after)
    }).collect::<Vec<(u32, u32)>>();
    // the second part are array of integet separated by comma
    let manuals = parts.next().unwrap().lines().collect::<Vec<&str>>();
    // for each manual, split on comma and parse each integer
    let manual_pages = manuals.iter().map(|manual| {
        manual.split(",").map(|page| page.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    // for each manual, if the index and rule are in the array, the index need to be before rule
    let mut valid = 0;
    'manual_loop: for manual in manual_pages.iter() {
        for rule in rules.iter() {
            if manual.contains(&rule.0) && manual.contains(&rule.1) {
                if manual.iter().position(|&x| x == rule.0).unwrap() < manual.iter().position(|&x| x == rule.1).unwrap() {
                    continue;
                } else {
                    // go to next manual
                    continue 'manual_loop;
                }
            }
        }
        // add to valid the value in the middle of the manual array
        valid += manual[manual.len() / 2];
    }
    //
    Some(valid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    // for each line in the first part, split on | to get 2 integers
    let rules = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split("|");
        let page_before = parts.next().unwrap().parse::<u32>().unwrap();
        let page_after = parts.next().unwrap().parse::<u32>().unwrap();
        (page_before, page_after)
    }).collect::<Vec<(u32, u32)>>();

    // create a sorting function that will sort the manual pages based on the rules
    let sort = |a: &u32, b: &u32| {
            for rule in rules.iter() {
                if a == &rule.0 && b == &rule.1 {
                    return Ordering::Less;
                }
                if a == &rule.1 && b == &rule.0 {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        };

    // the second part are array of integet separated by comma
    let manuals = parts.next().unwrap().lines().collect::<Vec<&str>>();
    // for each manual, split on comma and parse each integer
    let manual_pages = manuals.iter().map(|manual| {
        manual.split(",").map(|page| page.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    // sort each manual, compare the sorted manual with the original manual
    let mut valid = 0;
    for manual in manual_pages.iter() {
        let mut sorted = manual.clone();
        sorted.sort_by(sort);
        if sorted != *manual {
            valid += sorted[sorted.len() / 2];
        }
    }
    Some(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
