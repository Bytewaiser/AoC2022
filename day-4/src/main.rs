use std::fs;

fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let halves = line.split(",").collect::<Vec<&str>>();
        let first_numbers = halves[0].split("-").collect::<Vec<&str>>();
        let last_numbers = halves[1].split("-").collect::<Vec<&str>>();
        let s1 = first_numbers[0].parse::<i32>().unwrap();
        let e1 = first_numbers[1].parse::<i32>().unwrap();

        let s2 = last_numbers[0].parse::<i32>().unwrap();
        let e2 = last_numbers[1].parse::<i32>().unwrap();

        // 2-8, 3-7
        // 5-6, 4-6

        if (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2) {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let halves = line.split(",").collect::<Vec<&str>>();
        let first_numbers = halves[0].split("-").collect::<Vec<&str>>();
        let last_numbers = halves[1].split("-").collect::<Vec<&str>>();
        let s1 = first_numbers[0].parse::<i32>().unwrap();
        let e1 = first_numbers[1].parse::<i32>().unwrap();

        let s2 = last_numbers[0].parse::<i32>().unwrap();
        let e2 = last_numbers[1].parse::<i32>().unwrap();

        // 2-8, 3-7
        // 5-6, 4-6
        //
        if (s1 >= s2 && s1 <= e2) || (e1 >= s2 && e1 <= e2) || (s2 >= s1 && s2 <= e1) || (e2 >= s1 && e2 <= e1) {
            count += 1;
        }
    }
    count
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let value1 = part1(&contents);
    println!("{}", value1);
    let value2 = part2(&contents);
    println!("{}", value2);
}
