use utils::*;

struct Lanternfish {
    age: usize,
}

pub fn part1() {
    let mut fishes = Vec::<Lanternfish>::new();
    spawn_initial_lanternfishes(&mut fishes);
    for day in 1..=80 {
        advance_day(&mut fishes);
        println!("Day {}, Lanternfishes: {}", day, fishes.iter().count());
    }
}

fn spawn_initial_lanternfishes(fishes: &mut Vec<Lanternfish>) {
    let input = Input::new("day_6/resources/input.txt");
    for line in input.lines {
        let line = line.expect("Could not read line");
        for number in line.split(",") {
            fishes.push(Lanternfish {
                age: number.parse::<usize>().expect("Could not parse number"),
            });
        }
    }
}

fn advance_day(fishes: &mut Vec<Lanternfish>) {
    let mut new_fishes = 0;
    for mut lanternfish in fishes.iter_mut() {
        match lanternfish.age {
            0 => {
                new_fishes += 1;
                lanternfish.age = 6;
            }
            _ => {
                lanternfish.age -= 1;
            }
        }
    }
    for _ in 0..new_fishes {
        fishes.push(Lanternfish { age: 8 });
    }
}
