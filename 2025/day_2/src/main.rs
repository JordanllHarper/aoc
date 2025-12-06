use std::{env, fs, ops::ControlFlow};

// notes:
// Variable length repetitions
// So we have a range 11 - 22
// 11 + 22 are repeated digit numbers
//
//
//Once we find, *add them all up*
//
//
// If our number is odd characters long, it's invalid (must be even)
//
//We can figure out if the characters repeat by converting our number to a string and then splitting in the middle. Then compare the 2 slices

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

fn pt_1(input: String) -> u64 {
    let ranges: Vec<(u64, u64)> = input
        .split(',')
        .map(|each| each.split('-'))
        .map(|mut each_split| {
            let first = each_split.next().expect("Invalid input").trim();
            let last = each_split.next_back().expect("Invalid input").trim();
            (first, last)
        })
        .map(|(first, last)| {
            println!("First: {} Last: {}", first, last);
            let first_num = first.parse::<u64>().expect("Invalid number");
            let last_num = last.parse::<u64>().expect("Invalid number");
            (first_num, last_num)
        })
        .collect();
    let mut duplicated_ids: Vec<u64> = Vec::new();
    for (first, last) in ranges {
        for num in first..=last {
            // iterate our ranges
            let s_num = num.to_string();
            // if it's odd, guaranteed no symmetry
            if s_num.len() % 2 != 0 {
                continue;
            }
            // split at midpoint and compare
            let (left, right) = s_num.split_at(s_num.len() / 2);
            if left == right {
                duplicated_ids.push(num);
            }
        }
    }

    duplicated_ids.iter().sum()
}

fn pt_2(input: String) -> u64 {
    let ranges: Vec<(u64, u64)> = input
        .split(',')
        .map(|each| each.split('-'))
        .map(|mut each_split| {
            let first = each_split.next().expect("Invalid input").trim();
            let last = each_split.next_back().expect("Invalid input").trim();
            (first, last)
        })
        .map(|(first, last)| {
            println!("First: {} Last: {}", first, last);
            let first_num = first.parse::<u64>().expect("Invalid number");
            let last_num = last.parse::<u64>().expect("Invalid number");
            (first_num, last_num)
        })
        .collect();
    let mut duplicated_ids: Vec<u64> = Vec::new();
    for (first, last) in ranges {
        for num in first..=last {
            let s_num = num.to_string();
            // 1|1
            // 1
            // 1 - have we seen this before? If so, it might be a duplicate
            // 123|123|123
            // 1
            // 2
            // 3
            // 1 - is this a duplicate? Either assume it is or isn't?
            // 2 -
            // 123|124
            // 12321123211
            // ... 3
            // 2
            // 1 - we have seen this before, is this now a pattern?
            // 1 - last was not a pattern, but is this a pattern?
            //
            // what is the point that we end our search and start looking for the pattern?
            // Answer: maybe the halfway point? Cut the string in 2, compare 2 strings together.
            // If match, we've got it! If no match, remove one character from left split and try
            // again.
            //
            // Problem with that approach, it needs to be a number of ONLY repeating pattern.
            //
            // i.e. 824824823 is NOT a repeating pattern but in the impl below, it is considered valid.
            //
            //
            //
            //What do we need to know:
            //- The pattern
            //- If it's consistent throughout the entire input
            //
            // Stepping through
            //
            // 82482 | 4823
            // 8248  | 4823
            // 824   | 4823
            // 82    | 4823
            // 8     | 4823
            //       | 4823 - done
            //
            // What about if we give characters back to right over time?
            //
            // 82482 | 4823
            // 8248  | 24823
            // 824   | 824823
            // 82    | 4824823
            // 8     | 24824823
            //       | 824824823 - done
            //
            //
            //
            // Edge case 2121|212121
            // 2121
            //
            //
            // Wrong answers (so we don't repeat):
            // 30260171261 - false positives

            let (mut left, right) = s_num.split_at((s_num.len() - 1).div_ceil(2));
            let mut right = right.to_string();
            while !right.split(left).all(|e| e.is_empty()) {
                if left.is_empty() {
                    break;
                }
                let (split_l, split_r) = left.split_at(left.len() - 1);
                left = split_l;
                right = split_r.to_string() + &right;
            }
            if !left.is_empty() {
                duplicated_ids.push(num);
            }
        }
    }

    duplicated_ids.iter().sum()
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
        assert_eq!(1227775554, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        let test = pt_2(test_content);
        assert_eq!(4174379265, test);
    }
}
