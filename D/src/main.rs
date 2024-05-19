use std::io::{self, BufRead};

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
    let max_score = pick_corse(0, corses, max_credit);
    println!("{max_score}");
}

fn pick_corse(score: usize, all: Vec<(usize, String)>, max_score: usize) -> usize {
    all.iter()
        .map(|(credits, lecturer)| {
            if credits + score <= max_score {
                let new_all = all
                    .clone()
                    .into_iter()
                    .filter(|(_, lect)| lect != lecturer)
                    .collect();
                pick_corse(credits + score, new_all, max_score)
            } else {
                score
            }
        })
        .max()
        .unwrap_or(score)
}
