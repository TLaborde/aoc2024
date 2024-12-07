advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let calibrations = parse_input(input);
    let sum: u64 = calibrations
        .iter()
        .filter(|calibration| check_calibration(calibration))
        .map(|calibration| calibration[0])
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let calibrations = parse_input(input);
    let sum: u64 = calibrations
        .iter()
        .filter(|calibration| check_calibration2(calibration))
        .map(|calibration| calibration[0])
        .sum();
    Some(sum)
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| {
                    n.chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u64>()
                        .ok()
                })
                .collect()
        })
        .collect()
}

fn check_calibration(calibration: &Vec<u64>) -> bool {
    let mut partial_sum: Vec<u64> = vec![calibration[1]];
    for &value in &calibration[2..] {
        let mut new_partial_sum = Vec::new();
        for &sum in &partial_sum {
            let s = sum + value;
            let m = sum * value;
            if s <= calibration[0] {
                new_partial_sum.push(s);
            }
            if m <= calibration[0] {
                new_partial_sum.push(m);
            }
        }
        partial_sum = new_partial_sum;
    }
    partial_sum.contains(&calibration[0])
}

fn check_calibration2(calibration: &Vec<u64>) -> bool {
    let mut partial_sum: Vec<u64> = vec![calibration[1]];
    for &value in &calibration[2..] {
        let mut new_partial_sum = Vec::new();
        for &sum in &partial_sum {
            let s = sum + value;
            let m = sum * value;
            let num_digits = (value as f64).log10().floor() as usize + 1;
            let c = sum * (10_usize.pow(num_digits as u32) as u64) + value;
            if s <= calibration[0] {
                new_partial_sum.push(s);
            }
            if m <= calibration[0] {
                new_partial_sum.push(m);
            }
            if c <= calibration[0] {
                new_partial_sum.push(c);
            }
        }
        partial_sum = new_partial_sum;
    }
    partial_sum.contains(&calibration[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
