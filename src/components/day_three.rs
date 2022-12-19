pub fn solve3(input: String) -> (String, String) {
    let mut count_1: i32 = 0;
    let mut count_2: i32 = 0;

    let mut group: Vec<&str> = Vec::new();
    for line in input.split("\n") {
        let (first, second) = line.split_at(line.len() / 2);
        for char in first.chars() {
            if second.contains(char) {
                if char.is_uppercase() {
                    count_1 = count_1 + char as i32 - 38;
                } else {
                    count_1 = count_1 + char as i32 - 96
                }
            }
        }

        //part 2
        group.push(line);
        if group.len() == 3 {
            let first_group = group.pop().unwrap();
            let second_group = group.pop().unwrap();
            let third_group = group.pop().unwrap();
            let found = first_group.chars().find(|char| second_group.contains(*char) && third_group.contains(*char)).unwrap();
            if found.is_uppercase() {
                count_2 = count_2 + found as i32 - 38;
            } else {
                count_2 = count_2 + found as i32 - 96
            }
            group.clear()
        }
    }
    return (count_1.to_string(), count_2.to_string());
}