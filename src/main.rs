static DATA: &'static str = include_str!("words.txt");

use std::io;

fn main() {
    let lines = DATA.split("\n");
    let words = lines.map(|line| line.split("\t").next().unwrap());
    
    //only keep the 5 letter words
    let mut words = words.filter(|word| word.len() == 5).map(|word| word.to_string()).collect::<Vec<_>>();
    

    let mut ask_about = [true, true, true, true, true];

    loop {
        // first word
        let first = words[0].clone();
        println!("\n\n---------\ntry {}\n---------\n\n", first);
        

        //for every character in the first word
        for (i, c) in first.chars().enumerate() {
            if !ask_about[i] {
                continue;
            }

            println!("What about {}, in pos {}, was it found? (n -> nowhere, e -> elsewhere, y -> here)", c, i + 1);
            
            //read a single char from stdin
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            
            let input = input.trim();
            match input {
                "n" => {
                    words.retain(|word| !word.contains(c));
                },
                "e" => {
                    words.retain(|word| word.contains(c) && word.find(c).unwrap() != i as usize);
                },
                "y" => {
                    words.retain(|word| word.contains(c) && word.find(c).unwrap() == i as usize);
                    ask_about[i] = false;
                },
                _ => {
                    println!("Invalid input");
                }
            }

            println!("{} words left", words.len());
            if words.len() == 1 {
                println!("\n\n------------------\n\"{}\" must be it\n------------------\n\n", words[0]);
                return;
            }
        }

    }
}
