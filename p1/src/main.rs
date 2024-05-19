use std::io::{self, BufRead};

fn main() {
    let mut stdin = io::stdin().lock().lines();
    let num: usize = stdin.next().unwrap().unwrap().parse().unwrap();
    let name = stdin.next().unwrap().unwrap();
    println!("{}", num * name.len());
}
