use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::BufReader;
use std::io::Lines;

static NUMBERS: &'static [&str;10] = &[ 
"X", 
"one", 
"two",
"three",
"four",
"five",
"six",
"seven",
"eight",
"nine"
];

fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_remainder(remainder: &str) -> Option<u64> {
    for (ind,num) in NUMBERS.iter().enumerate(){
        if remainder.contains(num){
            return Some(ind as u64);
        }
    }
    None
}

fn get_first(s: &String) -> u64{
    for (index,i) in s.chars().enumerate() {
        if i.is_digit(10){
            return i.to_digit(10).unwrap() as u64;
        }

        let remainder = &s[0..index+1];
        if let Some(val) = check_remainder(remainder) {
            return val;
        }
    }
    0 as u64
}

fn get_last(s: &String) -> u64{
    for i in (0..s.len()).rev(){
        let mc = s.chars().nth(i).unwrap();
        if mc.is_digit(10) {
            return mc.to_digit(10).unwrap() as u64;
        }

        let remainder = &s[i..s.len()];
        if let Some(val) = check_remainder(remainder) {
            return val;
        }
    }
    0 as u64
}

fn driver(x: Lines<BufReader<File>>) -> u64{
    let mut sum:u64 = 0;

    for entry in x {
        let entry = entry.unwrap();
        let mut first:u64 = get_first(&entry);
        first *= 10;
        let last:u64 = get_last(&entry);
        sum += first + last;
    }

    sum
}

fn main() {
    let vec = lines_from_file("src/input.txt").unwrap();
    // println!("{:?}", vec);
    println!("{:?}", driver(vec));
    // println!("{:?}",get_first(&"five4onetwo3".to_string()));
    // println!("{:?}",get_last(&"five4onetiwo".to_string()));
}
