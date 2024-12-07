advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    // split input per line
    let lines: Vec<&str> = input.lines().collect();
    // split each line on space and , cast all to int
    let calibrations: Vec<Vec<u64>> = lines
        .iter()
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
        .collect();
    // for each line, run a check function
    let mut sum: u64 = 0;
    for calibration in &calibrations {
        // check if the calibration is valid
        if check_calibration(calibration) {
            sum += calibration[0];
        }
    }
    Some(sum)
}

fn check_calibration(calibration: &Vec<u64>) -> bool {
    let mut partial_sum: Vec<u64> = Vec::new();
    partial_sum.push(calibration[1]);
    for i in 2..calibration.len() {
        let mut new_partial_sum = Vec::new();
        for sum in &partial_sum {
            let s = sum + calibration[i];
            let m = sum * calibration[i];
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
    let mut partial_sum: Vec<u64> = Vec::new();
    partial_sum.push(calibration[1]);
    for i in 2..calibration.len() {
        let mut new_partial_sum = Vec::new();
        for sum in &partial_sum {
            let s = sum + calibration[i];
            let m = sum * calibration[i];
            let num_digits = (calibration[i] as f64).log10().floor() as usize + 1;
            let c = sum * (10_usize.pow(num_digits as u32) as u64) + calibration[i];
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
pub fn part_two(input: &str) -> Option<u64> {
    // split input per line
    let lines: Vec<&str> = input.lines().collect();
    // split each line on space and , cast all to int
    let calibrations: Vec<Vec<u64>> = lines
        .iter()
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
        .collect();
    // for each line, run a check function
    let mut sum: u64 = 0;
    for calibration in &calibrations {
        // check if the calibration is valid
        if check_calibration2(calibration) {
            sum += calibration[0];
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
