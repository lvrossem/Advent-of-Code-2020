struct Line {
    min: usize,
    max: usize,
    character: u8,
    password: String
}

impl Line {
    fn parse(line: &str) -> Line {
        let split_line = line.split(' ').collect::<Vec<&str>>();
        let amount = split_line[0].split('-').collect::<Vec<&str>>();

        return Line {
            min: amount[0].parse::<usize>().unwrap(),
            max: amount[1].parse::<usize>().unwrap(),
            character: split_line[1].as_bytes()[0],
            password: split_line[2].to_string()
        }
    }

    fn check1(&self) -> bool {
        let occurences = self.password.matches(self.character as char).count();
        return occurences >= self.min && occurences <= self.max;
    }

    fn check2(&self) -> bool {
        return (self.password.as_bytes()[self.min - 1] == self.character) != (self.password.as_bytes()[self.max - 1] == self.character);
    }
}

fn main() {
    let mut amount = std::fs::read_to_string("day02_input.txt")
        .expect("file not found!")
        .lines()
        .map(|x| Line::parse(x).check1())
        .filter(|x| *x)
        .count();

    println!("There are {} valid passwords for part 1", amount);

    amount = std::fs::read_to_string("day02_input.txt")
        .expect("file not found!")
        .lines()
        .map(|x| Line::parse(x).check2())
        .filter(|x| *x)
        .count();

    println!("There are {} valid passwords for part 2", amount);
}