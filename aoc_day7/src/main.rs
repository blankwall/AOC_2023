use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::fs::read_to_string;


static RANKING:[char;13] = ['2','3','4','5','6','7','8','9','T', 'J','Q','K','A'];

#[derive(PartialEq)]
#[derive(Debug)]
enum Hands {
    HighCard(char,char,char,char, char),
    OnePair(char,char,char,char),
    TwoPair(char,char, char),
    ThreeofaKind(char,char,char),
    FullHouse(char,char),
    FourofaKind(char,char),
    FiveofaKind(char),
}

impl Hands {
    fn printme(&self){
        println!("I am a hand");
    }
}

#[derive(Debug)]
struct MyHand{
    hand: String,
    id: usize,
    rank: usize,
    score: Hands,
}

#[derive(Debug)]
struct Entry {
    hand: MyHand,
    score: usize,
}

impl MyHand {
    fn new(s: String) -> Self{
        Self {
            hand: s,
            rank: 0,
            id: 0,
            // score: MyHand::score_hand(s),
            score: Hands::HighCard(' ',' ',' ',' ', ' '),
        }
    }

    fn score_hand(&mut self) -> Hands {
        let s = &self.hand;
        let letter_counts: HashMap<char, i32> =
            s
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        let mut store = 0;
        let mut chold: char = ' ';
        let js = letter_counts.get(&'J').unwrap_or(&0);
        // println!("{:?}", js);
        // letter_counts.remove(&'j');

        let hold = letter_counts.iter().fold(Vec::new(), |mut veccy, v| {
            if v.0 != &'J'{
                store = store.max(*v.1);
                
                if v.1 >= &3 {
                    chold = *v.0;
                } else {
                    veccy.push(*v.0);
                }
            }
            veccy
        });

        store += js;
        // println!("Store: {:?}", store);
        if store == 5 {
            self.id = 6;
            return Hands::FiveofaKind(s.chars().nth(0).unwrap());
        } else if store == 4 {
            self.id = 5;
            return Hands::FourofaKind(chold, hold[0]);
        } else if store == 3 {
            if hold.len() == 1 || hold.len() == 2 && js == &1{
                self.id = 4;
                return Hands::FullHouse(chold, hold[0]);
            }
            self.id = 3;
            return Hands::ThreeofaKind(chold, hold[0], hold[1]);
        } else {
            // two pair, one pair and high card 
            if hold.len() == 3 || hold.len() == 4 {
                let v = hold.iter().fold(Vec::new(), |mut veccy, v| {
                    if letter_counts.get(v).unwrap() == &2 {
                        veccy.insert(0,*v);
                    } else {
                        veccy.push(*v);
                    }
                    veccy
                });
                if hold.len() == 3 {
                    self.id = 2;
                    return Hands::TwoPair(v[0],v[1],v[2]);
                } else {
                    self.id = 1;
                    return Hands::OnePair(v[0],v[1],v[2], v[3]);
                }
            } 
        }
        self.id = 0;
        // println!("{:?} {:?}", hold, s);
        Hands::HighCard(hold[0],hold[1],hold[2],hold[3],hold[4])
    }
}

fn convert(char1: char) -> u32{
        if char1 == 'T'{
            return 10;
        } else if char1 == 'J'{
            return 0;
        } else if char1 == 'Q'{
            return 12;
        } else if char1 == 'K'{
            return 13;
        } else if char1 == 'A'{
            return 14;
        } 
        0
}

fn where_to_insert(my_hand: &Entry, r: &Vec<Entry>) -> usize{
    let comparator = &my_hand.hand.hand;
    let mut index:usize = 0;
    let ops = ['T','J','Q','K','A'];

    for other in r{
        // println!("Me: {:?} Other: {:?}", comparator, other.hand.hand);
        let mut other_chars = other.hand.hand.chars();
        let mut my_chars = comparator.chars();

        while let Some(char1) = my_chars.next(){
            let char2 = other_chars.next().unwrap();
            let mut val1 = 0;
            if ops.contains(&char1){
                val1 = convert(char1);
            } else {
                val1 = char1.to_digit(10).unwrap()
            }

            let mut val2 = 0;
            if ops.contains(&char2){
                val2 = convert(char2);
            } else {
                val2 = char2.to_digit(10).unwrap()
            }

            if val2 > val1 {
                return index;
            } else if val1 > val2 {
                break;
            }

            // println!("{:?} {:?}", val1, val2);

        }
        index += 1;
    }
    // println!("EE: {:?}", comparator);
    index
}

fn driver(){    
    let mut results: Vec<Vec<Entry>> = Vec::new();
    for i in 0..7{
        let v: Vec<Entry> = Vec::new();
        results.push(v);
    }
 

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.flatten() {
            let entry = line.split_whitespace().collect::<Vec<&str>>();
            let score = entry[1].parse::<usize>().unwrap();
            let mut hand = MyHand::new(entry[0].to_string());
            hand.score = hand.score_hand();
            let new_entry = Entry{hand, score};

            let insert_location = where_to_insert(&new_entry, &results[new_entry.hand.id]);
            // println!("Should insert at: {:?}", insert_location);
            results[new_entry.hand.id].insert(insert_location,new_entry);
        }
    }

    let mut rank = 1;
    let mut result:usize = 0;
    for i in results {
        for j in &i{
            println!("{} {:?}", rank,j);
            result += rank*j.score;
            rank += 1;

        }
    }
    println!("{:?}", result);



}

fn main() {
    // let y = "32T3K".to_string();
    // let y = "325KK".to_string();
    // let f = MyHand::score_hand(&y);
    // println!("{:?}", f);
    driver();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g1() {
        let y = "325KK".to_string();
        let enum_comp = Hands::OnePair('K','3','2','5');
        assert!(std::mem::discriminant(&MyHand::score_hand(&y)) == std::mem::discriminant(&enum_comp));
    }

    #[test]
    fn g2() {
        let y = "32KKK".to_string();
        let enum_comp = Hands::ThreeofaKind('K','3','2');
        assert!(std::mem::discriminant(&MyHand::score_hand(&y)) == std::mem::discriminant(&enum_comp));
    }

    #[test]
    fn g3() {
        let y = "3KKKK".to_string();
        let enum_comp = Hands::FourofaKind('K','3');
        assert!(MyHand::score_hand(&y) == enum_comp);
    }

    #[test]
    fn g4() {
        let y = "KKKKK".to_string();
        let enum_comp = Hands::FiveofaKind('K');
        assert!(MyHand::score_hand(&y) == enum_comp);
    }

    #[test]
    fn g5() {
        let y = "KKKQQ".to_string();
        let enum_comp = Hands::FullHouse('K', 'Q');
        assert!(MyHand::score_hand(&y) == enum_comp);
    }

    #[test]
    fn g6() {
        let y = "KK4QQ".to_string();
        let enum_comp = Hands::TwoPair('K', 'Q', '4');
        assert!(std::mem::discriminant(&MyHand::score_hand(&y)) == std::mem::discriminant(&enum_comp));
    }

    #[test]
    fn g7() {
        let y = "KQJT8".to_string();
        let enum_comp = Hands::HighCard('K', 'Q', '4', '5','6');
        assert!(std::mem::discriminant(&MyHand::score_hand(&y)) == std::mem::discriminant(&enum_comp));
    }
}
