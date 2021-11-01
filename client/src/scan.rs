use std::sync::Arc;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(module = "/js/qrcode.js")]
extern "C" {
    #[wasm_bindgen]
    fn init_webcam();

    #[wasm_bindgen]
    async fn read_qrcode() -> JsValue;
}

pub struct Scan {
    pub display_cb: Arc<Callback<String>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ScanProps {
    pub display_cb: Callback<String>,
}

pub enum Msg {}

impl Component for Scan {
    type Message = Msg;
    type Properties = ScanProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Scan { display_cb: Arc::new(props.display_cb) }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="width: 250px;height: 250px">
                <video id="webcam" autoplay="autoplay"></video>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            init_webcam();
            let display_cb = self.display_cb.clone();
            spawn_local(async move {
                let id = read_qrcode().await;
                display_cb.emit(id.as_string().unwrap());
            });
        }
    }
}
