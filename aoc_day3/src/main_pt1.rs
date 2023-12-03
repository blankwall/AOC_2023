use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

#[derive(Debug)]
struct Finding {
    begin: usize,
    end: usize,
    val: usize,
    row: usize,
}

fn parse_line(row: &String, row_val: usize) -> Vec<Finding> {
    let mut store = String::new();
    let mut result: Vec<Finding> = Vec::new();

    for (index,v) in row.chars().enumerate() {
        if v.is_ascii_digit(){
            store.push(v);
            continue
        } else if !store.is_empty(){
            let val = store.parse::<usize>().unwrap();
            let begin = index - store.len();
            let end = index-1;
            let row = row_val;

            result.push(Finding{val,begin,end,row});
            store.clear();
        }
 
    }

    if !store.is_empty(){
        let val = store.parse::<usize>().unwrap();
        let begin = row.len() - store.len();
        let end = row.len();
        let row = row_val;

        result.push(Finding{val,begin,end,row});    
    }

    // println!("{:?}",result );

    result

}

fn is_valid(find: &Finding, board: &Vec<String>) -> bool {

    let top_row = if find.row > 0 { find.row -1 } else { 0 };
    let bot_row = if find.row < board.len()-1 { find.row + 1} else { find.row };
    let begin = if find.begin > 0 { find.begin - 1} else { find.begin };
    let mut end = if find.end < board[0].len()-1  { find.end + 1} else { find.end-1};
    end += 1;
    
    let top = &board[top_row];  
    let bottom = &board[bot_row]; 
    let cur = &board[find.row]; 
    for i in begin..end{
        if !top.chars().nth(i).unwrap().is_ascii_digit() && top.chars().nth(i) != Some('.')  {
            return true
        }

        if !bottom.chars().nth(i).unwrap().is_ascii_digit() && bottom.chars().nth(i) != Some('.')  {
            return true
        }

        if !cur.chars().nth(i).unwrap().is_ascii_digit() && cur.chars().nth(i) != Some('.') {
            return true
        }
   
    }
    false
}

fn part_1(board: &Vec<String>){
    let mut result: usize = 0;

    for (index,row) in board.iter().enumerate(){
        for finding in parse_line(row,index){
            if is_valid(&finding, board) {
                result += finding.val;
            } else {
                println!("Not valid: {:?}", finding);
            }
        }
    }
    println!("{:?}", result);

}

fn main() {
    let mut res_pt1: u64 = 0;
    let mut res_pt2: u64 = 0;

    let mut board: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./src/input.txt") {
        for ip in lines.flatten() {
            board.push(ip.clone());
        }
    }

    part_1(&board);
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
