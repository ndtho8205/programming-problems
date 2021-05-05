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

#[derive(Debug)]
enum Turn {
    A,
    B,
}

fn solve(num_shots_per_team: usize, all_shots: String) -> usize {
    let (mut count_a_wins, mut count_b_wins) = (0, 0);
    let (mut count_a_remaining_shots, mut count_b_remaining_shots) =
        (num_shots_per_team, num_shots_per_team);

    let mut turn = Turn::A;
    let mut count_shots = 0;

    for shot_result_ in all_shots.chars() {
        count_shots += 1;

        let shot_result = shot_result_
            .to_digit(10)
            .expect("Failed to parse shot results") as usize;

        match turn {
            Turn::A => {
                count_a_wins += shot_result;
                count_a_remaining_shots -= 1;
                turn = Turn::B;
            }
            Turn::B => {
                count_b_wins += shot_result;
                count_b_remaining_shots -= 1;
                turn = Turn::A;
            }
        }

        if (count_a_wins > count_b_wins + count_b_remaining_shots)
            || (count_b_wins > count_a_wins + count_a_remaining_shots)
        {
            return count_shots;
        }
    }

    count_shots
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let num_test_cases: usize = read(&mut reader);
    for _ in 0..num_test_cases {
        let num_shots_per_team: usize = read(&mut reader);
        let all_shots = try_read(&mut reader);

        let result = solve(num_shots_per_team, all_shots);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tests = vec![
            (4, (3, "101011")),
            (6, (3, "100001")),
            //
            (0, (0, "")),
            //
            (2, (1, "00")),
            (2, (1, "01")),
            (2, (1, "10")),
            (2, (1, "11")),
            //
            (4, (2, "0000")),
            (4, (2, "0010")),
            (4, (2, "0011")),
            (3, (2, "0100")),
            (3, (2, "0101")),
            (4, (2, "0110")),
            (4, (2, "0111")),
            (4, (2, "1000")),
            (4, (2, "1001")),
            (3, (2, "1010")),
            (3, (2, "1011")),
            (4, (2, "1100")),
            (4, (2, "1101")),
            (4, (2, "1110")),
            (4, (2, "1111")),
        ];

        for (want, input) in tests {
            println!("should return {} with input {:?}", want, input);

            let got = solve(input.0, input.1.to_string());
            assert_eq!(want, got);
        }
    }
}
