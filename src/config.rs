use anyhow::Result;
use clap::ArgMatches;
use icalendar::Calendar;
use ratatui::style::Color;

#[derive(Debug)]
pub struct Config {
    pub color: Color,
}

#[derive(Debug, Default)]
struct ConfigBuilder {
    color: Option<Color>,
}

/// The builder to complete the builder pattern for the application configuration
impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn build(&self) -> Result<Config> {
        Ok(
            Config {
                color: self.color.expect("No color specified. Please build the application config first.")
            }
        )
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    name: String,
    pub config: Config,
    calendar: Calendar,
}

impl User {
    pub fn new(cmd: &ArgMatches) -> Self {
        let name = String::from(cmd.get_one::<String>("user").unwrap());
        let color = String::from(cmd.get_one::<String>("color").unwrap());
        let config = ConfigBuilder::new().color(conv_str_to_color(color)).build().unwrap();
        let calendar = Calendar::new();

        User {
            name,
            config,
            calendar,
        }
    }
}

pub fn conv_str_to_color(color_as_string: String) -> Color {
    match color_as_string.to_lowercase().as_str() {
        "yellow" => return Color::Yellow,
        "red" => return Color::Red,
        "green" => return Color::Green,
        "blue" => return Color::Blue,
        "cyan" => return Color::Cyan,
        "black" => return Color::Black,
        _ => unimplemented!("This color is not implemented yet")
    };
}
