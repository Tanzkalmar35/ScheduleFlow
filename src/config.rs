use anyhow::Result;
use icalendar::Calendar;

/// Holds all available colors to choose from
enum Colors {
    Yellow,
}

#[derive(Debug)]
struct Config {
    color: Colors,
}

#[derive(Debug)]
struct ConfigBuilder {
    color: Option<Colors>,
}

/// The builder to complete the builder pattern for the application configuration
impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn color(mut self, color: impl Into<Colors>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn build(&self) -> Result<Config> {
        Ok(
            Config {
                color: self.color.expect("")
            }
        )
    }
}

pub struct User {
    name: String,
    config: Config,
    calendar: Calendar,
}

impl User {
    pub fn new() -> Self {
        User {
            name: String::new(),
            config: ConfigBuilder::new().build().unwrap(),
            calendar: Calendar::new(),
        }
    }
}
