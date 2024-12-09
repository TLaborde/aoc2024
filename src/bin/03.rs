extern crate regex;

advent_of_code::solution!(3);
pub fn part_one(input: &str) -> Option<u32> {
    // run a regular expression on the string and extract all matching groups
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum: u32 = re
        .captures_iter(input)
        .filter_map(|cap| {
            let min = cap[1].parse::<u32>().ok()?;
            let max = cap[2].parse::<u32>().ok()?;
            Some(min * max)
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"(do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let re2 = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum: u32 = 0;
    let mut enabled_state = true;

    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => enabled_state = true,
            "don't()" => enabled_state = false,
            act if enabled_state => {
                if let Some(cap2) = re2.captures(act) {
                    if let (Ok(min), Ok(max)) = (cap2[1].parse::<u32>(), cap2[2].parse::<u32>()) {
                        sum += min * max;
                    }
                }
            }
            _ => {}
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
