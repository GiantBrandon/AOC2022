fn find_start(input: String, size: usize) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for (index, char) in input.chars().enumerate() {
        stack.insert(0, char);
        if stack.len() == size {
            let mut copy = stack.clone();
            copy.sort();
            copy.dedup();
            if copy.len() == size {
                return index + 1
            }
            stack.pop();
        }
    }
    return 0 as usize
}

pub fn solve6(input: String) -> (String, String) {
    let cloned = input.clone();
    return (find_start(input, 4).to_string(), find_start(cloned, 14).to_string());
}