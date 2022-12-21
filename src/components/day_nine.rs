use std::collections::HashSet;

use gloo::console::log;

fn resolve_rope(mut rope: Vec<(i32, i32)>, head: (i32, i32)) -> Vec<(i32, i32)> {
    for index in 0..rope.len() {
        if index == 0{
            rope[index] = head;
        } else {
            let (prev_x, prev_y) = rope[index - 1];
            let (curr_x, curr_y) = rope[index];

            if curr_x == prev_x || curr_y == prev_y {
                if curr_x == prev_x + 2 {
                    rope[index] = (curr_x - 1, curr_y);
                } else if curr_x == prev_x - 2 {
                    rope[index] = (curr_x + 1, curr_y);
                } else if curr_y == prev_y + 2 {
                    rope[index] = (curr_x, curr_y - 1);
                } else if curr_y == prev_y - 2 {
                    rope[index] = (curr_x, curr_y + 1);
                }
            } else {
                if curr_x == prev_x + 2 {
                    rope[index] = (curr_x - 1, if curr_y > prev_y { curr_y - 1 } else { curr_y + 1 });
                } else if curr_x == prev_x - 2 {
                    rope[index] = (curr_x + 1, if curr_y > prev_y { curr_y - 1 } else { curr_y + 1 });
                } else if curr_y == prev_y + 2 {
                    rope[index] = (if curr_x > prev_x { curr_x - 1 } else { curr_x + 1 }, curr_y - 1);
                } else if curr_y == prev_y - 2 {
                    rope[index] = (if curr_x > prev_x { curr_x - 1 } else { curr_x + 1 }, curr_y + 1);
                }
            }
        }
    }
    return rope
}

fn solve(input: String, size: usize) -> i32 {
    let mut coordinates: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); size];

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["R", amount] => {
                for _ in 0..amount.parse::<i32>().unwrap() {
                    let head = (rope[0].0 + 1, rope[0].1);
                    rope = resolve_rope(rope, head);
                    coordinates.insert(*rope.last().unwrap());
                }
            }
            ["L", amount] => {
                for _ in 0..amount.parse::<i32>().unwrap() {
                    let head = (rope[0].0 - 1, rope[0].1);
                    rope = resolve_rope(rope, head);
                    coordinates.insert(*rope.last().unwrap());
                }
            }
            ["U", amount] => {
                for _ in 0..amount.parse::<i32>().unwrap() {
                    let head = (rope[0].0, rope[0].1 + 1);
                    rope = resolve_rope(rope, head);
                    coordinates.insert(*rope.last().unwrap());
                }
            }
            ["D", amount] => {
                for _ in 0..amount.parse::<i32>().unwrap() {
                    let head = (rope[0].0, rope[0].1 - 1);
                    rope = resolve_rope(rope, head);
                    coordinates.insert(*rope.last().unwrap());
                }
            }
            _ => {}
        };
    }
    return coordinates.len() as i32
}

//2482
pub fn solve9(input: String) -> (String, String) {
    return (solve(input.clone(), 2).to_string(), solve(input.clone(), 10).to_string())
}