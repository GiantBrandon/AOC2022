use std::{collections::BinaryHeap};

pub fn solve1(input: String) -> (String, String) {
    let mut current_count = 0;
    let mut max = 0;
            
    let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
    for line in input.split("\n") {
        if line == "" {
            let mut stringify = "".to_owned();
            for item in &heap {
                stringify = format!("{}, {}", stringify, *item)
            }
            if current_count > max {
                max = current_count;
            }
            heap.push(-current_count);
            if heap.len() > 3 {
                heap.pop();
            }
            current_count = 0;
        } else {
            current_count = current_count + line.parse::<i32>().unwrap()
        }
    };
    if current_count > max {
        heap.push(-current_count);
            if heap.capacity() > 3 {
                heap.pop();
            }
        max = current_count;
    }
    let max_group: i32 = heap.into_iter().map(|item| -item).sum();
    return (max.to_string(), max_group.to_string());
}