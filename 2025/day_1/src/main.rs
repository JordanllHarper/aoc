use std::{env, fs};
fn parse_is_subtract_amount(line: &str) -> (bool, i32) {
    let (direction, amount) = line.split_at(1);
    let is_subtract = match direction {
        "L" => true,
        "R" => false,
        _ => unreachable!(),
    };
    let amount = amount.parse::<i32>().expect("Bad input");
    (is_subtract, amount)
}

fn pt_1(input: String) -> i32 {
    let mut zero_count = 0;
    let mut current = 50;
    for line in input.lines() {
        let (is_subtract, amount) = parse_is_subtract_amount(line);
        let mut new = if is_subtract {
            current - amount
        } else {
            current + amount
        };

        // Problem: value can be added or subtracted by values 200 and greater
        // e.g. R500 or L348 -> -270 + 100 = 170 which will just continue with 170 as the value
        // (which is bad)
        //
        while new < 0 {
            new += 100
        }

        while new > 99 {
            new -= 100;
        }

        if new == 0 {
            zero_count += 1;
        }
        current = new;
    }
    zero_count
}

// paths:
// start on zero -> L0/R0 - end on zero - inc zero = no
// start on zero -> L0/R100 - end on zero - inc zero = yes by 1
// start on zero -> L0/R200 - end on zero - inc zero = yes by 2
//
// start on zero -> L100/R0 - end on zero - inc zero = yes by 1
// start on zero -> L200/R0 - end on zero - inc zero = yes by 2
//
// start on one -> L101 - end on zero - inc zero = yes by 2 (one for skip, one for land)
// start on one -> R199 - end on zero - inc zero = yes by 2 (one for skip, one for land)
//

fn pt_2(input: String) -> i32 {
    let mut zero_count = 0;
    let mut dial_pos = 50;

    for line in input.lines() {
        let (is_subtract, mut amount) = parse_is_subtract_amount(line);
        let was_on_zero = dial_pos == 0;

        while amount >= 100 {
            amount -= 100;
            zero_count += 1;
        }

        dial_pos = if is_subtract {
            dial_pos - amount
        } else {
            dial_pos + amount
        };

        if dial_pos == 100 {
            dial_pos = 0
        }

        if dial_pos == 0 {
            // we have ticked onto 0
            zero_count += 1;
            continue;
        }

        if dial_pos < 0 {
            dial_pos += 100;
            if !was_on_zero {
                zero_count += 1;
            }
        }

        if dial_pos > 99 {
            dial_pos -= 100;
            if !was_on_zero {
                zero_count += 1;
            }
        }
    }
    zero_count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let run_part_1 = match args.get(1).as_ref().map(|e| e.as_str()) {
        Some("1") => true,
        Some("2") => false,
        Some(v) => panic!("Invalid part argument {}", v),
        None => panic!("Missing part argument"),
    };

    let input = "input.txt";
    let file = fs::read_to_string(input).expect("File doesn't exist");

    let answer = if run_part_1 { pt_1(file) } else { pt_2(file) };
    println!("Part {}: {}", if run_part_1 { "1" } else { "2" }, answer);
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{pt_1, pt_2};

    const TEST: &str = "test.txt";

    #[test]
    fn t_pt1() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        let test = pt_1(test_content);
        assert_eq!(3, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        let test = pt_2(test_content);
        assert_eq!(6, test);
    }
}
