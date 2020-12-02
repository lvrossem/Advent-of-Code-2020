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

fn parse(line: &str) -> (usize, usize, u8, &str) {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    let amount = split_line[0].split('-').collect::<Vec<&str>>();

    let min = amount[0].parse::<usize>().unwrap();
    let max = amount[1].parse::<usize>().unwrap();

    let character = split_line[1].as_bytes()[0];
    
    return (min, max, character, split_line[2]);
}

fn check_part1(line: &str) -> bool {
    let (min, max, character, password) = parse(line);
    let occurences = password.matches(character as char).count();

    return occurences >= min && occurences <= max;
}

fn check_part2(line: &str) -> bool {
    let (min, max, character, password) = parse(line);
    
    return (password.as_bytes()[min - 1] == character) != (password.as_bytes()[max - 1] == character);
}
