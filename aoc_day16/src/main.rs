use std::fs::read_to_string;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Coord {
    fn mv(&mut self) -> bool {
        if self.dir == Dir::Right {
            self.x += 1;
        } else if self.dir == Dir::Left {
            if self.x <= 0 {
                return false;
            }
            self.x -= 1;
        } else if self.dir == Dir::Up {
            if self.y <= 0 {
                return false;
            }
            self.y -= 1;
        } else if self.dir == Dir::Down {
            self.y += 1;
        }
        true
    }
}

fn traverse(
    board: &mut Vec<Vec<char>>,
    mut cur: Coord,
    shadow_board: &mut Vec<Vec<char>>,
) -> Vec<Coord> {

    let mut new_paths: Vec<Coord> = vec![];
    while cur.x < board[0].len() && cur.y < board.len() {
        let cur_item = board[cur.y][cur.x];
        shadow_board[cur.y][cur.x] = 'X';

        println!("{:?} {:?}", cur, cur_item);
        // board[cur.y][cur.x] = 'X';
        if cur_item == '.' {
            let result = cur.mv();
            if !result {
                break;
            }
        } else if cur_item == '|' {
            if cur.dir == Dir::Up || cur.dir == Dir::Down {
                let result = cur.mv();
                if !result {
                    break;
                }
            } else {
                let mut new_coord = cur.clone();
                new_coord.dir = Dir::Up;
                new_paths.push(new_coord);

                let mut new_coord = cur.clone();
                new_coord.dir = Dir::Down;
                new_paths.push(new_coord);
                break;
            }
        } else if cur_item == '-' {
            if cur.dir == Dir::Left || cur.dir == Dir::Right {
                let result = cur.mv();
                if !result {
                    break;
                }
            } else {
                let mut new_coord = cur.clone();
                new_coord.dir = Dir::Left;
                new_paths.push(new_coord);

                let mut new_coord = cur.clone();
                new_coord.dir = Dir::Right;
                new_paths.push(new_coord);

                break;
            }
        } else if cur_item == '\\' {
            if cur.dir == Dir::Right {
                cur.dir = Dir::Down;
            } else if cur.dir == Dir::Left {
                cur.dir = Dir::Up;
            } else if cur.dir == Dir::Up {
                cur.dir = Dir::Left;
            } else if cur.dir == Dir::Down {
                cur.dir = Dir::Right;
            }
            let result = cur.mv();
            if !result {
                break;
            }
        } else if cur_item == '/' {
            if cur.dir == Dir::Right {
                cur.dir = Dir::Up;
            } else if cur.dir == Dir::Left {
                cur.dir = Dir::Down;
            } else if cur.dir == Dir::Up {
                cur.dir = Dir::Right;
            } else if cur.dir == Dir::Down {
                cur.dir = Dir::Left;
            }
            let result = cur.mv();
            if !result {
                break;
            }
        }
    }

    return new_paths;
}
fn part1() {
    // let data = read_lines("./sample.txt");
    let data = read_lines("./question.txt");
    let mut grid: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();
    let mut shadow_grid = grid.clone();
    let start = Coord {
        x: 0,
        y: 0,
        dir: Dir::Right,
    };
    let mut paths = vec![start];
    let mut seen = vec![];
    while let Some(start_pos) = paths.pop() {
        println!("Calling traverse with: {:?}", start_pos);
        seen.push(start_pos);
        let result = traverse(&mut grid, start_pos, &mut shadow_grid);
        println!("Returned new length == {:?}", result.len());
        for i in &result {
            if !seen.contains(i) {
                paths.push(*i)
            }
        }
    }

    let sum: usize = shadow_grid
        .iter()
        .map(|x| x.iter().filter(|y| **y == 'X').count())
        .sum();
    println!("{:?}", sum);
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
