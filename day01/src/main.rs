fn main() {
    let mut numbers: Vec<usize> = Vec::new();

    std::fs::read_to_string("day01_input.txt")
        .expect("file not found!")
        .lines()
        .for_each(|x| numbers.push(x.parse::<usize>().unwrap()));

    let length = numbers.len();

    numbers.sort();

    let mut start = 0;
    let mut end = length - 1;

    let mut sum = numbers[start] + numbers[end];

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

    let mut found = false;
    let mut third = 0;

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
}
