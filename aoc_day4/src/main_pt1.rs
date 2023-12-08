use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_card(card: &String) -> usize {
    let vals = card.split(":").collect::<Vec<&str>>()[1].split("|");
    let mut scores = HashMap::new();
    let mut part2 = false;
    let mut result: usize = 0;

    for val in vals {
        for entry in val.split_whitespace() {
            if !part2 {
                scores.insert(entry, 1);
            } else {
                result = if scores.get(entry).copied().unwrap_or(0) == 0 {
                    result
                } else {
                    if result == 0 {
                        1
                    } else {
                        result * 2
                    }
                };
            }
        }
        part2 = true;
    }
    result
}

fn driver<T>(lines: io::Lines<T>)
where
    T: BufRead,
{
    let mut i = 0;
    for ip in lines.flatten() {
        // println!("{}", ip);
        i += parse_card(&ip);
    }
    println!("{}", i);

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
