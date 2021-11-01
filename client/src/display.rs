use yew::prelude::*;

pub struct Display {
    title: String,
    text: String,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct DisplayProps {
    pub title: String,
    pub text: String,
}

impl Component for Display {
    type Message = Msg;
    type Properties = DisplayProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Display { title: props.title, text: props.text }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title != props.title || self.text != props.text
    }

    fn view(&self) -> Html {
        let title = &self.title;
        let text = &self.text;
        html! {
            <div>
                <h2>{title}</h2>
                <p>{text}</p>
            </div>
        }
    }
}
