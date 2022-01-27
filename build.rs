use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut data = String::new();

    //load data from file
    File::open("data.txt").unwrap()
        .read_to_string(&mut data)
        .unwrap();

    //split data into lines
    let lines = data.split("\n");
    let words:Vec<String> = lines
        .map(|line| line.split("\t").next().unwrap())
        .filter(|word| word.len() == 5)
        .map(|word| word.to_string())
        .collect();
    
    //write the words to the file "words-build.txt"
    let mut file = io::BufWriter::new(File::create("src/words.txt").unwrap());
    for word in words.clone() {
        file.write_all(word.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    println!("cargo:rerun-if-changed=data.txt");
}