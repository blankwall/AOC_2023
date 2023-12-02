use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

static COLORS: [&str; 3] = ["blue", "red", "green"];

#[derive(Debug)]
struct Cube {
    color: String,
    quan: u64,
}

struct Lexer<'a> {
    stream: Chars<'a>,
    game_num: Option<u64>,
}

impl<'a> Lexer<'a> {
    fn get_next_token(&mut self) -> Option<Cube> {
        let mut store = String::new();
        let mut game_num: Option<u64> = self.game_num;
        let mut game_digit: Option<u64> = None;

        for i in self.stream.by_ref() {
            if i == ';' {
                return Some(Cube {
                    color: "END".to_string(),
                    quan: 0,
                });
            }

            //PARSE THE GAME NUMBER
            if game_num.is_none() {
                if i.is_ascii_digit() {
                    store.push(i);
                } else if i == ':' {
                    game_num = Some(store.parse::<u64>().unwrap());
                    self.game_num = game_num;
                    return Some(Cube {
                        color: "game".to_string(),
                        quan: game_num.unwrap(),
                    });
                }
                continue;
            }

            //get the quantity
            if game_digit.is_none() {
                if i.is_ascii_digit() {
                    store.push(i);
                } else if i == ' ' && !store.is_empty() {
                    game_digit = Some(store.parse::<u64>().unwrap());
                }
                continue;
            }

            //get the color
            store.push(i);
            for color in COLORS {
                if store.contains(color) {
                    return Some(Cube {
                        color: color.to_string(),
                        quan: game_digit.unwrap(),
                    });
                }
            }
        }
        None
    }
}

fn driver_pt1(m: String) -> u64 {
    let mut l = Lexer {
        stream: m.chars(),
        game_num: None,
    };
    let mut game_num = 0;
    while let Some(v) = l.get_next_token() {
        if v.color == "END" {
            continue;
        } else if v.color == "game" {
            game_num = v.quan;
            // println!("Game: {:?}", game_num);
        } else if v.color == "blue" {
            game_num = if v.quan > 14 { 0 } else { game_num };
        } else if v.color == "green" {
            game_num = if v.quan > 13 { 0 } else { game_num };
        } else if v.color == "red" {
            game_num = if v.quan > 12 { 0 } else { game_num };
        }
    }
    // println!("Game result: {:?}", game_num);
    game_num
}

fn driver_pt2(m: String) -> u64 {
    let mut l = Lexer {
        stream: m.chars(),
        game_num: None,
    };
    let mut max = vec![0, 0, 0];
    while let Some(v) = l.get_next_token() {
        if v.color == "END" || v.color == "game" {
            continue;
        } else if v.color == "blue" {
            max[0] = max[0].max(v.quan);
        } else if v.color == "green" {
            max[1] = max[1].max(v.quan);
        } else if v.color == "red" {
            max[2] = max[2].max(v.quan);
        }
    }
    let mut result: u64 = 1;
    for i in max {
        result *= i
    }
    result
}

fn main() {
    let mut res_pt1: u64 = 0;
    let mut res_pt2: u64 = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for ip in lines.flatten() {
            res_pt1 += driver_pt1(ip.clone());
            res_pt2 += driver_pt2(ip);
        }
    }
    println!("Result: {:?}", res_pt1);
    println!("Result: {:?}", res_pt2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
