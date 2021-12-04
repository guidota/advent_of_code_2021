use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read lines
    let file_name = "src/input.txt";
    let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.expect("error reading line");
        let (command, x) = parse_line(&line);

        match command {
            "forward" => {
                horizontal += x;
                depth += aim * x;
            }
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!("unknown command"),
        }
    }

    println!("{}", horizontal * depth);
}

fn parse_line(line: &String) -> (&str, usize) {
    let words = line.split_whitespace().collect::<Vec<&str>>();
    if words.len() != 2 && words[1].parse::<usize>().is_err() {
        panic!("invalid line: {}", line);
    }

    (
        words[0],
        words[1].parse::<usize>().expect("error parsing number"),
    )
}
