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

#[derive(Debug)]
struct StreetFood {
    num_stores: usize,
    num_people: usize,
    price: usize,
}

impl StreetFood {
    fn new(values: Vec<usize>) -> StreetFood {
        StreetFood {
            num_stores: values[0],
            num_people: values[1],
            price: values[2],
        }
    }
}

fn solve(all_food_types: Vec<StreetFood>) -> usize {
    let mut max_daily_profit = 0;

    for food in all_food_types {
        let daily_profit = food.num_people / (food.num_stores + 1) * food.price;

        if daily_profit > max_daily_profit {
            max_daily_profit = daily_profit;
        }
    }

    max_daily_profit
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let num_test_cases: usize = read(&mut reader);
    for _ in 0..num_test_cases {
        let num_food_types: usize = read(&mut reader);

        let mut all_food_types: Vec<StreetFood> = Vec::with_capacity(num_food_types);

        for _ in 0..num_food_types {
            let values: Vec<usize> = read_vec(&mut reader);
            all_food_types.push(StreetFood::new(values))
        }

        let max_daily_profit = solve(all_food_types);
        println!("{}", max_daily_profit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tests = vec![
            (
                12,
                vec![
                    StreetFood::new(vec![4, 6, 8]),
                    StreetFood::new(vec![2, 6, 6]),
                    StreetFood::new(vec![1, 4, 3]),
                ],
            ),
            (0, vec![StreetFood::new(vec![7, 7, 4])]),
            (
                1,
                vec![
                    StreetFood::new(vec![0, 1, 1]),
                    StreetFood::new(vec![3, 4, 1]),
                ],
            ),
        ];

        for (want, input) in tests {
            println!("should return {} with input {:?}", want, input);

            let max_daily_profit = solve(input);
            assert_eq!(want, max_daily_profit);
        }
    }
}
