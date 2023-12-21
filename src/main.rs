use std::fs::File;
use std::io::{BufReader, BufRead};
fn main() {
    println!("Hello world");
}

fn get_first_digit<T>(text: T) -> u32
where
    T: Iterator<Item = char>, 
{
    for c in text {
        if let Some(digit) = c.to_digit(10) {
            return digit;
        }
    }
    0
}

fn get_calibration_value(lines: Vec<String>) -> u32 {
    let mut res = 0;
    for line in lines {
        let first = get_first_digit(line.chars());
        let last = get_first_digit(line.chars().rev());
        res += first * 10 + last;
    }
    res  
}

#[test]
fn testcase_1() {
    let lines = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];
    assert_eq!(get_calibration_value(lines), 142);
}

#[test]
fn testcase_2() {
    let file = File::open("data/input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|el| el.expect("Could not parse line"))
        .collect::<Vec<String>>();
    assert_eq!(get_calibration_value(lines), 55607);
}