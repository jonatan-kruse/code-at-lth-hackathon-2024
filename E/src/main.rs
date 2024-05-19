use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead},
    rc::Rc,
};

fn main() {
    let mut stdin = io::stdin().lock().lines();
    let line1 = stdin.next().unwrap().unwrap();
    let binding = line1
        .split_ascii_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let [h, w, robots] = binding.as_slice() else {
        panic!()
    };
    if *robots != 1 {
        panic!()
    }
    let mut grid = Vec::new();
    for _ in 0..*h {
        let line = stdin.next().unwrap().unwrap();
        let chars = line.chars().collect::<Vec<_>>();
        grid.push(chars);
    }
    let smallest_distances: Rc<RefCell<HashMap<(usize, usize), usize>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let mut robot_coord = (0, 0);
    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            if c == &'R' {
                robot_coord = (j, i);
                smallest_distances.clone().borrow_mut().insert((j, i), 0);
            }
        })
    });

    let mut positions = vec![robot_coord];
    let mut distance = 0;

    while !positions.is_empty() {
        let bor_distance = distance;
        let temp_pos = positions
            .iter()
            .map(|p| {
                traverse_once(
                    *p,
                    bor_distance,
                    smallest_distances.clone(),
                    &grid,
                    w - 1,
                    h - 1,
                )
            })
            .collect::<Vec<_>>();
        if temp_pos.iter().any(|i| i.is_none()) {
            break;
        }
        distance += 1;

        let new_positions = temp_pos.into_iter().flatten().flatten().collect::<Vec<_>>();

        positions = new_positions
    }
    println!("{distance}")
}

fn traverse_once(
    (x, y): (usize, usize),
    distance: usize,
    map: Rc<RefCell<HashMap<(usize, usize), usize>>>,
    grid: &[Vec<char>],
    max_x: usize,
    max_y: usize,
) -> Option<Vec<(usize, usize)>> {
    if grid[y][x] == 'P' {
        map.clone().borrow_mut().insert((x, y), distance);
        return None;
    }
    let mut possible_directions = Vec::new();
    if x > 0 && grid[y][x - 1] != '#' && !map.clone().borrow().contains_key(&(x - 1, y)) {
        map.clone().borrow_mut().insert((x - 1, y), distance + 1);
        possible_directions.push((x - 1, y));
    }
    if x < max_x && grid[y][x + 1] != '#' && !map.clone().borrow().contains_key(&(x + 1, y)) {
        map.clone().borrow_mut().insert((x + 1, y), distance + 1);
        possible_directions.push((x + 1, y));
    }
    if y > 0 && grid[y - 1][x] != '#' && !map.clone().borrow().contains_key(&(x, y - 1)) {
        map.clone().borrow_mut().insert((x, y - 1), distance + 1);
        possible_directions.push((x, &y - 1));
    }

    if y < max_y && grid[y + 1][x] != '#' && !map.clone().borrow().contains_key(&(x, y + 1)) {
        map.clone().borrow_mut().insert((x, y + 1), distance + 1);
        possible_directions.push((x, &y + 1));
    }

    Some(possible_directions)
}
