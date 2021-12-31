#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        part1("day_10/resources/example.txt");
    }

    #[test]
    fn part1_input() {
        part1("day_10/resources/input.txt");
    }

    #[test]
    fn part2_example() {}

    #[test]
    fn part2_input() {}
}

use utils::*;

pub fn part1(path: &str) {
    let input = Input::new(path);
    let mut total_syntax_error_score = 0;
    for line in input {
        match test_line(line) {
            Ok(_) => (),
            Err((expected, found)) => {
                println!("Expected {}, but found {} instead.", expected, found);
                let score = match found {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
                total_syntax_error_score += score;
            }
        }
    }
    println!("Total syntax error score: {}", total_syntax_error_score);
}

fn test_line(line: String) -> Result<(), (char, char)> {
    let mut stack = vec![];
    for char in line.chars() {
        let result = match char {
            '{' | '[' | '(' | '<' => {
                stack.push(char);
                None
            }
            '}' => test_char(&mut stack, '{', char),
            ']' => test_char(&mut stack, '[', char),
            ')' => test_char(&mut stack, '(', char),
            '>' => test_char(&mut stack, '<', char),
            _ => None,
        };
        if let Some(result) = result {
            return result;
        }
    }
    Ok(())
}

fn test_char(
    stack: &mut Vec<char>,
    expected_char: char,
    actual_char: char,
) -> Option<Result<(), (char, char)>> {
    if let Some(expected) = stack.pop() {
        if expected != expected_char {
            return Some(Err((expected, actual_char)));
        }
    } else {
        return Some(Err((expected_char, actual_char)));
    }
    None
}
