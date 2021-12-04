use std::time::Instant;

use utils::Input;

mod bits;
mod part_1;
mod part_2;

use part_1::*;
use part_2::*;

pub static FILE_NAME: &str = "day_3/resources/input.txt";

fn main() {
    let calculate_power_consumption = calculate_power_consumption(Input::new(FILE_NAME));
    println!(
        "calculate_power_consumption: {}",
        calculate_power_consumption
    );

    let start = Instant::now();
    let life_support_rating = life_support_rating(Input::new(FILE_NAME)); // 4667284 637470 2974222
    let duration = start.elapsed();

    println!(
        "Time elapsed in life_support_rating() is: {:?} and result is {}",
        duration, life_support_rating
    );
}
