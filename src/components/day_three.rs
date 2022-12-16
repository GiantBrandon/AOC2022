use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn DayThree() -> Html {
    let part1 = use_state(|| 0);
    let part2 = use_state(|| 0);
    let onchange = {
        let part1 = part1.clone();
        let part2 = part2.clone();
        Callback::from(move |event: Event| {
            let text = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            let mut count_1 = 0;
            let mut count_2 = 0;

            let mut group: Vec<&str> = Vec::new();
            for line in text.split("\n") {
                let (first, second) = line.split_at(line.len() / 2);
                for char in first.chars() {
                    if second.contains(char) {
                        if char.is_uppercase() {
                            count_1 = count_1 + char as u32 - 38;
                        } else {
                            count_1 = count_1 + char as u32 - 96
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
                        count_2 = count_2 + found as u32 - 38;
                    } else {
                        count_2 = count_2 + found as u32 - 96
                    }
                    group.clear()
                }
            }
            part1.set(count_1);
            part2.set(count_2);
        })
    };

    html! {
        <div>
            <textarea onchange={onchange} />
            <p>{ "Part One:"} { *part1 }</p>
            <p>{ "Part Two:"} { *part2 }</p>
        </div>
    }
}