use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_card(card: &str) -> usize {
    let cards = card.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .collect::<Vec<&str>>();
    let mut scores = HashMap::new();
    let mut result = 0;

    if let [win_numbers, my_numbers] = &cards[..] {
        win_numbers.split_whitespace().for_each(|x| {
            scores.insert(x, 1);
        });

        my_numbers.split_whitespace().for_each(|x| {
            result += scores.get(x).copied().unwrap_or(0);
        });
    }
    result
}

fn driver<T>(lines: io::Lines<T>)
where
    T: BufRead,
{
    let mut card = 0;
    let mut iters = vec![1; 200];
    iters[0] = 0;

    for ip in lines.flatten() {
        card += 1;
        let i = parse_card(&ip);
        for _ in 0..iters[card] {
            for g in 1..i + 1 {
                iters[card + g] += 1;
            }
        }
    }
    let result = &iters[..card + 1].iter().fold(0, |mut sum, &x| {
        sum += x;
        sum
    });
    println!("{:?}", result);
}

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        driver(lines)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
