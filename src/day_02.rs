use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn decrypt(shape_txt: &str) -> Shape {
    match shape_txt {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Invalid input for decrypt."),
    }
}

fn calc_needed_shape(opp_shape: Shape, inp_code: &str) -> Shape {
    match (opp_shape, inp_code) {
        (Shape::Rock, "X") => Shape::Scissors,
        (Shape::Rock, "Y") => Shape::Rock,
        (Shape::Rock, "Z") => Shape::Paper,
        (Shape::Paper, "X") => Shape::Rock,
        (Shape::Paper, "Y") => Shape::Paper,
        (Shape::Paper, "Z") => Shape::Scissors,
        (Shape::Scissors, "X") => Shape::Paper,
        (Shape::Scissors, "Y") => Shape::Scissors,
        (Shape::Scissors, "Z") => Shape::Rock,
        _ => panic!("Impossible"),
    }
}

fn my_score(me: Shape) -> u32 {
    match me {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn calc_win(opp: Shape, me: Shape) -> u32 {
    match (opp, me) {
        (Shape::Rock, Shape::Scissors) => 0,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Scissors, Shape::Scissors) => 3,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
    }
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|v| calc_win(decrypt(&v[0]), decrypt(&v[1])) + my_score(decrypt(&v[1])))
        .sum::<u32>()
        .to_string();

    Ok(result)
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let result = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|v| {
            let o = decrypt(&v[0]);
            let m = calc_needed_shape(o.clone(), &v[1]);
            calc_win(o, m) + my_score(m)
        })
        .sum::<u32>()
        .to_string();

    Ok(result)
}

#[test]
fn test_part_1() {
    let data = vec![("A", "Y"), ("B", "X"), ("C", "Z")];

    let result = data
        .into_iter()
        .map(|(opp, me)| calc_win(decrypt(opp), decrypt(me)) + my_score(decrypt(me)))
        .collect::<Vec<u32>>();

    assert_eq!(result.iter().sum::<u32>(), 15);
}

#[test]
fn test_part_2() {
    let data = vec![("A", "Y"), ("B", "X"), ("C", "Z")];

    let result = data
        .into_iter()
        .map(|(opp, me)| {
            let o = decrypt(opp);
            let m = calc_needed_shape(o.clone(), me);
            calc_win(o, m) + my_score(m)
        })
        .collect::<Vec<u32>>();

    assert_eq!(result.iter().sum::<u32>(), 12);
}
