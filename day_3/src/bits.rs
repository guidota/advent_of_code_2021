use utils::Input;

pub fn count_bits(input: Input) -> Vec<isize> {
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

pub fn toggle_number(number: usize) -> usize {
    let mut result = number.clone();
    for index in 0..12 {
        toggle_bit(&mut result, index);
    }

    result
}

pub fn set_bit(original: &mut usize, bit: u8) {
    let mask = 1 << bit;
    *original = *original | mask;
}

pub fn toggle_bit(original: &mut usize, bit: u8) {
    let mask = 1 << bit;
    *original = *original ^ mask;
}

pub fn is_bit_set(original: usize, bit: u8) -> bool {
    let mask = 1 << bit;
    (original & mask) != 0
}

pub fn binary_string_to_number(binary_string: &String) -> usize {
    let mut result = 0;
    for (index, char) in binary_string.chars().rev().enumerate() {
        if char == '1' {
            set_bit(&mut result, index.try_into().unwrap());
        }
    }
    result
}
