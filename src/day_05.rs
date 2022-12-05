use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_crate_init(filename: &str) -> (Vec<String>, Vec<String>) {
    let f = File::open(filename).unwrap();

    let mut crate_info: Vec<String> = Vec::new();

    let mut iter = BufReader::new(&f).lines().map(|l| l.unwrap());

    while let line = iter.next().unwrap() {
        if line == "" {
            break;
        } else {
            crate_info.push(line.to_owned());
        }
    }

    let instructions: Vec<String> = iter.map(|v| v.to_owned()).collect();

    (crate_info, instructions)
}

fn crate_to_stack(crate_info: Vec<String>) -> Vec<Vec<String>> {
    // Determine number of stacks by reading the last
    // ine of `crate_info`
    let num_crates = crate_info
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();
    let mut stacks: Vec<Vec<String>> = vec![Vec::new(); num_crates];

    // parse each line
    let result = crate_info
        .split_last()
        .unwrap()
        .1
        .iter()
        .map(|l| {
            l.as_bytes()
                .chunks(4)
                .map(|v| std::str::from_utf8(v).unwrap().trim().to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // parse into stacks
    for line in result.iter() {
        for (i, c) in line.iter().enumerate() {
            match c.as_str() {
                "" => (),
                v => stacks[i].push(v.to_owned()),
                _ => panic!("Error in crate stack string parsing"),
            }
        }
    }
    stacks
}

fn parse_move_cmd(cmd: &str) -> (u32, u32, u32) {
    let amt: u32 = cmd.as
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result: String = "part 1 placeholder".to_owned();

    Ok(result)
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result: String = "part 2 placeholder".to_owned();
    Ok(result)
}

#[test]
fn test_part_1() {
    let test_file = "/home/sam/adventofcode2022/resources/day_05_test.txt";
    let (crate_info, instructions) = parse_crate_init(test_file);

    eprintln!("{:#?}", crate_to_stack(crate_info));
    eprintln!("{:#?}", instructions);

    assert_eq!(1, 2);
}
