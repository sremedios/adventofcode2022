use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn seeker(s: &str, n: usize) -> usize {
    let mut i = n - 1;

    let iter = s.chars().collect::<Vec<char>>();

    for substring in iter.windows(n) {
        i += 1;
        let window: HashSet<&char> = HashSet::from_iter(substring.iter());

        if window.len() == n {
            break;
        }
    }
    i
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let data = &BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()[0];

    let result: String = seeker(&data, 4).to_string();

    Ok(result)
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let data = &BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()[0];

    let result: String = seeker(&data, 14).to_string();

    Ok(result)
}

#[test]
fn test_part_1() {
    let marker_len = 4;

    let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(seeker(data, marker_len), 7);

    let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(seeker(data, marker_len), 5);

    let data = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(seeker(data, marker_len), 6);

    let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(seeker(data, marker_len), 10);

    let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(seeker(data, marker_len), 11);
}

#[test]
fn test_part_2() {
    let marker_len = 14;

    let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(seeker(data, marker_len), 19);

    let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(seeker(data, marker_len), 23);

    let data = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(seeker(data, marker_len), 23);

    let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(seeker(data, marker_len), 29);

    let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(seeker(data, marker_len), 26);
}
