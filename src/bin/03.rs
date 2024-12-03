extern crate regex;

advent_of_code::solution!(3);
pub fn part_one(input: &str) -> Option<u32> {
    // run a regular expression on the string and extract all matching groups
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum:u32 = 0;
    for cap in re.captures_iter(input) {
        let (min, max) = (
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap()
        );
        sum += min * max;
        }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"(do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let re2 = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum:u32 = 0;
    let mut enabled_state = true;
    for cap in re.captures_iter(input) {
        let act = &cap[0];
        if act == "do()" {
            enabled_state = true;
        } else if act == "don't()" {
            enabled_state = false;
        } else {
            if enabled_state {
            let cap2 = re2.captures(act).unwrap();
            let (min, max) = (
                cap2[1].parse::<u32>().unwrap(),
                cap2[2].parse::<u32>().unwrap()
            );
            sum += min * max;
            }
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
