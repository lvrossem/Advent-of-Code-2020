use std::io::{self};

fn main() -> io::Result<()> {
    let mut amount:usize = std::fs::read_to_string("day02_input.txt")
        .expect("file not found!")
        .lines()
        .map(|x| parse_part1(x.to_string()))
        .filter(|x| *x)
        .count();

    println!("There are {} valid passwords for part 1", amount);

    amount = std::fs::read_to_string("day02_input.txt")
        .expect("file not found!")
        .lines()
        .map(|x| parse_part2(x.to_string()))
        .filter(|x| *x)
        .count();

    println!("There are {} valid passwords for part 2", amount);
    Ok(())
}

fn parse_part1(line:String) -> bool {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    let amount = split_line[0].split('-').collect::<Vec<&str>>();

    let min:u16 = amount[0].parse::<u16>().unwrap();
    let max:u16 = amount[1].parse::<u16>().unwrap();

    let character:char = split_line[1].as_bytes()[0] as char;

    let occurences:u16 = split_line[2].matches(character).count() as u16;

    return occurences >= min && occurences <= max;
}

fn parse_part2(line:String) -> bool {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    let amount = split_line[0].split('-').collect::<Vec<&str>>();

    let min:usize = amount[0].parse::<usize>().unwrap() as usize;
    let max:usize = amount[1].parse::<usize>().unwrap() as usize;

    let character:u8 = split_line[1].as_bytes()[0];
    
    return (split_line[2].as_bytes()[min - 1] == character) ^ (split_line[2].as_bytes()[max - 1] == character);
}
