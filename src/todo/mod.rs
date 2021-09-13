mod card;

use yew::prelude::*;

use self::card::Model as TaskCard;

pub struct Model {
    link: ComponentLink<Self>,
    task_list: Vec<TaskInfo>,
    now_title: String,
    now_content: String,
}

pub enum Msg {
    AddTask,
    RemoveTask(usize),
    TitleUpdate(ChangeData),
    ContentUpdate(ChangeData),
}

struct TaskInfo {
    title: String,
    content: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            task_list: vec![TaskInfo {
                title: "Hello".to_string(),
                content: "World !".to_string(),
            }],
            now_title: String::new(),
            now_content: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTask => {
                self.task_list.push(TaskInfo {
                    title: self.now_title.clone(),
                    content: self.now_content.clone(),
                });
                true
            }
            Msg::RemoveTask(idx) => {
                self.task_list.remove(idx);
                true
            }
            Msg::TitleUpdate(data) => {
                match data {
                    ChangeData::Value(msg) => self.now_title = msg,
                    _ => unreachable!(),
                }
                false
            }
            Msg::ContentUpdate(data) => {
                match data {
                    ChangeData::Value(msg) => self.now_content = msg,
                    _ => unreachable!(),
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input onchange=self.link.callback(Msg::TitleUpdate) />
                <input onchange=self.link.callback(Msg::ContentUpdate) />
                <button onclick=self.link.callback(|_| Msg::AddTask)>{"add"}</button>

                <div>
                    {for (&self.task_list).into_iter().enumerate().map(|(i, info)| html!{
                        <TaskCard
                            title=info.title.clone()
                            content=info.content.clone()
                            onremove=self.link.callback(move |_| Msg::RemoveTask(i))
                        />
                    })}
                </div>
            </div>
        }
    }
}
