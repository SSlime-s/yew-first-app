use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    _props: Props,
    value: i64,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or(0)]
    pub init_value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let init_value = props.init_value;
        Self {
            link,
            _props: props,
            value: init_value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Decrement => self.value -= 1,
            Msg::Increment => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Decrement)>{"-1"}</button>
                <span>{ self.value }</span>
                <button onclick=self.link.callback(|_| Msg::Increment)>{"+1"}</button>
                <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>{"+1, +1"}</button>
            </div>
        }
    }
}
