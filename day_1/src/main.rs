use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read lines
    let file_name = "resources/input.txt";
    let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

    let mut measure_window: Vec<usize> = Vec::with_capacity(3);
    let mut result = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // we expect the line represents a number
                let measure = line.parse::<usize>().unwrap();
                if measure_window.len() < 3 {
                    measure_window.push(measure);
                } else {
                    // we have a full window
                    let first_window_sum: usize = measure_window.iter().sum();

                    measure_window.remove(0);
                    measure_window.push(measure);

                    let second_window_sum: usize = measure_window.iter().sum();

                    if first_window_sum < second_window_sum {
                        result += 1;
                    }
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }

    println!("result: {}", result);
}
