use yew::prelude::*;

pub struct Header {
    pub props: HeaderProps,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub scan_cb: Callback<MouseEvent>,
    pub history_cb: Callback<MouseEvent>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex items-center justify-between sticky top-0 z-50 bg-gray-800 p-2">
                <svg onclick={&self.props.history_cb} class="text-white fill-current stroke-current" viewBox="0 0 100 80" width="40" height="30">
                    <rect y="0" width="100" height="10"></rect>
                    <rect y="35" width="100" height="10"></rect>
                    <rect y="70" width="100" height="10"></rect>
                </svg>
                <button onclick={&self.props.scan_cb} class="btn btn-blue">{ "Scan QrCode" }</button>
            </div>
        }
    }
}
