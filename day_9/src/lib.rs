use utils::Input;

pub fn part_1(path: &str) {
    let matrix = Input::parse_matrix(path);
    let low_points = get_low_points(&matrix);

    let risk_levels_sum = low_points.iter().map(|(_, _, v)| v + 1).sum::<u32>();
    println!("{}", risk_levels_sum);
}

pub fn part_2(path: &str) {
    let matrix = Input::parse_matrix(path);
    let low_points = get_low_points(&matrix);
    let mut basin_sizes = vec![];

    for low_point in low_points {
        let basin = get_basin(&matrix, low_point);
        basin_sizes.push(basin.len());
    }

    basin_sizes.sort();
    println!("{}", basin_sizes.iter().rev().take(3).product::<usize>());
}

fn get(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<u32> {
    if matrix.get(x).is_none() || matrix.get(x).unwrap().get(y).is_none() {
        return None;
    }

    matrix.get(x).unwrap().get(y).map(|v| *v)
}

fn get_point(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<Point> {
    get(matrix, x, y).map(|v| (x, y, v))
}

fn is_low_point(x: u32, values: [Option<u32>; 4]) -> bool {
    values.iter().all(|v| v.is_none() || v.unwrap() > x)
}

fn get_adjacents(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> [Option<u32>; 4] {
    let top = if y > 0 { get(&matrix, x, y - 1) } else { None };
    let down = get(&matrix, x, y + 1);
    let left = if x > 0 { get(&matrix, x - 1, y) } else { None };
    let right = get(&matrix, x + 1, y);
    [top, down, left, right]
}

fn get_adjacent_points(
    matrix: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
) -> [Option<(usize, usize, u32)>; 4] {
    let top = if y > 0 {
        get_point(&matrix, x, y - 1)
    } else {
        None
    };
    let down = get_point(&matrix, x, y + 1);
    let left = if x > 0 {
        get_point(&matrix, x - 1, y)
    } else {
        None
    };
    let right = get_point(&matrix, x + 1, y);
    [top, down, left, right]
}

fn get_low_points(matrix: &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let mut low_points = vec![];

    for (x, row) in matrix.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if is_low_point(*value, get_adjacents(&matrix, x, y)) {
                low_points.push((x, y, *value));
            }
        }
    }

    low_points
}

type Point = (usize, usize, u32);

fn get_basin(matrix: &Vec<Vec<u32>>, low_point: Point) -> Vec<(usize, usize, u32)> {
    let mut basin = vec![];
    let mut visited: Vec<Point> = vec![];
    let mut queue: Vec<Point> = vec![low_point];

    while let Some(visiting) = queue.pop() {
        visited.push(visiting);
        let (x, y, value) = visiting;
        match value {
            9 => continue,
            _ => basin.push(visiting),
        }
        for adjacent_point in get_adjacent_points(&matrix, x, y) {
            if let Some(adjacent_point) = adjacent_point {
                if !visited.contains(&adjacent_point) && !&queue.contains(&adjacent_point) {
                    queue.push(adjacent_point);
                }
            }
        }
    }

    basin
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use crate::part_1;

        #[test]
        fn example_input() {
            part_1("day_9/resources/example.txt");
        }

        #[test]
        fn input() {
            part_1("day_9/resources/input.txt");
        }
    }

    mod part_2 {
        use crate::part_2;

        #[test]
        fn example_input() {
            part_2("day_9/resources/example.txt");
        }

        #[test]
        fn input() {
            part_2("day_9/resources/input.txt");
        }
    }
}
