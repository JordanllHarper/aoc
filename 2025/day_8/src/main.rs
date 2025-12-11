use std::{collections::HashMap, env, fs};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    let mut straight_lines: HashMap<(&JunctionBox, &JunctionBox), u32> = HashMap::new();
    for source in &boxes {
        for target in &boxes {
            if source == target {
                continue;
            }
            let straight_line = source.x.abs_diff(target.x).pow(2)
                + source.y.abs_diff(target.y).pow(2)
                + source.z.abs_diff(target.z).pow(2);

            if straight_lines
                .keys()
                .filter(|e| e.0 == target && e.1 == source)
                .count()
                == 0
            {
                straight_lines.insert((source, target), straight_line);
            }
        }
    }

    let mut kvps = straight_lines
        .iter()
        .map(|eachv| (eachv.0, eachv.1))
        .collect::<Vec<(&(&JunctionBox, &JunctionBox), &u32)>>();
    kvps.sort_by(|a, b| a.1.cmp(b.1));
    let mut iter = kvps.iter();
    let first = iter.next();
    let second = iter.next();
    println!("{:?}", first);
    println!("{:?}", second);

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
