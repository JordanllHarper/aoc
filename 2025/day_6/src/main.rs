use core::panic;
use std::{collections::HashMap, env, fs, ops::RangeInclusive};

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
    // we have a vector of vectors and then the final operation
    let mut lines = input.lines();
    let operations: Vec<char> = lines
        .next_back()
        .expect("Invalid input")
        .chars()
        .filter(|each| !each.is_whitespace())
        .collect();

    let number_rows: Vec<Vec<u64>> = lines
        .map(|each| {
            each.split_whitespace()
                .map(|each| each.parse::<u64>().expect("Invalid input"))
                .collect::<Vec<u64>>()
        })
        .collect();
    let mut totals: HashMap<usize, u64> = HashMap::new();

    for row in number_rows.iter() {
        for (col_idx, num) in row.iter().enumerate() {
            let op = operations.get(col_idx).expect("Invalid col count");

            let current = totals.get(&col_idx);
            if let Some(v) = current {
                let new_val = match op {
                    '+' => v + num,
                    '*' => v * num,
                    _ => panic!("Invalid symbol"),
                };
                totals.insert(col_idx, new_val);
            } else {
                totals.insert(col_idx, *num);
            }
        }
    }

    totals.values().sum()
}

// wrong answers:
//
// - 31979180435256 too high
// - 11744693116217 too low - inclusive definitely needed
// - 11744693538946
fn pt_2(input: String) -> u64 {
    let mut rows = input.lines().map(|each| each.chars().collect());

    let operators: Vec<char> = rows.next_back().expect("Invalid input");
    // let op_idx_map: HashMap<RangeInclusive<usize>, char> = operators
    //     .iter()
    //     .enumerate()
    //     // problem, the widths are not fixed to 4 chars - we need a parsing solution for variable
    //     // width operator spacing
    //     .map(|(op_idx, op)| (op_idx..=(op_idx + 3), *op))
    //     .filter(|(_, c)| !c.is_whitespace())
    //     .collect();

    let mut op_idx_map: HashMap<RangeInclusive<usize>, char> = HashMap::new();
    let mut current_op = operators.first().expect("Invalid op input");
    let mut current_op_idx = 0;
    let op_iter = operators.iter().enumerate().skip(1);
    for (idx, c) in op_iter {
        if c.is_whitespace() {
            continue;
        }

        op_idx_map.insert(current_op_idx..=(idx - 1), *current_op);
        current_op = c;
        current_op_idx = idx;
    }

    op_idx_map.insert(current_op_idx..=(current_op_idx + 3), *current_op);

    println!("{:?}", op_idx_map);

    let mut idx_to_col_num: HashMap<usize, Vec<char>> = HashMap::new();

    for r in rows {
        for (ch_idx, c) in r.iter().enumerate() {
            // ignore all whitespace, it's used soley for maintaining indexes
            if c.is_whitespace() {
                continue;
            }
            let current = idx_to_col_num.get_mut(&ch_idx);
            if let Some(v) = current {
                v.push(*c);
            } else {
                idx_to_col_num.insert(ch_idx, vec![*c]);
            }
        }
    }
    let nums: Vec<(usize, u64)> = idx_to_col_num
        .iter()
        .filter_map(|(idx, chars)| {
            let binding = chars
                .iter()
                .fold(String::new(), |acc, c| acc + &c.to_string());
            let r = binding.trim();

            assert!(r.len() <= 4);

            r.parse::<u64>().ok().map(|v| (*idx, v))
        })
        .collect();

    let mut totals: HashMap<RangeInclusive<usize>, u64> = HashMap::new();
    for (idx, n) in nums {
        for (range, op) in &op_idx_map {
            if !range.contains(&idx) {
                continue;
            }
            let current_total = totals.get(range);
            if let Some(pat) = current_total {
                let res = match op {
                    '+' => pat + n,
                    '*' => pat * n,
                    _ => panic!("Bad symbol!!!"),
                };
                totals.insert(range.clone(), res);
            } else {
                totals.insert(range.clone(), n);
            }
        }
    }

    totals.values().sum()
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
        assert_eq!(4277556, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        if test_content.is_empty() {
            panic!("Test file is empty! Double check the contents.");
        }
        let test = pt_2(test_content);
        assert_eq!(3263827, test);
    }
}
