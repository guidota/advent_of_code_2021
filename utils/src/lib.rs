use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub struct Input {
    pub lines: Lines<BufReader<File>>,
}

impl Input {
    pub fn new(file_name: &str) -> Self {
        let buffer = BufReader::new(File::open(file_name).expect("File not found"));
        let lines = buffer.lines();
        Input { lines }
    }

    pub fn parse_numbers(file_name: &str) -> Vec<i32> {
        let buffer = BufReader::new(File::open(file_name).expect("File not found"));
        buffer
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|number| number.parse::<i32>().unwrap())
            .collect()
    }
    pub fn parse_matrix(file_name: &str) -> Vec<Vec<u32>> {
        let buffer = BufReader::new(File::open(file_name).expect("File not found"));
        buffer
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    }
}

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line {
            Some(Ok(line)) => Some(line),
            Some(Err(_)) => {
                println!("Error reading line");
                None
            }
            _ => None,
        }
    }
}
