use yew::prelude::*;
use crate::Header;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="root" class="overflow-x-hidden">
                <Header/>
                <div class="relative">
                    <p>{ "Hello world!" }</p>
                    <div style="width: 250px;height: 250px">
                        <video autoplay="autoplay" controls=true></video>
                    </div>
                </div>
            </div>
        }
    }
}
