use std::fs;

fn main() {
    let contents = fs::read_to_string("day05_input.txt")
        .expect("Something went wrong reading the file");

    let split_lines = contents.split("\n").collect::<Vec<&str>>();
    let mut max_id = 0;
    let mut ids: Vec<usize> = Vec::new();

    for pass in split_lines {
        let id = bin_search(pass, 128, 0, 7, 'B') * 8 + bin_search(pass, 8, 7, 10, 'R');
        ids.push(id);

        if id > max_id {
            max_id = id;
        }
    }

    println!("Part 1: {}", max_id);

    for i in 1..1024 {
        if ids.contains(&(i - 1)) && ids.contains(&(i + 1)) && !ids.contains(&i) {
            println!("Part 2: {}", i);
        }
    }
}

fn bin_search(pass: &str, limit: usize, start: usize, stop: usize, add_char: char) -> usize {
    let mut add = limit / 2;
    let mut result = 0;

    for i in start..stop {
        if pass.as_bytes()[i] as char == add_char {
            result += add;
        }

        add /= 2;
    }

    return result;
}