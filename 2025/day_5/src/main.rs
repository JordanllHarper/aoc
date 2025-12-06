use std::{collections::HashSet, env, fs, ops::RangeInclusive};
// incorrect answers:
// - 9604399248348  too low
// - 19398315025598 too low
// - 246971403589457 too low
// - 344378119285354

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
    if file.is_empty() {
        panic!("Input file is empty! Double check your input");
    }

    let answer = if run_part_1 { pt_1(file) } else { pt_2(file) };
    println!("Part {}: {}", if run_part_1 { "1" } else { "2" }, answer);
}

fn pt_1(input: String) -> u64 {
    let (fresh_ranges, available) = input.split_once("\r\n\r\n").expect("Invalid input");
    let raw_ranges = fresh_ranges
        .lines()
        .map(|each| {
            let (lower, upper) = each.split_once('-').expect("Invalid range input");
            (
                lower.parse::<u64>().expect("Invalid number in input"),
                upper.parse::<u64>().expect("Invalid number in input"),
            )
        })
        .collect::<Vec<(u64, u64)>>();
    let available = available
        .lines()
        .map(|each| each.parse::<u64>().expect("Invalid available id ranges"))
        .collect::<Vec<u64>>();

    let ranges: Vec<RangeInclusive<u64>> = raw_ranges.iter().map(|each| each.0..=each.1).collect();

    let mut contained = 0;
    'outer: for each in available {
        for each_range in &ranges {
            if each_range.contains(&each) {
                contained += 1;
                continue 'outer;
            }
        }
    }

    contained
}

fn pt_2(input: String) -> u64 {
    let (fresh_ranges, _) = input.split_once("\r\n\r\n").expect("Invalid input");
    let mut ranges = fresh_ranges
        .lines()
        .map(|each| {
            let (lower, upper) = each.split_once('-').expect("Invalid range input");
            (
                lower.parse::<u64>().expect("Invalid number in input"),
                upper.parse::<u64>().expect("Invalid number in input"),
            )
        })
        .enumerate()
        .collect::<Vec<_>>();
    // We need to figure out where our range boundaries lie so we don't include existing ranges
    //
    // A few cases:
    // [
    //      3 - 5
    //      10 - 14
    // ] => these are separate :)
    //
    // [
    //      16-20
    //      12-18
    // ] => 12-18 is included in 16-20
    //
    // [
    //      16-20
    //      18-24
    // ] => 16-20 partially includes 18-24
    //      here we want the ranges to change to
    //      16 - 17
    //      18 - 24
    //
    // We need to find the ranges that overlap.
    //
    // Perhaps we can get overlaps by looking for a range that is
    // greater than our lowerbound, and less than our upperbound
    //
    // how do we find overlapping ranges
    let mut had_changes = false;
    loop {
        let mut new_ranges: HashSet<(u64, u64)> = HashSet::new();
        for (current_idx, (current_lower, current_upper)) in &ranges {
            for (comparison_idx, (comparison_lower, comparison_upper)) in &ranges {
                // println!(
                //     "Comparing {}-{} to {}-{}",
                //     current_lower, current_upper, comparison_lower, comparison_upper
                // );

                if current_idx == comparison_idx {
                    new_ranges.insert((*current_lower, *current_upper));
                    continue;
                }

                if comparison_upper < current_lower || current_upper < comparison_lower {
                    new_ranges.insert((*current_lower, *current_upper));
                    continue;
                }

                if comparison_lower <= current_lower && current_upper <= comparison_upper {
                    // println!(
                    //     "{}-{} includes {}-{}",
                    //     comparison_lower, comparison_upper, current_lower, current_upper
                    // );
                    new_ranges.insert((*comparison_lower, *comparison_upper));
                    new_ranges.remove(&(*current_lower, *current_upper));
                    had_changes = true;
                    continue;
                }

                if comparison_lower <= current_lower && comparison_upper <= current_upper {
                    println!("New range: {}-{}", comparison_lower, current_upper);
                    let new_range = (*comparison_lower, *current_upper);
                    new_ranges.insert(new_range);
                    // remove the conflicts
                    new_ranges.remove(&(*comparison_lower, *comparison_upper));
                    new_ranges.remove(&(*current_lower, *current_upper));
                    had_changes = true;
                    continue;
                }

                if current_lower <= comparison_lower && current_upper <= comparison_upper {
                    println!("New range: {}-{}", current_lower, comparison_upper);
                    let new_range = (*current_lower, *comparison_upper);
                    new_ranges.insert(new_range);
                    // remove the conflicts
                    new_ranges.remove(&(*comparison_lower, *comparison_upper));
                    new_ranges.remove(&(*current_lower, *current_upper));
                    had_changes = true;
                    continue;
                }
            }
        }
        if !had_changes {
            break;
        } else {
            had_changes = false;
            println!("Ranges after iter: {:?}", new_ranges);
            ranges = new_ranges.into_iter().enumerate().collect();
        }
    }
    println!("Final ranges: {:?}", ranges);

    ranges
        .iter()
        .map(|(_, (lower, upper))| (upper - lower) + 1)
        .sum()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{pt_1, pt_2};

    const TEST: &str = "test.txt";

    #[test]
    fn t_pt1() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        if test_content.is_empty() {
            panic!("Test file is empty! Double check the contents.");
        }
        let test = pt_1(test_content);
        assert_eq!(3, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        if test_content.is_empty() {
            panic!("Test file is empty! Double check the contents.");
        }
        let test = pt_2(test_content);
        assert_eq!(14, test);
    }
}
