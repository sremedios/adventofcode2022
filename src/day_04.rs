use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_data(s: &str) -> Vec<u32> {
    s.split(",")
        .flat_map(|s| s.split('-'))
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn within_range(a: u32, b: u32, x: u32, y: u32) -> bool {
    (a <= x && b >= y) || (x <= a && y >= b)
}

fn out_of_range(a: u32, b: u32, x: u32, y: u32) -> bool {
    !(x > b || y < a)
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result: String = BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| {
            if let [a, b, x, y] = &parse_data(s)[..] {
                within_range(*a, *b, *x, *y)
            } else {
                true
            }
        })
        .collect::<Vec<String>>()
        .len()
        .to_string();

    Ok(result)
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result: String = BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| {
            if let [a, b, x, y] = &parse_data(s)[..] {
                out_of_range(*a, *b, *x, *y)
            } else {
                true
            }
        })
        .collect::<Vec<String>>()
        .len()
        .to_string();

    Ok(result)
}

#[test]
fn test_part_1() {
    let data = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    let result = data
        .into_iter()
        .filter(|s| {
            if let [a, b, x, y] = &parse_data(s)[..] {
                within_range(*a, *b, *x, *y)
            } else {
                true
            }
        })
        .collect::<Vec<&str>>()
        .len();

    assert_eq!(result, 2);
}

#[test]
fn test_part_2() {
    let data = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    let result = data
        .into_iter()
        .filter(|s| {
            if let [a, b, x, y] = &parse_data(s)[..] {
                out_of_range(*a, *b, *x, *y)
            } else {
                false
            }
        })
        .collect::<Vec<&str>>()
        .len();

    assert_eq!(result, 4);
}
