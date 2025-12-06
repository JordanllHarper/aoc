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

fn pt_1(input: String) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|each| each.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut access_paper = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, item) in line.iter().enumerate() {
            if *item == '@' {
                let mut around = 0;

                let vert = i.saturating_sub(1)..=i + 1;
                let hor = j.saturating_sub(1)..=j + 1;

                for around_i in vert {
                    for around_j in hor.clone() {
                        // we are on current
                        if around_i == i && around_j == j {
                            continue;
                        }
                        if let Some(around_char) =
                            grid.get(around_i).and_then(|it| it.get(around_j))
                            && *around_char == '@'
                        {
                            around += 1;
                        }
                    }
                }

                if around < 4 {
                    access_paper += 1;
                }
            }
        }
    }

    access_paper
}

fn pt_2(input: String) -> i32 {
    let mut removed_paper = 0;

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|each| each.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut change_occurred = true;

    while change_occurred {
        change_occurred = false;
        for (i, line) in grid.clone().iter().enumerate() {
            for (j, item) in line.iter().enumerate() {
                if *item == '@' {
                    let mut around = 0;

                    let vert = i.saturating_sub(1)..=i + 1;
                    let hor = j.saturating_sub(1)..=j + 1;

                    for around_i in vert {
                        for around_j in hor.clone() {
                            // we are on current
                            if around_i == i && around_j == j {
                                continue;
                            }
                            if let Some(around_char) =
                                grid.get(around_i).and_then(|it| it.get(around_j))
                                && *around_char == '@'
                            {
                                around += 1;
                            }
                        }
                    }

                    if around < 4 {
                        grid[i][j] = '.';
                        removed_paper += 1;
                        change_occurred = true;
                    }
                }
            }
        }
    }

    removed_paper
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
        assert_eq!(13, test);
    }
    #[test]
    fn t_pt2() {
        let test_content = fs::read_to_string(TEST).expect("Missing test text file");
        let test = pt_2(test_content);
        assert_eq!(43, test);
    }
}
