use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day01_input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let parsed_int = line.unwrap().parse::<u32>().unwrap();
        numbers.push(parsed_int);
    }

    let length = numbers.len();

    numbers.sort();

    let mut start:usize = 0;
    let mut end:usize = length - 1;

    let mut sum:u32 = numbers[start] + numbers[end];

    while sum != 2020 {
        if sum > 2020 {
            end -= 1;
        } else {
            start += 1;
        }

        sum = numbers[start] + numbers[end];
    }

    println!("The result for 2 numbers is {}", numbers[start] * numbers[end]);

    start = 0;
    end = length - 1;

    let mut found:bool = false;
    let mut third:usize = 0;

    sum = numbers[start] + numbers[end] + numbers[third];

    while !found && third < length {
        while sum != 2020 && end >= start {
            if sum > 2020 {
                end -= 1;
            } else {
                start += 1;
            }

            sum = numbers[start] + numbers[end] + numbers[third];
        }

        if sum == 2020 {
            found = true;
            println!("The result for 3 numbers is {}", numbers[start] * numbers[end] * numbers[third])
        } else {
            third += 1;
            start = 0;
            end = length - 1;
        }
    }

    Ok(())
}
