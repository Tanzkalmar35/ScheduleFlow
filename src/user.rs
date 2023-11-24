// TODO: add the config ref

#[derive(Debug, Default)]
pub struct User {
    name: String,
    config: Config,
}

impl User {
    pub fn new() -> Self {
        User {
            name: String::new(),
            config: Config::new(),
        }
    }
}
