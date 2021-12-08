use utils::Input;

pub fn part2() {
    let mut buckets = vec![0; 9];

    // read input
    let mut input = Input::new("day_6/resources/input.txt");
    let initial_fishes = input
        .lines
        .next()
        .unwrap()
        .expect("Could not read line")
        .split(",")
        .map(|x| x.parse::<usize>().expect("Could not parse number"))
        .collect::<Vec<usize>>();

    // fill buckets
    for age in initial_fishes {
        buckets[age] += 1;
    }

    println!("Fishes {:?}", buckets);
    for day in 1..=256 {
        buckets.rotate_left(1);
        buckets[6] += buckets[8];

        println!(
            "Day {}, Fishes: {:?}, amount {}",
            day,
            buckets,
            buckets.iter().sum::<usize>()
        );
    }
}
