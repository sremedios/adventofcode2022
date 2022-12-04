use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn halve_string(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

fn intersect(x: &str, y: &str) -> char {
    let x: HashSet<_> = HashSet::from_iter(x.chars());
    let y: HashSet<_> = HashSet::from_iter(y.chars());

    *x.intersection(&y).next().unwrap()
}

fn intersect3(x: &str, y: &str, z: &str) -> char {
    let mut a: Vec<char> = x.chars().collect();
    let sets: Vec<Vec<char>> = vec![y.chars().collect(), z.chars().collect()];

    for other in sets {
        a.retain(|v| other.contains(v));
    }

    a[0]
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result: String = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|s| {
            let t = &s.unwrap();
            let (x, y) = halve_string(t);
            to_priority(intersect(x, y))
        })
        .sum::<u32>()
        .to_string();
    Ok(result)
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result: String = BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|r| {
            let v = r;
            let x = &v[0];
            let y = &v[1];
            let z = &v[2];
            to_priority(intersect3(&x, &y, &z))
        })
        .sum::<u32>()
        .to_string();
    Ok(result)
}

#[test]
fn test_part_1() {
    let data = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    let result: u32 = data
        .iter()
        .map(|s| {
            let (x, y) = halve_string(s);
            to_priority(intersect(x, y))
        })
        .sum();

    assert_eq!(result, 157);
}

#[test]
fn test_part_2() {
    let data = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    let result: u32 = data
        .chunks(3)
        .map(|r| {
            let v = r;
            let x = v[0];
            let y = v[1];
            let z = v[2];
            to_priority(intersect3(x, y, z))
        })
        .sum();

    assert_eq!(result, 70);
}
