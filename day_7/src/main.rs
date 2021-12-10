use utils::Input;

fn main() {
    part1();

    part2();
}

fn part1() {
    let mut numbers = Input::parse_numbers("day_7/resources/input.txt");
    // find median
    numbers.sort();
    let median = numbers[numbers.len() / 2];

    // sum fuel from all crabs
    let fuel = numbers.iter().fold(0, |acc, n| {
        let diff = if n < &median { median - n } else { n - median };
        acc + diff
    });

    println!("{}", fuel);
}

fn part2() {
    let crabs = Input::parse_numbers("day_7/resources/input.txt");

    let mut min_fuel = std::i32::MAX;
    let mut min_position = 0;

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    for position in *min..=*max {
        let mut fuel = 0;
        for crab2 in crabs.iter() {
            if position == *crab2 {
                continue;
            }
            fuel += calculate_fuel(*crab2, position);
        }
        if fuel < min_fuel {
            min_position = position;
            min_fuel = fuel;
        }
    }
    println!("min position {}, min fuel {}", min_position, min_fuel);
}

fn calculate_fuel(crab: i32, position: i32) -> i32 {
    let diff = if crab < position {
        position - crab
    } else {
        crab - position
    };
    (1..=diff).fold(0, |acc, i| acc + i)
}
