advent_of_code::solution!(24);
use std::collections::{HashMap, HashSet, VecDeque};

// create a struct with 4 strings

#[derive(Clone, Debug)]
pub struct Record {
    pub val1: String,
    pub val2: String,
    pub operation: String,
    pub res: String,
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut stored_values, records) = parse_input(input);

    calculate(&mut stored_values, records.clone());
    // get all key strting with z, sort them i ndescending order
    let mut res = stored_values
        .keys()
        .filter(|x| x.starts_with("z"))
        .collect::<Vec<&String>>();
    res.sort_by(|a, b| b.cmp(a));
    let mut binary_number = String::new();
    for r in res.iter() {
        binary_number.push(if stored_values[*r] { '1' } else { '0' });
    }
    let result = usize::from_str_radix(&binary_number, 2).unwrap();
    Some(result)
}

fn calculate(stored_values: &mut HashMap<String, bool>, mut records: VecDeque<Record>) {
    // count number of records with "res" starting with "z"
    let mut count = 0;
    for record in records.iter() {
        if record.res.starts_with("z") {
            count += 1;
        }
    }
    while count > 0 {
        // print records count
        let record = records.pop_front().unwrap();
        if stored_values.contains_key(&record.val1) && stored_values.contains_key(&record.val2) {
            println!("{:?}", records.len());
            let val1 = stored_values[&record.val1];
            let val2 = stored_values[&record.val2];
            let res = match record.operation.as_str() {
                "AND" => val1 & val2,
                "OR" => val1 | val2,
                "XOR" => val1 ^ val2,
                _ => val2,
            };
            if record.res.starts_with("z") {
                count -= 1;
            }
            stored_values.insert(record.res, res);
        } else {
            records.push_back(record);
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, bool>, VecDeque<Record>) {
    let lines = input.lines();
    let mut stored_values = HashMap::new();
    let mut records = VecDeque::new();
    for line in lines {
        if line.contains(":") {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().parse::<u32>().unwrap() == 1;
            stored_values.insert(key, value);
        }
        if line.contains("->") {
            let mut parts = line.split(" ");
            let val1 = parts.next().unwrap().to_string();
            let operation = parts.next().unwrap().to_string();
            let val2 = parts.next().unwrap().to_string();
            parts.next().unwrap(); // skip ->
            let res = parts.next().unwrap().to_string();
            let record = Record {
                val1,
                operation,
                val2,
                res,
            };
            records.push_back(record);
        }
    }
    (stored_values, records)
}

type Nodes = HashSet<String>;
type Graph = HashMap<String, Nodes>;

pub fn part_two(input: &str) -> Option<String> {
    let (mut stored_values, mut records) = parse_input(input);

    //create a graph based on records,
    let mut graph = Graph::new();
    for record in records.iter() {
        if !graph.contains_key(&record.res) {
            graph.insert(record.res.clone(), Nodes::new());
        }
        if !graph.contains_key(&record.val1) {
            graph.insert(record.val1.clone(), Nodes::new());
        }
        if !graph.contains_key(&record.val2) {
            graph.insert(record.val2.clone(), Nodes::new());
        }
        graph
            .get_mut(&record.res)
            .unwrap()
            .insert(record.val1.clone());
        graph
            .get_mut(&record.res)
            .unwrap()
            .insert(record.val2.clone());
    }

    // previous is XOR
    // (y01 AND x01 OR ((y01 XOR x01) AND (y00 AND x00))) XOR (y02 XOR x02) -> z02
    //  (tjn OR cfv) XOR (x03 XOR y03) -> z03

    // find all records with a results starting with z, check if the operation is a XOR
    let mut bad = Vec::new();

    for z in 2..44 {
        let current_z = format!("z{:02}", z);
        let current_record = records
            .clone()
            .into_iter()
            .filter(|x| x.res == current_z)
            .next()
            .unwrap();
        if current_record.operation == "XOR" {
            // find the previous XOR record
            let prev_x = format!("x{:02}", z );
            let prev_y = format!("y{:02}", z );
            let prev_record = records
                .clone()
                .into_iter()
                .filter(|x| {
                    ((x.val1 == prev_x && x.val2 == prev_y)
                        || (x.val1 == prev_y && x.val2 == prev_x))
                        && x.operation == "XOR"
                })
                .next()
                .unwrap();
            // check if the result is current_record.val2
            if prev_record.res != current_record.val2 && prev_record.res != current_record.val1 {
                println!("wrong record {:?}", current_record);
                let val2 = records
                    .clone()
                    .into_iter()
                    .filter(|x| x.res == prev_record.res && x.operation == "XOR")
                    .next()
                    .unwrap();
                println!("good record {:?}", val2);
                bad.push(val2.res.clone());
                //next line is hardcoded somehow
                bad.push(current_record.val2.clone());
            }

        } else {
            //println!("wrong record point to z {:?}", current_record);
            // let's find the rights records
            // find the previous XOR record
            let prev_x = format!("x{:02}", z );
            let prev_y = format!("y{:02}", z );
            let prev_record = records
                .clone()
                .into_iter()
                .filter(|x| {
                    ((x.val1 == prev_x && x.val2 == prev_y)
                        || (x.val1 == prev_y && x.val2 == prev_x))
                        && x.operation == "XOR"
                })
                .next()
                .unwrap();
            // find a record with the result of prev_record and a XOR operation
            let val2 = records
                .clone()
                .into_iter()
                .filter(|x| (prev_record.res == x.val1 || prev_record.res == x.val2) && x.operation == "XOR")
                .next()
                .unwrap();

            //println!("good record {:?}", val2);
            bad.push(val2.res.clone());
                bad.push(current_record.res.clone());
        }
    }
// sort bad, join with comma
    bad.sort();
    let result = bad.join(",");
println!("{:?}", result);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
