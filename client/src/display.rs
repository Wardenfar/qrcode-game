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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let title = &self.props.title;
        let image = self.props.image.clone();
        let text = &self.props.text;

        let img_html = if let Some(image) = image {
            html! {<img class="image" src=image />}
        } else {
            html! {}
        };

        let dom = html! {
            <div class="p-3">
                <h1 class="font-bold font-3xl text-center text-blue">{title}</h1>
                {img_html}
                <p>{text}</p>
            </div>
        };
        dom


    }
}
