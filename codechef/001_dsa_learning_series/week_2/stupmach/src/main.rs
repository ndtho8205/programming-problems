use std::cmp::min;
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

fn solve(boxes: Vec<usize>) -> usize {
    let mut count_tokens = 0;
    let mut min_capacity = usize::MAX;

    for box_capacity in boxes {
        min_capacity = min(min_capacity, box_capacity);
        count_tokens += min_capacity;

        if min_capacity == 0 {
            break;
        }
    }

    count_tokens
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let num_test_cases: usize = read(&mut reader);
    for _ in 0..num_test_cases {
        let num_boxes: usize = read(&mut reader);
        let boxes: Vec<usize> = read_vec(&mut reader);

        assert_eq!(num_boxes, boxes.len());

        let max_tokens = solve(boxes);
        println!("{}", max_tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tests = vec![
            (4, (3, vec![2, 1, 3])),
            //
            (0, (0, vec![])),
            //
            (0, (1, vec![0])),
            (1, (1, vec![1])),
            (2, (1, vec![2])),
            //
            (0, (2, vec![0, 0])),
            (0, (2, vec![0, 1])),
            (0, (2, vec![0, 2])),
            (1, (2, vec![1, 0])),
            (2, (2, vec![2, 0])),
            (2, (2, vec![1, 1])),
            (2, (2, vec![1, 2])),
            (3, (2, vec![2, 1])),
        ];

        for (want, input) in tests {
            println!("should return {} with input {:?}", want, input);

            let got = solve(input.1);
            assert_eq!(want, got);
        }
    }
}
