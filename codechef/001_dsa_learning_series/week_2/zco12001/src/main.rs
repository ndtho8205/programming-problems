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

fn solve(sequence: Vec<usize>) -> Vec<usize> {
    let (mut max_depth, mut pos_max_depth) = (0, 0);
    let mut current_depth = 0;

    let (mut longest_seg_length, mut pos_longest_seg) = (0, 0);
    let (mut count_bracket, mut pos_current_seg) = (0, 0);

    for (idx, bracket) in sequence.iter().enumerate() {
        if *bracket == 1 {
            // opening bracket - (
            current_depth += 1;
        } else {
            // closing bracket - )
            current_depth -= 1;
        }

        // subtask 1
        if current_depth > max_depth {
            max_depth = current_depth;
            pos_max_depth = idx + 1;
        }

        // subtask 2
        count_bracket += 1;
        if *bracket == 1 && current_depth == 1 {
            count_bracket = 1;
            pos_current_seg = idx + 1;
        } else if *bracket == 2 && current_depth == 0 {
            if count_bracket > longest_seg_length {
                longest_seg_length = count_bracket;
                pos_longest_seg = pos_current_seg;
            }
        }
    }

    vec![
        max_depth,
        pos_max_depth,
        longest_seg_length,
        pos_longest_seg,
    ]
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let sequence_length: usize = read(&mut reader);
    let sequence = read_vec(&mut reader);

    assert_eq!(sequence_length, sequence.len());

    for result in solve(sequence) {
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tests = vec![
            (
                vec![2, 4, 6, 9],
                (
                    20,
                    vec![1, 2, 1, 1, 2, 2, 1, 2, 1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2],
                ),
            ),
            (
                vec![5, 9, 14, 1],
                (14, vec![1, 1, 2, 1, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2]),
            ),
            //
            (vec![2, 2, 4, 1], (8, vec![1, 1, 2, 2, 1, 2, 1, 2])),
        ];

        for (want, input) in tests {
            println!("should return {:?} with input {:?}", want, input);

            let got = solve(input.1);
            assert_eq!(want, got);
        }
    }
}
