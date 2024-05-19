use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let mut stdin = io::stdin().lock().lines();
    let nums_str = stdin.next().unwrap().unwrap();
    let nums = nums_str.split_once(' ').unwrap();
    let num_corses: usize = nums.0.parse().unwrap();
    let max_credit: usize = nums.1.parse().unwrap();
    let mut corses: Vec<(usize, String)> = Vec::new();
    for _ in 0..num_corses {
        let gears = stdin.next().unwrap().unwrap();
        let (credits, lecturer) = gears.split_once(' ').unwrap();
        let credits: usize = credits.parse().unwrap();
        corses.push((credits, lecturer.to_owned()));
    }
    let max_score = pick_corse(Vec::new(), corses, max_credit);
    println!("{max_score}");
}

fn pick_corse(picked: Vec<(usize, String)>, all: Vec<(usize, String)>, max_score: usize) -> usize {
    let lecturers = picked.iter().map(|i| i.1.to_owned()).collect::<HashSet<_>>();
    let score = picked.iter().map(|i| i.0).sum();
    all.iter().map(|(credits, lecturer)| {
        if credits + score <= max_score && !lecturers.contains(lecturer) {
            let mut new_picked = picked.clone();
            new_picked.push((*credits, lecturer.to_owned()));
            pick_corse(new_picked, all.to_owned(), max_score)
        } else {
            score
        }
    }).max().unwrap()
}