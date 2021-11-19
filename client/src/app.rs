use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use yew::prelude::*;

use common::{Code, Game};

use crate::{Header, HeaderProps, Scan, ScanProps};
use crate::display::{Display, DisplayProps};

#[wasm_bindgen(module = "/js/game.js")]
extern "C" {
    #[wasm_bindgen]
    fn get_game_toml_val() -> JsValue;

    #[wasm_bindgen]
    fn set_hash(hash: String) -> JsValue;

    #[wasm_bindgen]
    fn on_hash_changed(closure: &Closure<dyn FnMut(String)>) -> JsValue;
}

pub struct App {
    state: State,
    history: Vec<Code>,
    unlocked: Vec<String>,
    link: ComponentLink<Self>,
    game: Game,
    _hash_changed_closure: Closure<dyn FnMut(String)>,
}

#[derive(Debug, Clone)]
pub enum State {
    Scan,
    Display(String),
    History,
}

pub enum Msg {
    State(State),
    Unlock(String),
    HashChanged,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let input = get_game_toml_val().as_string().unwrap();
        let game = common::parse(&input);

        // let link = Arc::new(link);

        let callback = link.callback(|m: Msg| m);
        let _hash_changed_closure = Closure::wrap(Box::new(move |_e: String| {
            callback.emit(Msg::HashChanged);
        }) as Box<dyn FnMut(String)>);
        on_hash_changed(&_hash_changed_closure);

        set_hash("scan".to_string());

        App { state: State::Scan, history: vec![], unlocked: vec![], link, game, _hash_changed_closure }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                self.state = state.clone();
                match state {
                    State::Scan => {
                        set_hash("scan".to_string());
                        true
                    }
                    State::Display(id) => {
                        if let Some(code) = self.game.find_code(&id) {
                            if !self.history.contains(code) {
                                self.history.push(code.clone())
                            }
                        }
                        set_hash("display".to_string());
                        true
                    }
                    State::History => {
                        set_hash("history".to_string());
                        true
                    }
                }
            }
            Msg::HashChanged => {
                if let State::Scan = self.state {
                    false
                } else {
                    self.state = State::Scan;
                    true
                }
            },
            Msg::Unlock(entered_pass) => {
                let unlocked= if let State::Display(id) = &self.state {
                    if self.unlocked.contains(id) {
                        None
                    } else if let Some(code) = self.game.find_code(id) {
                        if let Some(pass) = &code.pass {
                            if pass.secret == entered_pass {
                                Some(id.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(unlocked) = unlocked {
                    self.unlocked.push(unlocked.clone());
                    self.state = State::Display(unlocked);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let scan_callback = self.link.callback(|_evt: MouseEvent| Msg::State(State::Scan));
        let history_callback = self.link.callback(|_evt: MouseEvent| Msg::State(State::History));
        let display_callback = self.link.callback(|id| Msg::State(State::Display(id)));
        let unlock_callback = self.link.callback(|pass: InputData| Msg::Unlock(pass.value));

        let header_props = yew::props!(HeaderProps {
            scan_cb: scan_callback,
            history_cb: history_callback
        });

        match &self.state {
            State::Scan => {
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
            State::Display(id) => {
                let (title, text, image, is_pass) = if let Some(code) = self.game.find_code(id) {
                    if let Some(pass) = &code.pass {
                        if self.unlocked.contains(id) {
                            (code.title.clone(), code.text.clone(), code.image.clone(), false)
                        } else {
                            (pass.title.clone(), pass.text.clone(), pass.image.clone(), true)
                        }
                    } else {
                        (code.title.clone(), code.text.clone(), code.image.clone(), false)
                    }

                } else {
                    (String::from("QrCode invalide"), String::from("..."), None, false)
                };

                let props = yew::props!(DisplayProps {
                    title: title,
                    image: image,
                    text: text,
                    is_pass: is_pass,
                    unlock_cb: unlock_callback
                });
                let dom = html! {
                    <div id="root" class="overflow-x-hidden">
                        <Header with header_props />
                        <Display with props />
                    </div>
                };
                dom
            }
            State::History => {
                let mut list: Vec<_> = Vec::default();
                for code in &self.history {
                    let id = code.id.clone();
                    let onclick = self.link.callback(move |_| Msg::State(State::Display(id.clone())));
                    let dom = html! {
                        <div>
                            <a onclick=onclick>{&code.title}</a>
                        </div>
                    };
                    list.push(dom);
                }
                let dom = html! {
                    <div id="root" class="overflow-x-hidden">
                        <Header with header_props />
                        <div>
                            {list}
                        </div>
                    </div>
                };
                dom
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {}
    }
}