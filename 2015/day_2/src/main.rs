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
    if file.is_empty() {
        panic!("Input file is empty! Double check your input");
    }

    let answer = if run_part_1 { pt_1(file) } else { pt_2(file) };
    println!("Part {}: {}", if run_part_1 { "1" } else { "2" }, answer);
}

struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}

fn pt_1(input: String) -> i32 {
    input
        .lines()
        .map(|each| {
            let mut values = each.split('x');
            let length = values.next().unwrap().parse::<i32>().unwrap();
            let width = values.next().unwrap().parse::<i32>().unwrap();
            let height = values.next().unwrap().parse::<i32>().unwrap();

            (length, width, height)
        })
        .map(|(l, w, h)| {
            let mut sides = [l * w, w * h, h * l];
            sides.sort();
            let smallest = sides[0];
            let surface_area = sides.iter().map(|each| each * 2).sum::<i32>();
            surface_area + smallest
        })
        .sum()
}

fn pt_2(input: String) -> i32 {
    input
        .lines()
        .map(|each| {
            let mut values = each.split('x');
            let length = values.next().unwrap().parse::<i32>().unwrap();
            let width = values.next().unwrap().parse::<i32>().unwrap();
            let height = values.next().unwrap().parse::<i32>().unwrap();

            (length, width, height)
        })
        .map(|(l, w, h)| {
            let ribbon = l * w * h;
            println!("Ribbon: {}", ribbon);
            let mut sides = [l, w, h];
            sides.sort();
            let mut iter = sides.iter();

            let smallest = iter.next().unwrap() * 2;
            let next_smallest = iter.next().unwrap() * 2;
            println!("Smallest: {}", smallest);
            println!("Next smallest: {}", next_smallest);
            smallest + next_smallest + ribbon
        })
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
        assert_eq!(101, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        if test_content.is_empty() {
            panic!("Test file is empty! Double check the contents.");
        }
        let test = pt_2(test_content);
        assert_eq!(48, test);
    }
}
