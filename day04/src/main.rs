use std::fs;

fn main() {
    let contents = fs::read_to_string("day04_input.txt")
        .expect("Something went wrong reading the file");

    let split_lines = contents.split("\n\n").collect::<Vec<&str>>();
    
    let mut amount1 = 0;
    let mut amount2 = 0;

    for passport in split_lines {
        if check1(passport) {
            amount1 += 1;
        }

        if check2(passport) {
            amount2 += 1;
        }
    }

    println!("Part 1: {}", amount1);
    println!("Part 2: {}", amount2);
}

fn check1(passport: &str) -> bool {
    return passport.contains("byr") && passport.contains("iyr")
        && passport.contains("eyr") && passport.contains("hgt")
        && passport.contains("hcl") && passport.contains("ecl")
        && passport.contains("pid")
}

fn check2(passport: &str) -> bool {
    for part in passport.split(|x| (x == ' ') || (x == '\n')) {

        if part.contains("byr") && !check_byr(part.to_string()) {
            return false;
        }

        if part.contains("iyr") && !check_iyr(part.to_string()) {
            return false;
        }

        if part.contains("eyr") && !check_eyr(part.to_string()) {
            return false;
        }

        if part.contains("hgt") && !check_hgt(part.to_string()) {
            return false;
        }

        if part.contains("hcl") && !check_hcl(part.to_string()) {
            return false;
        }

        if part.contains("ecl") && !check_ecl(part.to_string()) {
            return false;
        }

        if part.contains("pid") && !check_pid(part.to_string()) {
            return false;
        }
    }

    return check1(passport);
}

fn parse_usize(text: &str, start: usize, stop: usize) -> usize {
    return text[start..stop].parse::<usize>().unwrap();
}

fn check_byr(part: String) -> bool {
    if part.len() == 8 {
        let year_parsed = parse_usize(&part, 4, 8);

        return year_parsed >= 1920 && year_parsed <= 2002
    } else {
        return false;
    }
}

fn check_iyr(part: String) -> bool {
    if part.len() == 8 {
        let year_parsed = parse_usize(&part, 4, 8);
        return year_parsed >= 2010 && year_parsed <= 2020
    } else {
        return false;
    }
}

fn check_eyr(part: String) -> bool {
    if part.len() == 8 {
        let year_parsed = parse_usize(&part, 4, 8);
        return year_parsed >= 2020 && year_parsed <= 2030;
    } else {
        return false;
    }
}

fn check_hgt(part: String) -> bool {
    if part.len() == 9 {
        if !part[4..7].chars().all(char::is_numeric) {
            return false;
        }

        let hgt_parsed = parse_usize(&part, 4, 7);
        return hgt_parsed >= 150 && hgt_parsed <= 193 && &part[7..9] == "cm"
    } else if part.len() == 8 {
        if !part[4..6].chars().all(char::is_numeric) {
            return false;
        }

        let hgt_parsed = parse_usize(&part, 4, 6);
        return hgt_parsed >= 59 && hgt_parsed <= 76 && &part[6..8] == "in";
    } else {
        return false;
    }
}

fn check_hcl(part: String) -> bool {
    if part.len() == 11 {
        if part.as_bytes()[4] as char == '#' {
            if !part[5..11].chars().all(|x| x.is_ascii_hexdigit()) {
                return false;
            }
        } else {
            return false
        }
    } else {
        return false;
    }
    return true;
}

fn check_ecl(part: String) -> bool {
    if part.len() == 7 {
        let color_part = &part[4..7];
        return color_part == "amb"
            || color_part == "blu"
            || color_part == "brn"
            || color_part == "gry"
            || color_part == "grn"
            || color_part == "hzl"
            || color_part == "oth"
    } else {
        return false
    }
}

fn check_pid(part: String) -> bool {
    if part.len() == 13 {
        if !part[4..13].chars().all(char::is_numeric) {
            return false;
        }
    } else {
        return false;
    }
    return true;
}