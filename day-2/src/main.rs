use std::{fs, collections::HashMap};


fn part1(input: &str) -> i32 {

    // A -> Rock, B -> Paper, C -> Scissors
    // X -> Rock, Y -> Paper, Z -> Scissors

    let status = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    input.lines().map(|item| status.get(item).unwrap()).sum()
}

fn part2(input: &str) -> i32 {
    // A -> Rock, B -> Paper, C -> Scissors
    // X -> Lose, Y -> Draw, Z -> Win
    let status = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    input.lines().map(|item| status.get(item).unwrap()).sum()
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let value1 = part1(&contents);
    println!("{}", value1);
    let value2 = part2(&contents);
    println!("{}", value2);
}
