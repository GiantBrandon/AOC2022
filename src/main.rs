use gloo::console;
use yew::prelude::*;

mod components;

use components::day_one::DayOne;
use components::day_two::DayTwo;

// Define the possible messages which can be sent to the component
pub enum Msg {
    SetValue(i64),
}

pub struct App {
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetValue(new_value) => {
                self.value = new_value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let indices = 1..26;
        console::log!(self.value == 1);
        html! {
            <div>
                <div class="panel">
                    { 
                        indices.into_iter().map(|index| {
                            html!{
                                <button key={index} onclick={ctx.link().callback(move |_| Msg::SetValue(index))}>
                                    { index }
                                </button>
                            }
                        }).collect::<Html>()
                    }
                </div>

                if self.value == 1 { <DayOne /> }
                if self.value == 2 { <DayTwo /> }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}