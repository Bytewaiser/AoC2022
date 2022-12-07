use std::fs;

fn part1(input: &str) -> u32 {
    let mut sum_points = 0;
    for line in input.lines() {
        let half: usize = line.len() / 2;
        let first_half = &line[..half];
        let second_half = &line[half..];
        for ch in first_half.chars() {
            if second_half.find(ch) != None {
                if ch.is_lowercase() {
                    sum_points += ch as u32 - 96;
                } else {
                    sum_points += ch as u32 - 38;
                }
                break;
            }
        }
    }

    sum_points
}

// a, b, c
// b, c, d
// c, d, e
// d, e, f

fn part2(input: &str) -> u32 {
    let mut sum_points = 0;
    let three_lines = input
        .lines()
        .zip(input.lines().skip(1))
        .zip(input.lines().skip(2));

    for (idx, ((x, y), z)) in three_lines.enumerate() {
        if (idx % 3) != 0 {
            continue;
        }
        for ch in x.chars() {
            if y.find(ch) != None && z.find(ch) != None {
                if ch.is_lowercase() {
                    sum_points += ch as u32 - 96;
                } else {
                    sum_points += ch as u32 - 38;
                }
                break;
            }
        }
    }

    sum_points
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let value1 = part1(&contents);
    println!("{}", value1);
    let value2 = part2(&contents);
    println!("{}", value2);
}
