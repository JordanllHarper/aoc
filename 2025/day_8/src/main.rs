use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

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
    x: i64,
    y: i64,
    z: i64,
}

fn pt_1(input: String) -> i64 {
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

    let mut straight_lines: HashMap<(&JunctionBox, &JunctionBox), u64> = HashMap::new();
    for source in &boxes {
        for target in &boxes {
            if source == target || straight_lines.contains_key(&(target, source)) {
                continue;
            }
            let straight_line = source.x.abs_diff(target.x).pow(2)
                + source.y.abs_diff(target.y).pow(2)
                + source.z.abs_diff(target.z).pow(2);

            straight_lines.insert((source, target), straight_line);
        }
    }

    let mut junction_to_straight_line = straight_lines
        .iter()
        .map(|eachv| (eachv.0, eachv.1))
        .collect::<Vec<(&(&JunctionBox, &JunctionBox), &u64)>>();
    junction_to_straight_line.sort_by(|a, b| a.1.cmp(b.1));

    let mut iter = junction_to_straight_line.iter();
    let first = iter.next().expect("Invalid input");
    let mut circuits: Vec<HashSet<&JunctionBox>> = vec![HashSet::from([first.0.0, first.0.1])];
    println!("{:?}", circuits);
    'outer: for ((j1, j2), _) in iter {
        println!("Comparing {:?} to {:?}", j1, j2);
        let mut new_circuits = Vec::new();
        for circ in &mut circuits {
            let contains_j1 = circ.contains(j1);
            let contains_j2 = circ.contains(j2);

            match (contains_j1, contains_j2) {
                (true, true) => continue 'outer,
                (false, false) => {
                    // not a circuit, make a new circuit
                    new_circuits.push(HashSet::from([*j1, *j2]));
                }
                (true, false) => {
                    // insert j2 in the circuit
                    circ.insert(j2);
                }
                (false, true) => {
                    // insert j1 into the circuit
                    circ.insert(j1);
                }
            }
        }
        circuits.append(&mut new_circuits);
    }
    circuits.sort_by_key(|v| v.len());
    let c = circuits
        .iter()
        .take(3)
        .map(|each| each.len() as i64)
        .collect::<Vec<_>>();
    println!("{:?}", c);
    c.iter().product()
}

fn pt_2(input: String) -> i64 {
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
