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
