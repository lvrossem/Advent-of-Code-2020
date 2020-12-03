fn check_tree(line: &str, x_slope: usize, y_slope: usize, line_nr: &mut usize) -> bool {
    if *line_nr > 0 {
        if (*line_nr) % y_slope == 0 {
            
            let result = line.as_bytes()[(x_slope * (*line_nr / y_slope)) % line.as_bytes().len()] as char == '#';
            *line_nr += 1;

            return result;
        }
    }

    *line_nr += 1;
    return false;
}

fn check_lines(x_slope: usize, y_slope: usize) -> usize {
    let mut line_nr = 0;

    return std::fs::read_to_string("day03_input.txt")
        .expect("file not found!")
        .lines()
        .map(|line| check_tree(line, x_slope, y_slope, &mut line_nr))
        .filter(|x| *x)
        .count();
}

fn main() {  
    println!("Part 1 has {} hits", check_lines(3, 1));
    println!("Part 2 has {} hits", check_lines(1, 1) * check_lines(3, 1) * check_lines(5, 1) * check_lines(7, 1) * check_lines(1, 2))
}