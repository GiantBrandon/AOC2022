use regex::Regex;

fn split(line: &str, stack: &mut Vec<Vec<char>>) {
    for group in 0..(line.len() + 1) / 4 {
        let c = line.chars().nth(group * 4 + 1).unwrap();
        if c != ' ' {
            stack[group].insert(0, c);
        }
    }
}

fn handle1(line: &str, stack: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let re: Regex = Regex::new(r"move (\d*) from (\d) to (\d)").unwrap();
    let captures = re.captures(line).unwrap();
    let count = captures[1].parse::<usize>().unwrap();
    let start = captures[2].parse::<usize>().unwrap();
    let end = captures[3].parse::<usize>().unwrap();

    for _ in 0..count {
        let val = stack[start - 1].pop().unwrap();
        stack[end - 1].push(val);
    }
    return stack.to_vec();
}

fn handle2(line: &str, stack: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let re: Regex = Regex::new(r"move (\d*) from (\d) to (\d)").unwrap();
    let captures = re.captures(line).unwrap();
    let count = captures[1].parse::<usize>().unwrap();
    let start = captures[2].parse::<usize>().unwrap();
    let end = captures[3].parse::<usize>().unwrap();

    let range = (stack[start - 1].len() - count)..(stack[start - 1].len());
    let mut last: Vec<char> = stack[start - 1].drain(range).collect();
    stack[end - 1].append(&mut last);
    return stack.to_vec();
}

pub fn solve5(input: String) -> (String, String) {
    let mut instrument_flag = false;

    let mut stack1: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut stack2: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in input.split("\n") {
        if line == "" {
            instrument_flag = true;
        } else if instrument_flag {
            stack1 = handle1(line, &mut stack1);
            stack2 = handle2(line, &mut stack2);
        } else {
            split(line, &mut stack1);
            split(line, &mut stack2);
        }
    }
    let tops1: Vec<String> = stack1.iter().map(|group| group.last().unwrap().to_string()).collect();
    let tops2: Vec<String> = stack2.iter().map(|group| group.last().unwrap().to_string()).collect();
        
    return (tops1.join(""), tops2.join(""));
}