use std::{collections::HashSet, fmt::Display};

use utils::Input;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let result = part1("day_11/resources/example");
        assert_eq!(result, 1656);
    }

    #[test]
    fn part1_input() {
        let result = part1("day_11/resources/input");
        assert_eq!(result, 1588);
    }

    #[test]
    fn part2_example() {
        let result = part2("day_11/resources/example");
        assert_eq!(result, 195);
    }

    #[test]
    fn part2_input() {
        let result = part2("day_11/resources/input");
        assert_eq!(result, 517);
    }
}
type Octopuses = [[u32; 10]; 10];

struct Cave {
    octopuses: Octopuses,
    to_flash: HashSet<(usize, usize)>,
}

impl Cave {
    fn new(octopuses: Octopuses) -> Self {
        Cave {
            octopuses,
            to_flash: HashSet::new(),
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.octopuses {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Cave {
    fn step(&mut self) -> usize {
        self.increase_energy();
        self.flash()
    }

    fn increase_energy(&mut self) {
        for x in 0..10 {
            for y in 0..10 {
                self.increase_energy_of(x, y);
            }
        }
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y) = (x as isize, y as isize);
        let mut neighbours: Vec<(isize, isize)> = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if !(i == 0 && j == 0) {
                    if (x + i) >= 0 && (x + i) < 10 && (y + j) >= 0 && (y + j) < 10 {
                        neighbours.push((x + i, y + j));
                    }
                }
            }
        }
        neighbours
            .iter()
            .filter(|(x, y)| x >= &0 && y >= &0 && x < &10 && y < &10)
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect::<Vec<(usize, usize)>>()
    }

    fn increase_energy_of(&mut self, x: isize, y: isize) {
        let mut to_check = vec![(x as usize, y as usize)];

        while let Some(octopuse) = to_check.pop() {
            if self.to_flash.contains(&octopuse) {
                continue;
            }
            self.octopuses[octopuse.0][octopuse.1] += 1;

            // check if flashes
            if self.octopuses[octopuse.0][octopuse.1] > 9 {
                self.to_flash.insert(octopuse);
                // increase neighbours energy
                let neighbours = self.get_neighbours(octopuse.0, octopuse.1);
                for neighbour_octopuse in neighbours {
                    to_check.push(neighbour_octopuse);
                }
            }
        }
    }

    fn flash(&mut self) -> usize {
        let flashes = self.to_flash.len();
        for octopuse in self.to_flash.iter() {
            self.octopuses[octopuse.0][octopuse.1] = 0
        }
        self.to_flash.clear();
        flashes
    }
}

pub fn part1(path: &str) -> usize {
    let octopuses: Octopuses = Input::parse_matrix_10(path);

    let mut cave = Cave::new(octopuses);

    let mut flashes = 0;

    for step in 1..=100 {
        flashes += cave.step();
        println!("step: {}", step);
        println!("{}", cave);
        println!("");
    }
    flashes
}

pub fn part2(path: &str) -> usize {
    let octopuses: Octopuses = Input::parse_matrix_10(path);

    let mut cave = Cave::new(octopuses);

    for step in 1.. {
        let flashes = cave.step();

        if flashes == 10 * 10 {
            return step;
        }
    }
    0
}
