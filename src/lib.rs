mod counter;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::counter::Model as Counter;

struct Model {
    _link: ComponentLink<Self>,
}

enum Msg {}

impl yew::Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link: link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {}
        true
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
                {for (0..3).map(|i| html! {<Counter init_value=i />})}
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
