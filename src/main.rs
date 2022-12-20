use components::day_five::solve5;
use components::day_four::solve4;
use components::day_seven::solve7;
use components::day_six::solve6;
use components::day_three::solve3;
use components::day_two::solve2;
use components::day_one::solve1;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

mod components;

// Define the possible messages which can be sent to the component
pub enum Msg {
    SetDay(i32),
    UpdateInput(Event)
}

pub struct App {
    day: i32,
    part1: String,
    part2: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { day: 0, part1: "-".to_owned(), part2: "-".to_owned() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetDay(new_value) => {
                self.day = new_value;
                true
            }
            Msg::UpdateInput(event) => {
                let text = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
                let (result1, result2) = match self.day {
                    1 => { solve1(text) }
                    2 => { solve2(text) }
                    3 => { solve3(text) }
                    4 => { solve4(text) }
                    5 => { solve5(text) }
                    6 => { solve6(text) }
                    7 => { solve7(text) }
                    _ => { ("-".to_owned(), "-".to_owned()) }
                };
                self.part1 = result1;
                self.part2 = result2;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let indices = 1..26;
        html! {
            <div>
                <div class="panel">
                    { 
                        indices.into_iter().map(|index| {
                            html!{
                                <button key={index} onclick={ctx.link().callback(move |_| Msg::SetDay(index))}>
                                    { index }{ if self.day == index { "*" } else { "" } }
                                </button>
                            }
                        }).collect::<Html>()
                    }
                </div>

                <div>
                    <textarea style={"width: 400px; height: 400px"} onchange={ctx.link().callback(move |event: Event| Msg::UpdateInput(event))} />
                    <p>{ "Part One:"} { self.part1.clone() }</p>
                    <p>{ "Part Two:"} { self.part2.clone() }</p>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}