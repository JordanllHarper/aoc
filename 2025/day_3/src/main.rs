use std::{env, fs};

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
    let lines = input.lines();

    let mut battery_jolts: Vec<u64> = Vec::new();
    for line in lines {
        let mut highest_ten = 1;
        let mut highest_unit = 1;
        for (idx, num) in line
            .chars()
            .map(|n| n.to_digit(10).expect("Invalid number"))
            .enumerate()
        {
            if num > highest_ten && idx != line.len() - 1 {
                highest_ten = num;
                highest_unit = 1;
                continue;
            }
            if num > highest_unit {
                highest_unit = num;
            }
            // each num
        }
        let jolt = (highest_ten.to_string() + &highest_unit.to_string())
            .parse::<u64>()
            .expect("Invalid input");

        println!("{}", jolt);

        battery_jolts.push(jolt);
    }

    battery_jolts.iter().sum()
}

fn pt_2(input: String) -> u64 {
    // Notes:
    // We now need to find the largest 12 numbers in the bank
    // 987654321111111 -> 987654321111 - remove the end 1's
    // 811111111111119 -> 811111111119 - remove middle 1's
    // 234234234234278 -> 434234234278 -
    // we need to maximise our left most column to n - 12 - keep this index
    // Then we do the same from the prev index but instead of 12 it's 11
    // And same and same until we're done
    let mut battery_jolts: Vec<u64> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let mut total_joltage = String::new();
        let mut start_idx: usize = 0;
        // initially, give at least space for the last 12 numbers
        let mut end = line.len() - 12;
        while total_joltage.len() != 12 {
            let mut max_found = (line[start_idx..=start_idx]).parse::<u32>().unwrap();
            let mut max_found_idx = start_idx;
            for (idx, c) in line
                .chars()
                .map(|each| each.to_digit(10).expect("Invalid num"))
                .enumerate()
                // skip to our last num
                .skip(start_idx)
            {
                println!("idx: {}", idx);
                if idx > end {
                    break;
                }
                if c > max_found {
                    max_found = c;
                    max_found_idx = idx;
                }
            }
            // offset to start on the next num, not the max
            start_idx = max_found_idx + 1;
            // Compute the end idx - needs to be between the start index and remaining total
            // joltages
            // We need 12 - len remaining from the end of the line
            let required_end_space = (12 - total_joltage.len()) - 1;
            println!("Required end space: {}", required_end_space);
            end = line.len() - required_end_space;
            println!("End: {}", end);

            total_joltage += &max_found.to_string();
            println!("{}", total_joltage);
        }

        battery_jolts.push(total_joltage.parse::<u64>().expect("Invalid number"));
    }
    battery_jolts.iter().sum()
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
        assert_eq!(357, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        let test = pt_2(test_content);
        assert_eq!(3121910778619, test);
    }
}
