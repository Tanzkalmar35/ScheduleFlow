/// Holds all available colors to choose from
enum Colors {
    Yellow,
}

#[derive(Debug)]
pub struct Config {
    color: Colors,
    user: User, // TODO: Create a user
}

#[derive(Default, Clone)]
pub struct ConfigBuilder {
    color: Option<Colors>,
    user: Option<User>, // TODO: Create a user
}

/// The builder to complete the builder pattern for the application configuration
impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn color(mut self, color: impl Into<Colors>) -> Self {
        self.color.insert(color.into());
        self
    }

    pub fn user(mut self, user: impl Into<User>) -> Self {
        self.user.insert(user.into());
        self
    }   

    pub fn build(&self) -> Result<Config> {
        Ok(
            Config {
                config = self.config,
                user = self.user
            }
        )
    }
}