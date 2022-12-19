pub fn solve2(input: String) -> (String, String) {
    let mut count_1 = 0;
    let mut count_2 = 0;

    for line in input.split("\n") {
        let mut splits = line.split(" ");
        let theirs = splits.next().unwrap();
        let yours = splits.next().unwrap();
        match yours {
            "X" => {
                count_1 = count_1 + 1;
                match theirs {
                    "A" => {
                        count_1 = count_1 + 3;
                        count_2 = count_2 + 3;
                    }
                    "B" => {
                        count_2 = count_2 + 1;
                    }
                    "C" => {
                        count_1 = count_1 + 6;
                        count_2 = count_2 + 2;
                    }
                    _ => ()
                }
            }
            "Y" => {
                count_1 = count_1 + 2;
                count_2 = count_2 + 3;
                match theirs {
                    "A" => {
                        count_1 = count_1 + 6;
                        count_2 = count_2 + 1;
                    }
                    "B" => {
                        count_1 = count_1 + 3;
                        count_2 = count_2 + 2;
                    }
                    "C" => {
                        count_2 = count_2 + 3;
                    }
                    _ => ()
                }
            }
            "Z" => {
                count_1 = count_1 + 3;
                count_2 = count_2 + 6;
                match theirs {
                    "A" => {
                        count_2 = count_2 + 2;
                    }
                    "B" => {
                        count_1 = count_1 + 6;
                        count_2 = count_2 + 3;
                    }
                    "C" => {
                        count_1 = count_1 + 3;
                        count_2 = count_2 + 1;
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
    return (count_1.to_string(), count_2.to_string());
}