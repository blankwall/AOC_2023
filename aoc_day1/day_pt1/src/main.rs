use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::BufReader;
use std::io::Lines;

fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_first(s: &String) -> char{
    for i in s.chars() {
        if i.is_digit(10){
            return i;
        }
    }
    ' '
}

fn get_last(s: &String) -> char{
    for i in (0..s.len()).rev(){
        // println!("{:?}", &s[i..s.len()]);
        // let x = &s[i..i+1];
        let mc = s.chars().nth(i).unwrap();
        if mc.is_digit(10) {
            return mc;
        }
    }
    ' '
}

fn driver(x: Lines<BufReader<File>>) -> u64{
    let mut sum:u64 = 0;

    for entry in x {
        let entry = entry.unwrap();
        let mut first:u64 = get_first(&entry).to_digit(10).unwrap() as u64;
        first *= 10;
        let last:u64 = get_last(&entry).to_digit(10).unwrap() as u64;
        sum += first + last;
    }

    sum
}

fn main() {
    let vec = lines_from_file("src/input.txt").unwrap();
    // println!("{:?}", vec);
    println!("{:?}",driver(vec));
}
