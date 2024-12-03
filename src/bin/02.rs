advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // split input into array of lines
    let lines = input.lines();
    //iterate over lines, split by whitespace, get an array of number
    let numbers = lines.map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    // for each line, run a check function
    let mut sum: u32 = 0;
    for line in numbers {
        let growing = line[0] < line[1];
        let danger = line.windows(2).any(|pair| {
            let (j, k) = (pair[0], pair[1]);
            if growing {
                j > k || k - j > 3 || j == k
            } else {
                j < k || j - k > 3 || j == k
            }
        });
        if !danger {
            sum += 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // split input into array of lines
    let lines = input.lines();
    //iterate over lines, split by whitespace, get an array of number
    let numbers = lines.map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    // for each line, run a check function
    let mut sum: u32 = 0;
    for line in numbers {
        let safe = (0..line.len()).any(|skip| {
            let mut line_clone = line.clone();
            line_clone.remove(skip);
            let growing = line_clone[0] < line_clone[1];
            !line_clone.windows(2).any(|pair| {
                let (j, k) = (pair[0], pair[1]);
                if growing {
                    j > k || k - j > 3 || j == k
                } else {
                    j < k || j - k > 3 || j == k
                }
            })
        });
        if safe {
            sum += 1;
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
