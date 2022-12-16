use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn DayTwo() -> Html {
    let part1 = use_state(|| 0);
    let part2 = use_state(|| 0);
    let onchange = {
        let part1 = part1.clone();
        let part2 = part2.clone();
        Callback::from(move |event: Event| {
            let text = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            let mut count_1 = 0;
            let mut count_2 = 0;

            for line in text.split("\n") {
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