fn check_signal(cycle: i32, total: i32) -> i32 {
    if (cycle + 20) % 40 == 0 {
        return cycle * total
    } else {
        return 0
    }
}

fn check_render(cycle: i32, total: i32) -> String {
    let pixel = if ((cycle % 40) - total - 1).abs() <= 1 { "#".to_owned() } else { "0".to_owned() };
    if cycle % 40 == 0 {
        return pixel + "\n"
    } else {
        return pixel
    }
}

pub fn solve10(input: String) -> (String, String) {
    let mut cycles = 0;
    let mut total = 1;
    let mut signals = 0;
    let mut render = "\n".to_owned();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["noop"] => {
                cycles += 1;
                signals += check_signal(cycles, total);
                render += &check_render(cycles, total);
            }
            ["addx", amount] => {
                cycles += 1;
                signals += check_signal(cycles, total);
                render += &check_render(cycles, total);
                cycles += 1;
                signals += check_signal(cycles, total);
                render += &check_render(cycles, total);
                total += amount.parse::<i32>().unwrap()
            }
            _ => {}
        }
    }
    return (signals.to_string(), render.to_string())
}