use utils::Input;

use super::bits::*;
use super::*;

pub fn life_support_rating(input: Input) -> usize {
    let bits_count = count_bits(input);
    let common_bits = gamma_rate(bits_count);
    let least_common_bits = toggle_number(common_bits);

    let oxygen_generator_rating =
        filter_rating(Input::new(FILE_NAME), common_bits).expect("No oxygen generator found");

    let co2_scrubber_rating =
        filter_rating(Input::new(FILE_NAME), least_common_bits).expect("No co2 scrubber found");

    println!("most common bits: {:#014b}", common_bits);
    println!("Oxygen generator rating: {:#014b}", oxygen_generator_rating);

    println!("least common bits: {:#014b}", least_common_bits);
    println!("CO2 scrubber rating: {:#014b}", co2_scrubber_rating);

    oxygen_generator_rating * co2_scrubber_rating
}

fn filter_rating(input: Input, bit_criteria: usize) -> Option<usize> {
    let mut candidates: Vec<Vec<usize>> = vec![Vec::new(); 12];

    for line in input {
        let value = binary_string_to_number(&line);
        for index in (0..12).rev() {
            if is_bit_set(value, index) != is_bit_set(bit_criteria, index) {
                break;
            }
            candidates[index as usize].push(value);
        }
    }

    candidates
        .iter()
        .find(|candidate| candidate.len() == 1)
        .expect("No candidate found")
        .first()
        .cloned()
}
