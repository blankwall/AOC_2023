use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Chars;

fn tilt_north(mut board: &mut Vec<Vec<char>>) {
    let mut did_move = 1;

    while did_move > 0 {
        did_move = 0;
        for row in 1..board.len() {
            let next_row = row - 1;
            for (index, item) in board[row].clone().into_iter().enumerate() {
                if item == 'O' {
                    //Moveable rock
                    if board[next_row][index] == '.' {
                        board[next_row][index] = 'O';
                        board[row][index] = '.';
                        did_move += 1;
                    }
                }
            }
        }
    }
}

fn tilt_south(mut board: &mut Vec<Vec<char>>) {
    let mut did_move = 1;

    while did_move > 0 {
        did_move = 0;
        for row in (0..board.len() - 1).rev() {
            let next_row = row + 1;
            for (index, item) in board[row].clone().into_iter().enumerate() {
                if item == 'O' {
                    //Moveable rock
                    if board[next_row][index] == '.' {
                        board[next_row][index] = 'O';
                        board[row][index] = '.';
                        did_move += 1;
                    }
                }
            }
        }
    }
}

fn tilt_east(mut board: &mut Vec<Vec<char>>) {
    let mut did_move = 1;

    while did_move > 0 {
        did_move = 0;
        for column in 0..board[0].len() - 1 {
            let next_col = column + 1;
            for row in 0..board.len() {
                if board[row][column] == 'O' {
                    if board[row][next_col] == '.' {
                        board[row][next_col] = 'O';
                        board[row][column] = '.';
                        did_move += 1;
                    }
                }
            }
        }
    }
}

fn tilt_west(mut board: &mut Vec<Vec<char>>) {
    let mut did_move = 1;

    while did_move > 0 {
        did_move = 0;
        for column in (1..board[0].len()).rev() {
            let next_col = column - 1;
            for row in 0..board.len() {
                if board[row][column] == 'O' {
                    if board[row][next_col] == '.' {
                        board[row][next_col] = 'O';
                        board[row][column] = '.';
                        did_move += 1;
                    }
                }
            }
        }
    }
}

fn score_board(board: &Vec<Vec<char>>) -> usize {
    (0..board.len()).fold(0, |a, i| {
        a + board[i].iter().filter(|x| **x == 'O').count() * (board.len() - i)
    })
}
fn part1() {
    // let data = read_lines("./sample.txt");
    let data = read_lines("./my_input.txt");
    let mut board = data.iter().fold(vec![], |mut acc: Vec<Vec<char>>, x| {
        acc.push(x.chars().collect());
        acc
    });

    tilt_north(&mut board);
    let result = score_board(&board);
    println!("Part 1: {:?}", result);

    let mut board = data.iter().fold(vec![], |mut acc: Vec<Vec<char>>, x| {
        acc.push(x.chars().collect());
        acc
    });

    let mut scores = HashMap::new();
    for i in 0..10000 {
        tilt_north(&mut board);
        tilt_west(&mut board);
        tilt_south(&mut board);
        tilt_east(&mut board);
        let result = score_board(&board);
        let old = scores.entry(result).or_insert(0);
        *old += 1;
    }

    let mut v = vec![];
    let mut maxxy = 0;
    for (key, value) in &scores {
        println!("{key}: {value}");
        if value > &maxxy {
            maxxy = value - 10;
            v.push(key);
        }
    }
    // v.sort();
    // for i in v{
    //     println!("{:?}", i);
    // }
    let result = v[v.len() - 2];
    println!("Part 2: {:?}", result);
}

fn main() {
    part1();
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
