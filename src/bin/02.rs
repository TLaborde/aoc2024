advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // split input into array of lines
    let lines = input.lines();
    //iterate over lines, split by whitespace, get an array of number
    let numbers = lines.map(|line| line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    // for each line, run a check function
    let mut sum:u32 = 0;
    for line in numbers {
        let growing = line[0]<line[1];
        let mut danger = false;
        // check if the line is strictly growning or not
        for i in 0..line.len()-1 {
            let j = line[i];
            let k = line[i+1];
            if growing {
                if j > k || k-j > 3 || j == k {
                    danger = true;
                    break;
                }
            } else {
                if j < k || j-k > 3 || j == k {
                    danger = true;
                    break;
                }
            }
        }
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
    let numbers = lines.map(|line| line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    // for each line, run a check function
    let mut sum:u32 = 0;
    for line in numbers {
        let mut safe = false;
        for skip in 0..line.len() {
            let mut line = line.clone();
            line.remove(skip);
            let growing = line[0]<line[1];
            let mut danger = false;
            // check if the line is strictly growning or not
            for i in 0..line.len()-1 {
                let j = line[i];
                let k = line[i+1];
                if growing {
                    if j > k || k-j > 3 || j == k {
                        danger = true;
                        break;
                    }
                } else {
                    if j < k || j-k > 3 || j == k {
                        danger = true;
                        break;
                    }
                }
            }
            if !danger {
                safe = true;
            }
        }
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
