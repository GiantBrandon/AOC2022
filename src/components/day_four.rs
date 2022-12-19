pub fn solve4(input: String) -> (String, String) {
    let mut count_1: i32 = 0;
    let mut count_2: i32 = 0;

    for line in input.split("\n") {
        let mut splits = line.split(&['-', ',']);
        let start1 = splits.next().unwrap().parse::<i32>().unwrap();
        let end1 = splits.next().unwrap().parse::<i32>().unwrap();
        let start2 = splits.next().unwrap().parse::<i32>().unwrap();
        let end2 = splits.next().unwrap().parse::<i32>().unwrap();
        if start1 <= start2 && end1 >= end2 {
            count_1 = count_1 + 1;
        } else if start2 <= start1 && end2 >= end1 {
            count_1 = count_1 + 1;
        }

        if start1 <= start2 && end1 >= start2 {
            count_2 = count_2 + 1;
        } else if start2 <= start1 && end2 >= start1 {
            count_2 = count_2 + 1;
        }
    }
        
    return (count_1.to_string(), count_2.to_string());
}