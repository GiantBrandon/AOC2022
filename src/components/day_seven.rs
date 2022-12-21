fn parse(input: &str) -> Vec<i32> {
    let mut directories: Vec<i32> = Vec::new();
    let mut current_dir: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                directories.push(current_dir.pop().unwrap());
            },
            ["$", "cd", _] => {
                current_dir.push(0);
            }
            ["dir", _] => {
            }
            [s, _] if *s != "$" && *s != "dir" => {
                let size = s.parse::<i32>().unwrap();
                current_dir = current_dir.iter().map(|dir| dir + size).collect();
            }
            _ => {},
        }
    }
    while current_dir.len() > 0 {
        directories.push(current_dir.pop().unwrap());
    }
    return directories;
}

pub fn solve7(input: String) -> (String, String) {
    let mut directories = parse(&input);
    let part1: i32 = directories.iter().filter(|&&size| size <= 100000).sum();
    directories.sort_unstable();
    let used_size = directories.last().unwrap();
    let size_needed = 30000000 - (70000000 - used_size);
    let part2 = directories.iter().find(|&size| *size >= size_needed).unwrap();
    return (part1.to_string(), part2.to_string());
}