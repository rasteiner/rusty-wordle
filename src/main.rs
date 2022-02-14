static DATA: &'static str = include_str!("words.txt");

use std::{io, collections::HashMap, ops::AddAssign};

#[derive(Copy, Clone, Default)]
struct CharCount {
    capped: bool,
    count: u8,
}

//AddAssign
impl AddAssign<u8> for CharCount {
    fn add_assign(&mut self, rhs: u8) {
        self.count += rhs;
    }
}

#[derive(Copy, Clone)]
struct CharRule {
    sure: Option<char>,
    not: [bool; 26],
}

fn strong(msg: &str) {
    let len = msg.len();
    let dashes = "-".repeat(len);
    println!("\n\n{}\n{}\n{}\n\n", dashes, msg, dashes);
}

fn count_chars(word: &str) -> HashMap<char, u8> {
    let mut counts = HashMap::new();
    for c in word.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn counts_match(word: &HashMap<char, u8>, pattern: &HashMap<char, CharCount>) -> bool {
    for (c, count) in pattern.iter() {
        if word.contains_key(c) && word[c] != count.count {
            if count.capped && word[c] > count.count {
                return false;
            }
            if !count.capped && word[c] < count.count {
                return false;
            }
        }
        if !word.contains_key(c) && count.count > 0 {
            return false;
        }
    }
    true
}

fn get_input() -> Result<[char; 5], u8> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().chars().filter(|c| *c == 'y' || *c == 'n' || *c == 'e').collect::<Vec<char>>();

    if input.len() != 5 {
        return Err(1);
    }

    Ok([input[0], input[1], input[2], input[3], input[4]])
}



fn matches_charrule(c: char, rule: &CharRule) -> bool {
    if rule.sure.is_some() && rule.sure.unwrap() != c {
        return false;
    } 
    
    if rule.not[c as usize - 'a' as usize] {
        return false;
    }

    true
}

fn matches_charrules(word: &str, rules: &[CharRule]) -> bool {

    for i in 0..5 {
        if !matches_charrule(word.chars().nth(i).unwrap(), &rules[i]) {
            return false;
        }
    }

    true
}

fn main() {
    let lines = DATA.trim().split("\n");
    let mut words = lines.map(|line| line.split("\t").next().unwrap()).collect::<Vec<&str>>();
    
    let mut counts:HashMap<char, CharCount> = HashMap::new();

    let mut char_rules:[CharRule; 5] = [CharRule { sure: None, not: [false; 26] }; 5];

    //while there are still words
    while words.len() > 0 {
        let word = words[0];
        strong(word);

        // get results from user, input is a 5 char string of chars y, n or e
        let input = match get_input() {
            Ok(input) => input,
            Err(_err) => {
                println!("Invalid input. Try again.");
                continue;
            },
        };

        let mut guesscounts = HashMap::<char, CharCount>::new();

        for (i, c) in word.chars().enumerate() {
            let res = input[i];
            if res == 'y' || res == 'e' {
                *guesscounts.entry(c).or_insert(Default::default()) += 1;
            } else if res == 'n'{
                if guesscounts.contains_key(&c){
                    guesscounts.get_mut(&c).unwrap().capped = true;
                } else {
                    guesscounts.insert(c, CharCount { capped: true, count: 0 });
                }
            }

            if res == 'y' {
                char_rules[i].sure = Some(c);
            } else if res == 'n' || res == 'e' {
                char_rules[i].not[c as usize - 'a' as usize] = true;
            }
        }

        // copy new guesscounts values to counts
        for (c, count) in guesscounts.iter() {
            *counts.entry(*c).or_insert(Default::default()) = *count;
        }

        words.retain(|word| {
            if !matches_charrules(word, &char_rules) {
                return false;
            }
            if !counts_match(&count_chars(word), &counts) {
                return false;
            }
            true
        });
        
        println!("{} words remaining", words.len());
    }
}