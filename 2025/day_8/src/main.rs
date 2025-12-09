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

#[derive(Debug, Clone, PartialEq, Hash)]
struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
}

fn pt_1(input: String) -> i32 {
    let boxes = input
        .lines()
        .map(|each| {
            let mut coords = each.split(',');
            let x = coords
                .next()
                .expect("Invalid x")
                .parse()
                .expect("Invalid coord");
            let y = coords
                .next()
                .expect("Invalid y")
                .parse()
                .expect("Invalid coord");
            let z = coords
                .next()
                .expect("Invalid z")
                .parse()
                .expect("Invalid coord");

            JunctionBox { x, y, z }
        })
        .collect::<Vec<JunctionBox>>();

    println!("{:?}", boxes);

    todo!()
}

fn pt_2(input: String) -> i32 {
    todo!()
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
    // #[test]
    // fn t_pt2() {
    //     let test_content = fs::read_to_string(TEST).expect("Missing test text file");
    //     if test_content.is_empty() {
    //         panic!("Test file is empty! Double check the contents.");
    //     }
    //     let test = pt_2(test_content);
    //     assert_eq!(6, test);
    // }
}
