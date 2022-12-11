use std::fs::read_to_string;

fn get_crate_vectors(crates: &str) -> Vec<Vec<&str>> {
    let mut vectors: Vec<Vec<&str>> = Vec::new();
    let crate_numbers: Vec<usize> = crates
        .lines()
        .last()
        .unwrap()
        .trim()
        .split("   ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for _ in crate_numbers.iter() { 
        vectors.push(Vec::new());
    }

    let crate_content: &str = crates.split_once("\n ").unwrap().0;
    for i in crate_numbers.iter() {
        for j in crate_content.lines() {
            let k = j.get((i * 4 - 3)..(i * 4 - 2)).unwrap();
            if k != " " {
                vectors[*i-1].insert(0, k);
            }
        }
    }

    vectors
}

fn get_moves(moves: &str) -> Vec<Vec<usize>> {
    let moves: Vec<Vec<usize>> = moves.lines().map(|x| {
        x.split(" ")
            .map(|x| x.parse::<usize>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect()
    }).collect();
    
    moves
}

fn part1(crates: &mut Vec<Vec<&str>>, moves: &Vec<Vec<usize>>) {
    for item in moves.iter() {
        let count = item[0];
        let from = item[1]-1;
        let to = item[2]-1;
        for _ in 0..count {
            let val = crates[from].pop().unwrap();
            crates[to].push(val)
        }
    }

    println!("{:?}", crates);

    for i in crates {
        print!("{}", i.last().unwrap());
    }
    print!("\n\n");
}

fn part2(crates: &mut Vec<Vec<&str>>, moves: &Vec<Vec<usize>>) {
    for item in moves.iter() {
        let count = item[0];
        let from = item[1]-1;
        let to = item[2]-1;

        let lhs = &crates[from].clone()[..crates[from].len()-count];
        let rhs = &crates[from].clone()[crates[from].len()-count..];

        crates[from] = lhs.to_vec();
        crates[to].append(&mut rhs.to_vec());
    }

    println!("{:?}", crates);

    for i in crates {
        print!("{}", i.last().unwrap());
    }
    print!("\n");
}

fn main() {
    let contents = read_to_string("data.txt").unwrap();
    let (crates, moves) = contents.split_once("\nmove").unwrap();
    let moves = "move ".to_owned() + moves.trim();
    
    let crates = get_crate_vectors(crates);
    let moves = get_moves(&moves);

    let mut crates1 = crates.clone();
    let mut crates2 = crates.clone();

    part1(&mut crates2, &moves);
    part2(&mut crates1, &moves);

    
}
