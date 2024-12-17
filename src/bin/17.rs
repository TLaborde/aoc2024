use std::collections::HashMap;

advent_of_code::solution!(17);

pub fn get_combovalue(register: &Vec<i64>, operand: i64) -> i64 {
    match operand {
        4 | 5 | 6 => register[operand as usize - 4],
        0 | 1 | 2 | 3 => operand,
        _ => {
            panic!("Invalid operand");
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    //input format
    //Register A: 729
    //Register B: 0
    //Register C: 0
    //
    //Program: 0,1,5,4,3,0
    // save register into mutable variable

    let mut registers = input
        .lines()
        .take(3)
        .map(|line| {
            line.split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>();

    let program = input
        .lines()
        .skip(4)
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // group program by pair
    //let program = program.chunks(2).collect::<Vec<_>>();

    // run program
    let output = run_program(&program, &mut registers, false);
    // join output values with comma
    Some(
        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn run_program(program: &Vec<i64>, registers: &mut Vec<i64>, self_gen: bool) -> Vec<i64> {
    // for each operation operand in program:
    let mut i = 0;
    let mut output_values = Vec::new();
    while i < program.len() {
        let operation = program[i];
        let operand = program[i + 1];
        // match operand
        match operation {
            // if operation is 0, adv
            0 => {
                registers[0] =
                    (registers[0] / 2_i64.pow(get_combovalue(registers, operand) as u32)) as i64;
            }
            // if operand is 1, bxl
            1 => {
                // do a bitwaise XOR or register 1 and operand 1
                registers[1] ^= operand;
            }
            // if operand is 2, bst
            2 => {
                // multiply register by operand 1
                registers[1] = get_combovalue(registers, operand) % 8;
            }
            // if operand is 3, jnz
            3 => {
                if registers[0] != 0 {
                    i = operand as usize;
                    continue;
                }
            }
            // if operand is 4, bxc
            4 => {
                registers[1] ^= registers[2];
            }
            // if operand is 5, out
            5 => {
                output_values.push(get_combovalue(registers, operand) % 8);
                //if self_gen {
                //    if output_values.len() == program.len() {
                //        return output_values;
                //    }
                //}
            }
            // if operand is 6, bdv
            6 => {
                registers[1] =
                    (registers[0] / 2_i64.pow(get_combovalue(registers, operand) as u32)) as i64;
            }
            // if operand is 7, cdv
            7 => {
                registers[2] =
                    (registers[0] / 2_i64.pow(get_combovalue(registers, operand) as u32)) as i64;
            }
            _ => {
                panic!("Invalid operation");
            }
        };

        i += 2;
        // join output values with comma
    }
    output_values
}
pub fn part_two(input: &str) -> Option<i64> {
    let registers = input
        .lines()
        .take(3)
        .map(|line| {
            line.split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>();

    let program = input
        .lines()
        .skip(4)
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();


    let mut current_exp = (program.len() as u32 / 2) -1;
    let mut a = 0;
    let mut matching_bits = 2;
    while current_exp > 0 {
        // last 2 digits in program as vector
        let current_target = program[program.len() - matching_bits..].to_vec();

        // find the value of a that will generate the output
        loop {
            let mut registers = registers.clone();
            registers[0] = a;
            let output = run_program(&program, &mut registers, true);
            if output.len() >= matching_bits
            && output[output.len() - matching_bits..] == current_target
            {
                println!("a: {}, output: {:?}", a, output);
                break;
            }
            a += 64_i64.pow(current_exp);
        }

        // update target
        current_exp -= 1;
        matching_bits += 2;
    }
    let current_target = program.clone();
    loop {
        let mut registers = registers.clone();
        registers[0] = a;
        let output = run_program(&program, &mut registers, true);
        if output == current_target
        {
            println!("a: {}, output: {:?}", a, output);
            break;
        }
        a += 1;
    }


    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //fn test_part_one() {
    //    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //    assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    //}
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
