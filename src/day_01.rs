use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn count_calories(calories_list: Vec<String>) -> Vec<u32> {
    calories_list
        .group_by(|a, b| a.parse::<u32>().is_ok() && b.parse::<u32>().is_ok())
        .filter(|&x| x != ["".to_string()])
        .map(|v| v.iter().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect()
}

fn top_k_calories(calories_list: Vec<String>, k: usize) -> u32 {
    let mut tmp = count_calories(calories_list);
    tmp.sort_unstable();

    tmp.iter().rev().take(k).sum()
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let data: Vec<String> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().to_string())
        .collect();

    Ok(top_k_calories(data, 1).to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let data: Vec<String> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().to_string())
        .collect();

    Ok(top_k_calories(data, 3).to_string())
}

#[test]
fn test_part_1() {
    let data: Vec<String> = vec![
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(top_k_calories(data, 1), 24000);
}

#[test]
fn test_part_2() {
    let data: Vec<String> = vec![
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(top_k_calories(data, 3), 45000);
}
