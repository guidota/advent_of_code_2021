use std::collections::{HashMap, HashSet};

use utils::Input;

pub fn part1(path: &str) -> usize {
    let connections_input = Input::new(path);
    let connections = find_connections(connections_input);
    let paths = find_paths(connections);
    paths.len()
}

pub fn part2(path: &str) -> usize {
    let connections_input = Input::new(path);
    let connections = find_connections(connections_input);
    let paths = find_paths_with_extra_visit(connections);
    paths.len()
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}

fn is_not_start_or_end(cave: &str) -> bool {
    cave != "start" && cave != "end"
}

fn find_connections(connections_input: Input) -> HashMap<String, Vec<String>> {
    let mut connections = HashMap::new();
    for connection in connections_input {
        let (first, second) = connection.split_once('-').expect("connection");

        connections
            .entry(first.to_string())
            .or_insert(Vec::new())
            .push(second.to_string());
        connections
            .entry(second.to_string())
            .or_insert(Vec::new())
            .push(first.to_string());
    }
    connections
}

fn find_paths(connections: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let mut queue = Vec::new();

    queue.push((String::from("start"), vec![]));
    while let Some((cave, mut path)) = queue.pop() {
        if is_small_cave(&cave) && path.contains(&cave) {
            continue;
        }
        path.push(cave.clone());

        if cave == "end" {
            paths.push(path);
            continue;
        }
        if let Some(connections) = connections.get(&cave) {
            for connection in connections {
                queue.push((connection.to_string(), path.to_vec()));
            }
        }
    }
    paths
}

fn find_paths_with_extra_visit(connections: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let mut queue = Vec::new();

    queue.push((String::from("start"), vec![], false));
    while let Some((cave, mut path, mut was_a_cave_repeated)) = queue.pop() {
        if is_small_cave(&cave) && path.contains(&cave) {
            if cave == "start" {
                continue;
            }
            if was_a_cave_repeated {
                continue;
            }
            was_a_cave_repeated = true;
        }

        path.push(cave.clone());

        if cave == "end" {
            paths.push(path);
            continue;
        }
        if let Some(connections) = connections.get(&cave) {
            for connection in connections {
                queue.push((connection.to_string(), path.to_vec(), was_a_cave_repeated));
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let result = part1("day_12/resources/example0");
        assert_eq!(result, 10);
    }

    #[test]
    fn part1_example1() {
        let result = part1("day_12/resources/example1");
        assert_eq!(result, 19);
    }

    #[test]
    fn part1_input() {
        let result = part1("day_12/resources/input");
        println!("{}", result);
        assert_eq!(result, 4413);
    }

    #[test]
    fn part2_example() {
        let result = part2("day_12/resources/example0");
        println!("{}", result);
        assert_eq!(result, 36);
    }

    #[test]
    fn part2_example2() {
        let result = part2("day_12/resources/example1");
        println!("{}", result);
        assert_eq!(result, 103);
    }

    #[test]
    fn part2_example3() {
        let result = part2("day_12/resources/example2");
        println!("{}", result);
        assert_eq!(result, 3509);
    }

    #[test]
    fn part2_input() {
        let result = part2("day_12/resources/input");
        println!("{}", result);
        assert_eq!(result, 118803);
    }
}
