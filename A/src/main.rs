use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let mut stdin = io::stdin().lock().lines();
    let num: usize = stdin.next().unwrap().unwrap().parse().unwrap();
    let mut known_words = HashSet::new();
    for _ in 0..num {
        let new_word = stdin.next().unwrap().unwrap().to_lowercase();
        known_words.insert(new_word);
    }
    let test_words = stdin.next().unwrap().unwrap().to_lowercase();
    if test_words.split_ascii_whitespace().all(|word| known_words.contains(word)) {
        println!("Hi, how do I look today?");
    } else {
        println!("Thore has left the chat");
    }
}
