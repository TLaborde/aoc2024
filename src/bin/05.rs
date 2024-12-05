advent_of_code::solution!(5);
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, manual_pages) = parse_input(input);

    let valid = manual_pages.iter()
        .filter(|manual| {
            let mut sorted = (*manual).clone();
            sorted.sort_by(|a, b| sort_manual_pages(a, b, &rules));
            sorted == **manual
        })
        .map(|manual| manual[manual.len() / 2])
        .sum();
    Some(valid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, manual_pages) = parse_input(input);

    let valid = manual_pages.iter()
        .map(|manual| {
            let mut sorted = (*manual).clone();
            sorted.sort_by(|a, b| sort_manual_pages(a, b, &rules));
            if sorted != **manual {
                sorted[sorted.len() / 2]
            } else {
                0
            }
        })
        .sum();
    Some(valid)
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split('|');
        let page_before = parts.next().unwrap().parse::<u32>().unwrap();
        let page_after = parts.next().unwrap().parse::<u32>().unwrap();
        (page_before, page_after)
    }).collect::<Vec<(u32, u32)>>();

    let manuals = parts.next().unwrap().lines().collect::<Vec<&str>>();
    let manual_pages = manuals.iter().map(|manual| {
        manual.split(',').map(|page| page.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    (rules, manual_pages)
}

fn sort_manual_pages(a: &u32, b: &u32, rules: &[(u32, u32)]) -> Ordering {
    for rule in rules.iter() {
        if a == &rule.0 && b == &rule.1 {
            return Ordering::Less;
        }
        if a == &rule.1 && b == &rule.0 {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
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
