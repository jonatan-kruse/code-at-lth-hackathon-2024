use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut stdin = io::stdin().lock().lines();
    let nums_str = stdin.next().unwrap().unwrap();
    let nums = nums_str.split_once(' ').unwrap();
    let _n: usize = nums.0.parse().unwrap();
    let m: usize = nums.1.parse().unwrap();
    let mut spinning_directions: HashMap<usize, bool> = HashMap::new();
    for _ in 0..m {
        let gears = stdin.next().unwrap().unwrap();
        let (g1, g2) = gears.split_once(' ').unwrap();
        let g1: usize = g1.parse().unwrap();
        let g2: usize = g2.parse().unwrap();
        match (spinning_directions.get(&g1), spinning_directions.get(&g2)) {
            (Some(d1), Some(d2)) => {
                if d1 == d2 {
                    println!("no way");
                    return;
                }
            }
            (Some(d1), None) => {
                spinning_directions.insert(g2, !d1);
            }
            (None, Some(d2)) => {
                spinning_directions.insert(g1, !d2);
            }
            (None, None) => {
                spinning_directions.insert(g1, true);
                spinning_directions.insert(g2, false);
            }
        }
    }

    println!("attend here");
}
