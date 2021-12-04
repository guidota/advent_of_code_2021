use super::bits::*;
use utils::Input;

pub fn calculate_power_consumption(input: Input) -> usize {
    let bits_count = count_bits(input);
    println!("Bits: {:?}", bits_count);

    let gamma_rate = gamma_rate(bits_count);
    println!("Gamma rate {:#018b}", gamma_rate);

    let epsilon_rate = toggle_number(gamma_rate);
    println!("Epsilon rate {:#018b}", epsilon_rate);

    gamma_rate * epsilon_rate
}

pub fn gamma_rate(bits: Vec<isize>) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |mut acc, (index, bit)| {
            if *bit >= 0 {
                set_bit(&mut acc, index.try_into().unwrap());
            }
            acc
        })
}
