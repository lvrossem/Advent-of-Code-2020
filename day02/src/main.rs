struct Line<'a> {
    min: usize,
    max: usize,
    character: u8,
    password: &'a str
}

fn main() {
    let mut amount = std::fs::read_to_string("day02_input.txt")
        .expect("file not found!")
        .lines()
        .map(check_part1)
        .filter(|x| *x)
        .count();

    println!("There are {} valid passwords for part 1", amount);

    amount = std::fs::read_to_string("day02_input.txt")
        .expect("file not found!")
        .lines()
        .map(check_part2)
        .filter(|x| *x)
        .count();

    println!("There are {} valid passwords for part 2", amount);
}

fn parse(line: &str) -> Line {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    let amount = split_line[0].split('-').collect::<Vec<&str>>();

    return Line {
        min: amount[0].parse::<usize>().unwrap(),
        max: amount[1].parse::<usize>().unwrap(),
        character: split_line[1].as_bytes()[0],
        password: split_line[2]
    }
}

fn check_part1(line: &str) -> bool {
    let parsed = parse(line);
    let occurences = parsed.password.matches(parsed.character as char).count();

    return occurences >= parsed.min && occurences <= parsed.max;
}

fn check_part2(line: &str) -> bool {
    let parsed = parse(line);
    
    return (parsed.password.as_bytes()[parsed.min - 1] == parsed.character) != (parsed.password.as_bytes()[parsed.max - 1] == parsed.character);
}
