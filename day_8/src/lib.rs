use std::collections::HashMap;

use utils::Input;

fn diff(first: &str, second: &str) -> Vec<char> {
    first
        .chars()
        .filter(|c| !second.contains(&c.to_string()))
        .collect()
}

fn all_in(first: &str, second: &str) -> bool {
    first.chars().all(|c| second.contains(&c.to_string()))
}

fn join(first: &str, second: &str) -> usize {
    first
        .chars()
        .filter(|c| second.contains(&c.to_string()))
        .count()
}

fn same(first: &str, second: &str) -> bool {
    let join = join(first, second);
    join == second.len() && join == first.len()
}

pub fn parse_digits(path: &str) {
    let input = Input::new(path);

    let count: usize = input.lines.map(|s| s.unwrap()).map(calculate_ouput).sum();
    println!("count: {}", count);
}

fn calculate_ouput(line: String) -> usize {
    let mut result = 0;
    if let Some((pattern, output)) = line.split_once('|') {
        println!("pattern {}, output {}", pattern.trim(), output.trim());
        let mut map = HashMap::new();
        for number in pattern.trim().split_whitespace() {
            map.entry(number.len()).or_insert(vec![]).push(number);
        }
        let one = *map.get(&2).unwrap().first().unwrap();
        let four = *map.get(&4).unwrap().first().unwrap();
        let seven = *map.get(&3).unwrap().first().unwrap();
        let eight = *map.get(&7).unwrap().first().unwrap();

        let mut zero = "";
        let mut two = "";
        let mut three = "";
        let mut five = "";
        let mut six = "";
        let mut nine = "";

        for candidate in map.get(&6).unwrap().iter() {
            if all_in(&one, candidate) {
                if all_in(&four, &candidate) {
                    nine = candidate;
                } else {
                    zero = candidate;
                }
            } else {
                six = candidate;
            }
        }
        for candidate in map.get(&5).unwrap().iter() {
            if all_in(&one, candidate) {
                three = candidate;
            } else {
                if diff(&four, candidate).len() == 1 {
                    five = candidate;
                } else {
                    two = candidate;
                }
            }
        }

        result += output
            .trim()
            .split(' ')
            .map(|d| match d {
                d if same(&zero, d) => "0",
                d if same(&one, d) => "1",
                d if same(&two, d) => "2",
                d if same(&three, d) => "3",
                d if same(&four, d) => "4",
                d if same(&five, d) => "5",
                d if same(&six, d) => "6",
                d if same(&seven, d) => "7",
                d if same(&eight, d) => "8",
                d if same(&nine, d) => "9",
                _ => "",
            })
            .fold("".to_string(), |mut acc, d| {
                acc.push_str(d);
                acc
            })
            .parse::<usize>()
            .unwrap();
        println!("result was {}", result);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::parse_digits;

    #[test]
    fn example_input() {
        parse_digits("day_8/resources/example.txt");
    }

    #[test]
    fn real_input() {
        parse_digits("day_8/resources/input.txt");
    }
}
