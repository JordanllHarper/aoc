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

fn pt_1(input: String) -> u64 {
    let start_char = 'S';
    let grid: Vec<Vec<(usize, usize, char)>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| row.chars().enumerate().map(|(x, c)| (y, x, c)).collect())
        .collect();
    let (start_y, start_x, _) = grid
        .first()
        .expect("Invalid grid")
        .iter()
        .find(|(_, _, c)| *c == start_char)
        .expect("Invalid start");
    let mut num_splits = 0;
    // We have the start, now we need to
    // - Iterate downwards until we find a splitter
    // - When we find a splitter:
    //  - Split the line in 2
    //  - Keep track of the splits
    //  - If the split is not a merge (i.e. a new beam):
    //      inc num_splits
    //
    // We can handle merges using a hash set to track coords
    //

    let mut coords: HashSet<Coord> = HashSet::from([Coord {
        y: *start_y,
        x: *start_x,
    }]);

    'main: loop {
        let mut new_coords = coords.clone();
        let mut iter_coords = coords.into_iter().collect::<Vec<Coord>>();

        iter_coords.sort_by(|a, b| a.x.cmp(&b.x));
        for coord in iter_coords.into_iter().collect::<Vec<Coord>>() {
            let next_y = coord.y + 1;

            new_coords.retain(|each| each.y > coord.y);

            let maybe_pos = grid
                .get(next_y)
                .map(|s| s.get(coord.x).expect("Invalid x coord"));
            // we hit the end if this is none
            let (_, x, c) = match maybe_pos {
                Some(v) => v,
                None => break 'main,
            };

            if *c != '^' {
                let next = Coord {
                    y: next_y,
                    x: coord.x,
                };
                let _ = new_coords.insert(next);
                continue;
            }

            // Insert 1 to left and right,
            let left = Coord {
                y: next_y,
                x: x - 1,
            };
            let right = Coord {
                y: next_y,
                x: x + 1,
            };
            let is_new_left_split = new_coords.insert(left);
            let is_new_right_split = new_coords.insert(right);
            if !is_new_left_split && !is_new_right_split {
                continue;
            }
            num_splits += 1;
        }

        coords = new_coords;
    }
    num_splits
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    y: usize,
    x: usize,
}

fn pt_2(input: String) -> u64 {
    let start_char = 'S';
    let grid: Vec<Vec<(usize, usize, char)>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| row.chars().enumerate().map(|(x, c)| (y, x, c)).collect())
        .collect();
    let (start_y, start_x, _) = grid
        .first()
        .expect("Invalid grid")
        .iter()
        .find(|(_, _, c)| *c == start_char)
        .expect("Invalid start");

    let mut coords: HashMap<Coord, u64> = HashMap::from([(
        Coord {
            y: *start_y,
            x: *start_x,
        },
        1,
    )]);

    loop {
        let mut new_coords = coords.clone();
        println!("{:?}", coords);
        let iter_coords = coords.into_iter().collect::<Vec<(Coord, u64)>>();
        for (coord, num_timelines) in iter_coords {
            let next_y = coord.y + 1;

            let maybe_pos = grid
                .get(next_y)
                .map(|s| s.get(coord.x).expect("Invalid x coord"));
            // we hit the end if this is none
            let (_, x, c) = match maybe_pos {
                Some(v) => v,
                None => return new_coords.values().sum(),
            };

            if *c != '^' {
                let next = Coord {
                    y: next_y,
                    x: coord.x,
                };
                if let Some(c) = new_coords.get_mut(&next) {
                    *c += num_timelines;
                } else {
                    let _ = new_coords.insert(next, num_timelines);
                }

                new_coords.remove(&coord);
                continue;
            }

            // Insert 1 to left and right,
            let left = Coord {
                y: next_y,
                x: x - 1,
            };
            let right = Coord {
                y: next_y,
                x: x + 1,
            };
            if let Some(l) = new_coords.get_mut(&left) {
                println!("Merge at left");
                *l += num_timelines;
            } else {
                new_coords.insert(left, num_timelines);
            }

            if let Some(l) = new_coords.get_mut(&right) {
                println!("Merge at right");
                *l += num_timelines;
            } else {
                new_coords.insert(right, num_timelines);
            }
            new_coords.remove(&coord);
        }

        coords = new_coords;
    }
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
        assert_eq!(21, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        if test_content.is_empty() {
            panic!("Test file is empty! Double check the contents.");
        }
        let test = pt_2(test_content);
        assert_eq!(40, test);
    }
}
