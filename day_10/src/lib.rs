#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1("day_10/resources/example.txt");
        assert_eq!(result, 26397);
    }

    #[test]
    fn part1_input() {
        let result = part1("day_10/resources/input.txt");
        assert_eq!(result, 323691);
    }

    #[test]
    fn part2_example() {
        let result = part2("day_10/resources/example.txt");
        assert_eq!(result, 288957)
    }

    #[test]
    fn part2_input() {
        let result = part2("day_10/resources/input.txt");
        assert_eq!(result, 2858785164)
    }
}

use utils::*;

pub fn part1(path: &str) -> i32 {
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
    total_syntax_error_score
}

pub fn part2(path: &str) -> usize {
    let input = Input::new(path);
    let mut scores = vec![];

    for line in input {
        match test_line(line) {
            Ok(open_chars) => {
                if open_chars.len() > 0 {
                    let score = open_chars
                        .iter()
                        .rev()
                        .fold(0, |acc, char| acc * 5 + get_close_char_score(char));

                    scores.push(score);
                }
            }
            Err(_) => (),
        }
    }

    scores.sort_unstable();
    let middle_score = scores[scores.len() / 2];

    println!("Middle score: {}", middle_score);
    middle_score
}

fn get_close_char_score(char: &char) -> usize {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn test_line(line: String) -> Result<Vec<char>, (char, char)> {
    let mut open_chars = vec![];
    for char in line.chars() {
        let result = match char {
            '{' | '[' | '(' | '<' => {
                open_chars.push(char);
                None
            }
            '}' => test_char(open_chars.pop(), '{', char),
            ']' => test_char(open_chars.pop(), '[', char),
            ')' => test_char(open_chars.pop(), '(', char),
            '>' => test_char(open_chars.pop(), '<', char),
            _ => None,
        };

        match result {
            Some(result) => return Err(result),
            None => (),
        };
    }
    Ok(open_chars)
}

fn test_char(
    open_char: Option<char>,
    expected_open_char: char,
    close_char: char,
) -> Option<(char, char)> {
    if open_char.is_none() || open_char.unwrap() != expected_open_char {
        return Some((expected_open_char, close_char));
    }

    None
}
