mod counter;
mod todo;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::counter::Model as Counter;
use crate::todo::Model as TodoList;

struct Model {
    link: ComponentLink<Self>,
    init_values: Vec<i64>,
}

enum Msg {
    AddCounter,
}

impl yew::Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            init_values: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let nxt_val = self.init_values.last().unwrap_or(&-1) + 1;
        match msg {
            Msg::AddCounter => {
                self.init_values.push(nxt_val);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddCounter)>{"add"}</button>
                {for (&self.init_values).into_iter().map(|&i| html! {<Counter init_value=i />})}

                <TodoList />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
