use utils::Input;

use super::bits::*;

pub fn life_support_rating(input: Input) -> usize {
    let numbers = input
        .lines
        .map(|line| binary_string_to_number(&line.expect("Could not parse line")))
        .collect::<Vec<usize>>();

    let oxygen_generator_rating =
        filter_rating(numbers.clone(), true).expect("No oxygen generator found");

    let co2_scrubber_rating = filter_rating(numbers.clone(), false).expect("No co2 scrubber found");

    oxygen_generator_rating * co2_scrubber_rating
}

fn filter_rating(numbers: Vec<usize>, keep_most_common: bool) -> Option<usize> {
    let mut candidates = numbers;
    let mut filtered_candidates = Vec::new();

    loop {
        for index in (0..12).rev() {
            // find the most uncommon/common value for current candidates
            let most_common_value = most_common_flag_on(&candidates, index);
            let expected_bit_set = if keep_most_common {
                most_common_value
            } else {
                !most_common_value
            };

            // filter candidates based on the expected bit set
            for candidate in candidates.iter() {
                if is_bit_set(*candidate, index) == expected_bit_set {
                    filtered_candidates.push(*candidate);
                }
            }

            // if we have one candidate left, we're done
            if filtered_candidates.len() == 1 {
                break;
            }

            // set new candidates and reset filtered_candidates
            candidates = filtered_candidates;
            filtered_candidates = Vec::new();
        }
        break Some(filtered_candidates.first().unwrap().clone());
    }
}

fn most_common_flag_on(numbers: &Vec<usize>, index: u8) -> bool {
    let mut counts = 0;
    for number in numbers.iter() {
        counts += if is_bit_set(*number, index) { 1 } else { -1 };
    }
    counts >= 0
}
