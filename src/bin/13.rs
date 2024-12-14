advent_of_code::solution!(13);

// create a struct called claw machine
#[derive(Debug)]
pub struct ClawMachine {
    pub xa: f64,
    pub ya: f64,
    pub xb: f64,
    pub yb: f64,
    pub pricex: f64,
    pub pricey: f64,
}
pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let mut line = lines[i];
        let mut cm = ClawMachine {
            xa: 0.0,
            ya: 0.0,
            xb: 0.0,
            yb: 0.0,
            pricex: 0.0,
            pricey: 0.0,
        };
        let mut parts: Vec<&str> = line.split(|c| c == '+' || c == ',' || c == ' ').collect();
        cm.xa = parts[3].parse::<f64>().unwrap();
        cm.ya = parts[6].parse::<f64>().unwrap();
        // print full cm\
        i += 1;
        line = lines[i];
        parts = line.split(|c| c == '+' || c == ',' || c == ' ').collect();
        cm.xb = parts[3].parse::<f64>().unwrap();
        cm.yb = parts[6].parse::<f64>().unwrap();
        i += 1;
        line = lines[i];
        parts = line.split(|c| c == '=' || c == ',' || c == ' ').collect();
        cm.pricex = parts[2].parse::<f64>().unwrap();
        cm.pricey = parts[5].parse::<f64>().unwrap();
        claw_machines.push(cm);
        i += 2;
    }
    let mut total_cost = 0;

    for cm in claw_machines.iter() {
        let x = (cm.pricex * cm.yb - cm.pricey * cm.xb) / (cm.xa * cm.yb - cm.ya * cm.xb);
        let y =(cm.xa * cm.pricey - cm.ya * cm.pricex) / (cm.xa * cm.yb - cm.ya * cm.xb);
        if x.fract() <= 0.0 && y.fract() <= 0.0 {
            total_cost += (x * 3 as f64 + y ) as u64;
        }
    }

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let mut line = lines[i];
        let mut cm = ClawMachine {
            xa: 0.0,
            ya: 0.0,
            xb: 0.0,
            yb: 0.0,
            pricex: 0.0,
            pricey: 0.0,
        };
        let mut parts: Vec<&str> = line.split(|c| c == '+' || c == ',' || c == ' ').collect();
        cm.xa = parts[3].parse::<f64>().unwrap();
        cm.ya = parts[6].parse::<f64>().unwrap();
        // print full cm\
        i += 1;
        line = lines[i];
        parts = line.split(|c| c == '+' || c == ',' || c == ' ').collect();
        cm.xb = parts[3].parse::<f64>().unwrap();
        cm.yb = parts[6].parse::<f64>().unwrap();
        i += 1;
        line = lines[i];
        parts = line.split(|c| c == '=' || c == ',' || c == ' ').collect();
        cm.pricex = parts[2].parse::<f64>().unwrap() + 10000000000000f64;
        cm.pricey = parts[5].parse::<f64>().unwrap() + 10000000000000f64;
        claw_machines.push(cm);
        i += 2;
    }
    let mut total_cost = 0;
    for cm in claw_machines.iter() {
        let x = (cm.pricex * cm.yb - cm.pricey * cm.xb) / (cm.xa * cm.yb - cm.ya * cm.xb);
        let y =(cm.xa * cm.pricey - cm.ya * cm.pricex) / (cm.xa * cm.yb - cm.ya * cm.xb);
        if x.fract() <= 0.0 && y.fract() <= 0.0 {
            total_cost += (x * 3 as f64 + y ) as u64;
        }
    }

    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
