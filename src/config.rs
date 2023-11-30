use clap::ArgMatches;
use icalendar::Calendar;
use ratatui::style::Color;

#[derive(Debug)]
pub struct Config {
    pub color: Color,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        Self {
           color: Color::Yellow 
        }
    }

    pub fn color(color: Color) -> Self {
        Self {
            color
        }
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
        let config = Config::color(conv_str_to_color(color));
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
