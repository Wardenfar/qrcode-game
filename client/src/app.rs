use wasm_bindgen::JsValue;
use yew::prelude::*;
use crate::{Header, HeaderProps, Scan, ScanProps};
use web_sys::{Request, RequestInit, RequestMode, Response};
use common::Game;
use crate::display::{Display, DisplayProps};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/game.js")]
extern "C" {
    #[wasm_bindgen]
    fn get_game_toml_val() -> JsValue;
}

pub struct App {
    scan: bool,
    current_id: Option<String>,
    link: ComponentLink<Self>,
    game: Game,
}

pub enum Msg {
    Scan,
    Display(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let input = get_game_toml_val().as_string().unwrap();
        let game = common::parse(&input);
        App { scan: true, current_id: None, link, game }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Scan => {
                self.scan = true;
                self.current_id = None;
                true
            }
            Msg::Display(id) => {
                self.scan = false;
                self.current_id = Some(id);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let scan_callback = self.link.callback(|_evt: MouseEvent| Msg::Scan);
        let display_callback = self.link.callback(|id| Msg::Display(id));

        let header_props = yew::props!(HeaderProps {
            scan_cb: scan_callback
        });

        if let Some(id) = &self.current_id {
            let (title, text, image) = if let Some(code) = self.game.find_code(id) {
                (code.title.clone(), code.text.clone(), code.image.clone())
            } else {
                (String::from("QrCode invalide"), String::from("..."), None)
            };

            let props = yew::props!(DisplayProps {
                title: title,
                image: image,
                text: text
            });
            let dom =  html! {
                <div id="root" class="overflow-x-hidden">
                    <Header with header_props />
                    <Display with props />
                </div>
            };
            dom
        } else {
            let props = yew::props!(ScanProps {
                display_cb: display_callback
            });
            let dom = html! {
                <div id="root" class="overflow-x-hidden">
                    <Header with header_props />
                    <Scan with props />
                </div>
            };
            dom
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {}
    }
}
