use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Game {
    pub config: Config,
    pub codes: Vec<Code>
}

impl Game {
    pub fn find_code(&self, id: &str) -> Option<&Code> {
        self.codes.iter().find(|c| c.id == id)
    }
}

#[derive(Deserialize)]
pub struct Code {
    pub id: String,
    pub title: String,
    pub text: String
}

#[derive(Deserialize)]
pub struct Config {
    pub team: bool
}

pub fn parse(input: &str) -> Game {
    toml::from_str(input).unwrap()
}