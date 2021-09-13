use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub content: String,
    pub onremove: Callback<()>,
}

pub enum Msg {
    Remove,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Remove => self.props.onremove.emit(()),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props == self.props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>{self.props.title.as_str()}</div>
                <div>{self.props.content.as_str()}</div>
                <button onclick=self.link.callback(|_| Msg::Remove)>{"delete"}</button>
            </div>
        }
    }
}
