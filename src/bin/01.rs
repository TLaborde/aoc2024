use std::str::Lines;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // split input into array of lines

    let Lines = input.lines();
    // iterate over lines, grab the first column of number
    let column1 = Lines.clone().map(|line| {
        line.split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    });
    // iterate over lines, grab the second column of numbers
    let column2 = Lines.clone().map(|line| {
        line.split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap()
    });
    // sort the columns
    let mut column1 = column1.collect::<Vec<_>>();
    column1.sort();
    let mut column2 = column2.collect::<Vec<_>>();
    column2.sort();

    // find the sum of the differences between the columns
    let mut sum: u32 = 0;
    sum = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a.max(b) - a.min(b)))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
