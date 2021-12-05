use std::collections::HashMap;

use utils::Input;

#[derive(Debug)]
struct Line {
    origin: (usize, usize),
    destination: (usize, usize),
}

impl Line {
    fn horizontal_iter(&self) -> impl Iterator<Item = usize> {
        let min = self.origin.0.min(self.destination.0);
        let max = self.origin.0.max(self.destination.0);
        min..max + 1
    }

    fn vertical_iter(&self) -> impl Iterator<Item = usize> {
        let min = self.origin.1.min(self.destination.1);
        let max = self.origin.1.max(self.destination.1);
        min..max + 1
    }
}

fn main() {
    let input = Input::new("day_5/resources/input.txt");

    let lines = get_lines(input);

    let mut map = HashMap::new();
    for line in lines {
        // only consider horizontal or vertical lines
        if (line.origin.0 != line.destination.0) && (line.origin.1 != line.destination.1) {
            continue;
        }

        for x in line.horizontal_iter() {
            for y in line.vertical_iter() {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    println!(
        "Positions with 2 or more: {}",
        map.iter().filter(|(_, v)| **v > 1).count()
    );
}

fn get_lines(input: Input) -> Vec<Line> {
    input
        .lines
        .map(|line| {
            let line = line.expect("Parsing error");
            let mut parts = line.split(" -> ");
            let origin = parts.next().map(read_coord).expect("Parsing error");
            let destination = parts.next().map(read_coord).expect("Parsing error");
            Line {
                origin,
                destination,
            }
        })
        .collect::<Vec<Line>>()
}

fn read_coord(coord: &str) -> (usize, usize) {
    let mut parts = coord.split(",");
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}
