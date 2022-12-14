use std::{collections::BinaryHeap};

use gloo::console;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn DayOne() -> Html {
    let part1 = use_state(|| 0);
    let part2 = use_state(|| 0);
    let onchange = {
        let part1 = part1.clone();
        let part2 = part2.clone();
        Callback::from(move |event: Event| {
            let text = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            let mut current_count = 0;
            let mut max = 0;
            
            let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
            for line in text.split("\n") {
                if line == "" {
                    let mut stringify = "".to_owned();
                    console::log!("len {}", heap.len());
                    for item in &heap {
                        stringify = format!("{}, {}", stringify, *item)
                    }
                    console::log!(stringify);
                    console::log!(current_count);
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
            console::log!(current_count);
            if current_count > max {
                heap.push(-current_count);
                    if heap.capacity() > 3 {
                        heap.pop();
                    }
                max = current_count;
            }
            for item in &heap {
                console::log!(*item);
            }
            part1.set(max);
            part2.set(heap.into_iter().map(|item| -item).sum());
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