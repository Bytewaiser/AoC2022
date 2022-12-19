use std::fs;

#[derive(Debug)]
enum FileType {
    PlainFile(String, usize),
    Dir(String)
}


fn main() {
    let contents = fs::read_to_string("test.txt").unwrap();
    for line in contents.lines() {
        if line.starts_with("$") && line[2..].starts_with("cd") {
            let path = line[5..].to_owned();
            if path != ".." {
                let a = FileType::Dir(path);
                println!("{:?}", a);
            }
        }
    }
}
