use web_sys::Element;
use yew::prelude::*;

pub struct Display {
    props: DisplayProps,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct DisplayProps {
    pub title: String,
    pub image: Option<String>,
    pub text: String,
    pub is_pass: bool,
    pub unlock_cb: Callback<InputData>,
}

impl Component for Display {
    type Message = Msg;
    type Properties = DisplayProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Display { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let title = &self.props.title;
        let image = self.props.image.clone();
        let text = &self.props.text;
        let is_pass = self.props.is_pass;
        let unlock_cb = &self.props.unlock_cb;

        let img_html = if let Some(image) = image {
            html! {<img class="image" src=image />}
        } else {
            html! {}
        };

        let pass = if is_pass {
            html! {
                <div>
                    <input oninput={unlock_cb} type="password" placeholder="Mot de Passe"/>
                </div>
            }
        } else {
            html! {}
        };

        let dom = html! {
            <div class="p-3">
                <h1 class="font-bold font-3xl text-center text-blue">{title}</h1>
                {pass}
                {img_html}
                <p>{text}</p>
            </div>
        };
        dom


    }
}
