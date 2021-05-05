use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

fn try_read<T: BufRead>(reader: &mut T) -> String {
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("error: Failed to read input from stdin");

    buffer.trim().to_string()
}

fn read<T: FromStr, U: BufRead>(reader: &mut U) -> T
where
    <T as FromStr>::Err: Debug,
{
    let value: T = try_read(reader)
        .parse()
        .expect("error: Failed to parse input from stdin");

    value
}

#[allow(dead_code)]
fn read_vec<T: FromStr, U: BufRead>(reader: &mut U) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let line = try_read(reader);

    let vec: Vec<T> = line
        .split_whitespace()
        .map(|part| {
            part.parse()
                .expect("error: Failed to parse input from stdin")
        })
        .collect();

    vec
}

fn solve(expression: String) -> usize {
    let mut longest_prefix = 0;

    let mut count_open_broket = 0;
    let mut count_pairs = 0;

    for broket in expression.chars() {
        match broket {
            '<' => {
                count_open_broket += 1;
            }
            '>' => {
                count_open_broket -= 1;

                if count_open_broket < 0 {
                    break;
                } else if count_open_broket == 0 {
                    longest_prefix += (count_pairs + 1) * 2;
                    count_pairs = 0;
                } else {
                    count_pairs += 1;
                }
            }
            _ => {}
        }
    }

    longest_prefix
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let num_test_cases: usize = read(&mut reader);
    for _ in 0..num_test_cases {
        let expression = try_read(&mut reader);

        let longest_prefix = solve(expression);
        println!("{}", longest_prefix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tests = vec![
            (4, "<<>>"),
            (0, "><"),
            (2, "<>>>"),
            //
            (0, ""),
            (0, "<"),
            (0, ">"),
            //
            (2, "<>"),
            (0, "<<"),
            (0, ">>"),
            //
            (0, "><>"),
            (0, "<<>"),
            (2, "<><"),
            (2, "<>>"),
            //
            (4, "<><>"),
            (2, "<><<"),
            (0, "<<><"),
            (0, "<<<>"),
            (0, ">><<"),
        ];

        for (want, input) in tests {
            println!("should return {} with input {:?}", want, input);

            let got = solve(input.to_string());
            assert_eq!(want, got);
        }
    }
}
