use yew::prelude::*;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex justify-center sticky top-0 z-50 bg-gray-800 p-2">
                <button class="btn btn-blue">{ "Scan QrCode" }</button>
            </div>
        }
    }
}
