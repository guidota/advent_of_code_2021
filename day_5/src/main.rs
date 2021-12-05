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

    // diagonal iteration
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut x = self.origin.0;
        let mut y = self.origin.1;
        let dest_x = self.destination.0;
        let dest_y = self.destination.1;
        let mut last = true;
        std::iter::from_fn(move || {
            let result = (x, y);
            // move x to dest
            if x < dest_x {
                x += 1;
            } else {
                x -= 1;
            }

            if y < dest_y {
                y += 1;
            } else {
                y -= 1;
            }

            if x == dest_x && y == dest_y {
                if last {
                    last = false;
                    return Some(result);
                } else {
                    return None;
                }
            }
            Some(result)
        })
    }
}

fn main() {
    let input = Input::new("day_5/resources/input.txt");

    let lines = get_lines(input);

    let mut map = HashMap::new();
    for line in lines {
        if line.origin.0 == line.destination.0 || line.origin.1 == line.destination.1 {
            // iterate horizontally or vertically
            for x in line.horizontal_iter() {
                for y in line.vertical_iter() {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
        } else {
            // iterate diagnoally
            for (x, y) in line.iter() {
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
