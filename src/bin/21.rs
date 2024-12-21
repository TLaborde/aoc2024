use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    // define a graph with a hashmap

            let mut num_keymap:std::collections::HashMap<char,Vec<(char,char)>> = std::collections::HashMap::new();
            num_keymap.insert('A', vec![('0','<'), ('3','^')]);
            num_keymap.insert('0', vec![('2','^'), ('A','>')]);
            num_keymap.insert('1', vec![('4','^'), ('2','>')]);
            num_keymap.insert('2', vec![('1','<'), ('0','v'), ('3','>'), ('5','^')]);
            num_keymap.insert('3', vec![('2','<'), ('A','v'), ('6','^')]);
            num_keymap.insert('4', vec![('1','v'), ('5','>'), ('7','^')]);
            num_keymap.insert('5', vec![('2','v'), ('4','<'), ('6','>'), ('8','^')]);
            num_keymap.insert('6', vec![('3','v'), ('5','<'), ('9','^')]);
            num_keymap.insert('7', vec![('4','v'), ('8','>')]);
            num_keymap.insert('8', vec![('5','v'), ('7','<'), ('9','>')]);
            num_keymap.insert('9', vec![('6','v'), ('8','<')]);


            let mut dir_keymap:std::collections::HashMap<char,Vec<(char,char)>> = std::collections::HashMap::new();
            dir_keymap.insert('A', vec![('^', '<'), ('>', 'v')]);
            dir_keymap.insert('^', vec![('A', '>'), ('v', 'v')]);
            dir_keymap.insert('>', vec![('A', '^'), ('v', '<')]);
            dir_keymap.insert('v', vec![('<', '<'), ('^', '^'), ('>', '>')]);
            dir_keymap.insert('<', vec![('v', '>')]);

    //let memory = std::collections::HashMap::new();
    let mut total = 0;
    for line in input.lines() {
        let mut start = 'A';
    let mut sum = 0;
    for next in line.chars() {
        let presses = get_cheapest(&num_keymap ,&dir_keymap,start, next, 3);
        sum += presses;
        start = next;
    }
    // remove final char from line and parse to int
    let entry = line[..line.len()-1].parse::<u32>().unwrap();
    println!("entry: {}, sum: {}", entry, sum);
    total += sum*entry;
    }
    Some(total)
}

fn get_best_dirpad (num_keymap:&std::collections::HashMap<char,Vec<(char,char)>> ,dir_keymap:&std::collections::HashMap<char,Vec<(char,char)>>, start: char, end: char, level: u32) -> u32 {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start, String::new()));
    let mut min_presses = u32::MAX;
    while !queue.is_empty() {
        let (current, sequence) = queue.pop_front().unwrap();
        if current == end {
            // add A to sequence string
            let mut full_sequence = sequence.clone();
            full_sequence.push('A');
            let presses = get_best_robot(num_keymap, dir_keymap, full_sequence, level-1);
            if presses < min_presses {
                min_presses = presses;
            }
            continue;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        for (neighbor, seq) in dir_keymap.get(&current).unwrap() {
            // add neightbor.1 to sequence string
            let mut new_sequence = sequence.clone();
            new_sequence.push(*seq);
            queue.push_back((*neighbor, new_sequence));
        }
    }
   min_presses
}



fn get_cheapest(num_keymap:&std::collections::HashMap<char,Vec<(char,char)>> ,dir_keymap:&std::collections::HashMap<char,Vec<(char,char)>>, start: char, end: char, level: u32) -> u32 {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start, String::new()));
    let mut min_presses = u32::MAX;
    while !queue.is_empty() {
        let (current, sequence) = queue.pop_front().unwrap();
        if current == end {
            // add A to sequence string
            let mut full_sequence = sequence.clone();
            full_sequence.push('A');
            let presses = get_best_robot(num_keymap, dir_keymap, full_sequence, level);
            if presses < min_presses {
                min_presses = presses;
            }
            continue;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        for (neighbor, seq) in num_keymap.get(&current).unwrap() {
            let mut new_sequence = sequence.clone();
            new_sequence.push(*seq);
            queue.push_back((*neighbor, new_sequence));
        }
    }
    min_presses
}


fn get_best_robot(num_keymap:&std::collections::HashMap<char,Vec<(char,char)>> ,dir_keymap:&std::collections::HashMap<char,Vec<(char,char)>>,sequence: String, level: u32) -> u32 {
    if level == 1 {
        return sequence.len() as u32;
    }
    let mut start = 'A';
    let mut sum = 0;
    for next in sequence.chars() {
        let presses = get_best_dirpad(num_keymap ,dir_keymap,start, next, level);
        sum += presses;
        start = next;
    }
    sum
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
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
