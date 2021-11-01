use yew::prelude::*;

pub struct Header {
    scan_cb: Callback<MouseEvent>
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub scan_cb: Callback<MouseEvent>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { scan_cb: props.scan_cb }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_click = &self.scan_cb;
        html! {
            <div class="flex justify-center sticky top-0 z-50 bg-gray-800 p-2">
                <button onclick={on_click} class="btn btn-blue">{ "Scan QrCode" }</button>
            </div>
        }
    }
}
