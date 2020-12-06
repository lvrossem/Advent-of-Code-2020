use std::fs;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

fn main() {
    let contents = fs::read_to_string("day06_input.txt")
        .expect("Something went wrong reading the file");

    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    let mut result1 = 0;
    let mut result2 = 0;
    
    for group in groups {
        let lines: Vec<&str> = group.split('\n').collect();
        for a in ASCII_LOWER.iter() {
            if lines.iter().any(|x| x.contains(*a)) {
                result1 += 1;
            }

            if lines.iter().all(|x| x.contains(*a)) {
                result2 += 1;
            }
        }
    }
    

    println!("Part 1 {}", result1);
    println!("Part 2 {}", result2);
}

