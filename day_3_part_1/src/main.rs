use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct Input {
    lines: Lines<BufReader<File>>,
}

impl Input {
    fn new(file_name: &str) -> Self {
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

fn main() {
    // read lines
    let file_name = "src/input.txt";
    let input = Input::new(file_name);

    dbg!(calculate_power_consumption(input));
}

fn calculate_power_consumption(input: Input) -> i32 {
    let bits_count = count_bits(input);
    println!("Bits: {:?}", bits_count);

    let gamma_rate = gamma_rate(bits_count);
    println!("Gamma rate {:#018b}", gamma_rate);

    let epsilon_rate = toggle_number(gamma_rate);
    println!("Epsilon rate {:#018b}", epsilon_rate);

    gamma_rate * epsilon_rate
}

fn count_bits(input: Input) -> Vec<i32> {
    let mut bits = vec![0; 12];
    for item in input {
        for (index, char) in item.chars().enumerate() {
            match char {
                '1' => bits[index] += 1,
                '0' => bits[index] -= 1,
                _ => panic!("Invalid bit"),
            }
        }
    }
    bits
}

fn gamma_rate(bits: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for (index, bit) in bits.iter().rev().enumerate() {
        if *bit > 0 {
            set_bit(&mut result, index.try_into().unwrap());
        } else {
            clear_bit(&mut result, index.try_into().unwrap());
        }
        println!("{:#018b}", result);
    }

    result
}

fn toggle_number(number: i32) -> i32 {
    let mut result = number;
    for index in 0..12 {
        toggle_bit(&mut result, index);
    }

    result
}

fn set_bit(original: &mut i32, bit: i32) {
    let mask = 1 << bit;
    *original = *original | mask;
}

fn clear_bit(original: &mut i32, bit: i32) {
    let mask = 1 << bit;
    *original = *original & !mask;
}

fn toggle_bit(original: &mut i32, bit: i32) {
    let mask = 1 << bit;
    *original = *original ^ mask;
}
