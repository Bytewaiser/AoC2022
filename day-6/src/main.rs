use std::{fs, collections::HashSet};

fn part1(input: &str, num_after: usize) -> usize {
    let mut num = 0;
    for i in 0..input.len()-num_after {
        let elem = input.get(i..i+num_after).unwrap();
        let mut set: HashSet<char> = HashSet::new();
        for ch in elem.chars() {
            set.insert(ch);
        }
        if set.len() == num_after {
            num = i+num_after;
            break;
        }
    }
    num
}


fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let value1 = part1(&contents, 4);
    println!("{}", value1);
    let value2 = part1(&contents, 14);
    println!("{}", value2);
}
