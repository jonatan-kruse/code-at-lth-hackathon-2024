use std::io::{self, BufRead};

fn main() {
    let mut stdin = io::stdin().lock().lines();
    let num = stdin.next().unwrap().unwrap().parse::<usize>().unwrap();
    let name = stdin.next().unwrap().unwrap();
    println!("{}", num * name.len());
}
