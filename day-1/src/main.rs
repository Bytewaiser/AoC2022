use std::fs;

fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let mut result = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        }).collect::<Vec<i32>>();

    result.sort_by(|a, b| b.cmp(a));
    result.iter().take(3).sum()
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let value1 = part1(&contents);
    println!("{}", value1);
    let value2 = part2(&contents);
    println!("{}", value2);
}
