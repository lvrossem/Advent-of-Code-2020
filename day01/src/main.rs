use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day01_input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let parsed_int = line.unwrap().parse::<i32>().unwrap();
        numbers.push(parsed_int);
    }

    numbers.sort();

    let mut start = 0;
    let mut end = numbers.len() - 1;

    while numbers[start] + numbers[end] != 2020 {
        if numbers[start] + numbers[end] > 2020 {
            end -= 1;
        } else {
            start += 1;
        }
    }

    println!("The result for 2 numbers is {}", numbers[start] * numbers[end]);

    start = 0;
    end = numbers.len() - 1;

    let mut found = false;
    let mut third = 0;

    while !found && third < numbers.len() {
        while numbers[start] + numbers[end] + numbers[third] != 2020 && end >= start {
            if numbers[start] + numbers[end] + numbers[third] > 2020 {
                end -= 1;
            } else {
                start += 1;
            }
        }

        if numbers[start] + numbers[end] + numbers[third] == 2020 {
            found = true;
            println!("The result for 3 numbers is {}", numbers[start] * numbers[end] * numbers[third])
        } else {
            third += 1;
            start = 0;
            end = numbers.len() - 1;
        }
    }
    

    Ok(())
}
