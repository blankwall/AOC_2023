use std::fs::read_to_string;

fn difference(x: &String, y: &String) -> usize {
    x.chars()
        .zip(y.chars())
        .fold(0, |acc, x| acc + if x.0 == x.1 { 0 } else { 1 })
}

fn is_reflected(board: &Vec<String>, to_match: usize) -> u8 {
    for index in 1..board.len() {
        let before = &board[..index].iter().rev().collect::<Vec<&String>>();
        let after = &board[index..].iter().collect::<Vec<&String>>();

        // println!("Before: {:?}\n After: {:?}", before,after);
        if before
            .iter()
            .zip(after.iter())
            .fold(0, |acc, x| acc + difference(x.0, x.1))
            == to_match
        {
            return index as u8;
        }
    }
    0
}

fn part1(u: usize) {
    let game = read_lines("./my_input.txt");
    let mut new_board: Vec<String> = vec![];
    let mut sum: u64 = 0;
    for line in game {
        if line.len() == 0 {
            let mut res = is_reflected(&new_board, u);
            if res != 0 {
                // println!("Horizontally reflected!");
                sum += res as u64 * 100;
            } else {
                let invert_board: Vec<String> =
                    (0..new_board[0].len()).fold(vec![], |mut vec, index| {
                        vec.push(
                            new_board
                                .iter()
                                .map(|x| x.chars().nth(index))
                                .fold("".to_string(), |x, y| x + &y.unwrap().to_string()),
                        );
                        vec
                    });
                res = is_reflected(&invert_board, u);
                sum += res as u64;
            }
            new_board.clear();
        } else {
            new_board.push(line);
        }
    }
    println!("{:?}", sum);
}

// In part 1, we had to make sure every matched pair were identical.
// In part 2, we need exactly one pair to differ by a single character.

fn main() {
    part1(0); //part1
    part1(1); //part2
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
