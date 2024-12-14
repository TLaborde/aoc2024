advent_of_code::solution!(14);

// create a struct to hold the position and velocity of a robot
#[derive(Debug, Default)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

pub fn part_one(input: &str) -> Option<u32> {
    // for each line in input looking like p=0,4 v=3,-3 extract the values
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut robot = Robot::default();
        for part in line.split(" ") {
            let mut parts = part.split("=");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            let mut values = value.split(",");
            let x = values.next().unwrap().parse().unwrap();
            let y = values.next().unwrap().parse().unwrap();
            match key {
                "p" => robot.position = (x, y),
                "v" => robot.velocity = (x, y),
                _ => panic!("Unknown key: {}", key),
            }
        }
        robots.push(robot);
    }
    let (width, height) = if robots.len() == 12 {
        //test run
        (11,7)
    } else {
        //real run
    (101,103)
    };
    // show width and height
    println!("width: {}, height: {}", -1 % width, height);
    // for each second, move the robots

    // move the robots
    for robot in robots.iter_mut() {
        robot.position.0 += 100 * robot.velocity.0;
        robot.position.1 += 100 * robot.velocity.1;
        while robot.position.0 < 0 {
            robot.position.0 += width;
        }
        while robot.position.1 < 0 {
            robot.position.1 += height;
        }
        robot.position.0 %= width;
        robot.position.1 %= height;

    }

    // count robot per quadrant
    let mut quadrants = vec![0; 4];
    for robot in robots.iter() {
        let quadrant = if robot.position.0 < width / 2 {
            if robot.position.1 < height / 2 {
                0
            } else if robot.position.1 > height / 2   {
                2
            } else {
                continue;
            }
        } else if robot.position.0 > width / 2 {
            if robot.position.1 < height / 2 {
                1
            } else if robot.position.1 > height / 2 {
                3
            } else {
                continue;
            }
        } else {
           continue;
        };
        quadrants[quadrant] += 1;
    }
    // multiphy the numbers in quadrants
    println!("{:?}", quadrants);
    let result = quadrants.iter().fold(1, |acc, x| acc * x);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut robot = Robot::default();
        for part in line.split(" ") {
            let mut parts = part.split("=");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            let mut values = value.split(",");
            let x = values.next().unwrap().parse().unwrap();
            let y = values.next().unwrap().parse().unwrap();
            match key {
                "p" => robot.position = (x, y),
                "v" => robot.velocity = (x, y),
                _ => panic!("Unknown key: {}", key),
            }
        }
        robots.push(robot);
    }
    let (width, height) = if robots.len() == 12 {
        //test run
        (11,7)
    } else {
        //real run
    (101,103)
    };
    // show width and height
    println!("width: {}, height: {}", -1 % width, height);
    // for each second, move the robots

    // move the robots
    let mut i = 0;
    let mut min_distance = std::f32::MAX;
    loop {
        for robot in robots.iter_mut() {
            robot.position.0 += 1 * robot.velocity.0;
            robot.position.1 += 1 * robot.velocity.1;
            while robot.position.0 < 0 {
                robot.position.0 += width;
            }
            while robot.position.1 < 0 {
                robot.position.1 += height;
            }
            robot.position.0 %= width;
            robot.position.1 %= height;
        }
        // print i
        // for each robot find the closest robot and calculate the distance, average it over all robots
        let mut avg_distance:f32 = 0.0;
        for robot in robots.iter() {
            let mut min_distance = std::i32::MAX;
            for other_robot in robots.iter() {
                if robot.position == other_robot.position {
                    continue;
                }
                let distance = (robot.position.0 - other_robot.position.0).abs()
                    + (robot.position.1 - other_robot.position.1).abs();
                if distance < min_distance {
                    min_distance = distance;
                }
            }
            avg_distance += min_distance as f32;
        }
        avg_distance /= robots.len() as f32;

        // check that no robot is on the same position
        if avg_distance < min_distance {
            min_distance = avg_distance;
            println!("i: {}", i+1 );

            // draw the robots
            for y in 0..height {
                for x in 0..width {
                    let mut found = false;
                    for robot in robots.iter_mut() {
                        if robot.position.0 == x && robot.position.1 == y {
                            print!("#");
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        print!(".");
                    }
                }
                println!();
            }
        }

        i += 1;
        if i > 10000 {
            break;
        }
    }
    Some(12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

}
