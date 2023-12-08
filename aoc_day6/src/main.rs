use std::fs::read_to_string;
use std::str::FromStr;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn ways_to_win(time: usize, distance: usize) -> usize {
    let mut wins: usize = 0;
    for i in 1..time + 1 {
        if (time - i) * i > distance {
            wins += 1
        }
    }
    wins
}

fn driver(s: Vec<String>) {
    let time = s[0].split_whitespace().collect::<Vec<_>>();
    let distance = s[1].split_whitespace().collect::<Vec<_>>();
    let mut result: Vec<usize> = Vec::new();

    for i in 1..time.len() {
        let t = usize::from_str(time[i]).unwrap();
        let d = usize::from_str(distance[i]).unwrap();
        result.push(ways_to_win(t, d));
    }

    // let res = result.iter().fold(1, |sum, val| sum * val);
    println!("{:?}", result.iter().product::<usize>());
}

fn driver2(s: Vec<String>) {
    let time = s[0].split_whitespace().collect::<Vec<_>>()[1..]
        .join("")
        .parse::<usize>()
        .unwrap();

    let distance = s[1].split_whitespace().collect::<Vec<_>>()[1..]
        .join("")
        .parse::<usize>()
        .unwrap();

    println!("{:?}", ways_to_win(time, distance));
}

fn main() {
    let strings = read_lines("./src/input.txt");
    driver(strings.clone());
    driver2(strings);
}
