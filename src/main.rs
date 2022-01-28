static DATA: &'static str = include_str!("words.txt");

use std::io;

fn strong(msg: &str) {
    let len = msg.len();
    let dashes = "-".repeat(len);
    println!("\n\n{}\n{}\n{}\n\n", dashes, msg, dashes);
}

fn main() {
    let lines = DATA.split("\n");
    let words = lines.map(|line| line.split("\t").next().unwrap());
    
    //only keep the 5 letter words
    let mut words:Vec<String> = words.filter(|word| word.len() == 5).map(|word| word.to_string()).collect();
    

    let mut ask_about = [true, true, true, true, true];

    loop {
        // first word
        let first = words[0].clone();
        strong(format!("try {}", first).as_str());

        //for every character in the first word
        for (i, c) in first.chars().enumerate() {
            if !ask_about[i] {
                continue;
            }
            
            loop {
                println!("What about {}, in pos {}, was it found? (n -> nowhere, e -> elsewhere, y -> here)", c, i + 1);
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                let input = input.trim();
                match input {
                    "n" => {
                        for j in 0..5 {
                            if ask_about[j] {
                                words.retain(|word| word.chars().nth(j).unwrap() != c);
                            }
                        }

                        break
                    },
                    "e" => {
                        words.retain(|word| {

                            for j in 0..5 {
                                if !ask_about[j] {
                                    continue;
                                }
                                
                                let c2 = word.chars().nth(j).unwrap();
                                
                                if j == i {
                                    if c2 == c {
                                        //c should not be found here
                                        return false;
                                    }
                                } else {
                                    if c2 == c {
                                        //c should be found elsewhere
                                        return true;
                                    }
                                }
                            }

                            //if c was not found anywhere...
                            return false
                        });
                        break
                    },
                    "y" => {
                        words.retain(|word| word.chars().nth(i).unwrap() == c );
                        ask_about[i] = false;
                        break
                    },
                    _ => {
                        println!("Invalid input");
                    }
                }
            }

            println!("{} words left", words.len());
            if words.len() == 1 {
                strong(format!("\"{}\" must be it", words[0]).as_str());
                return;
            } else if words.len() == 0 {
                strong("No matching words found");
                return;
            }

            println!("");
        }

    }
}
