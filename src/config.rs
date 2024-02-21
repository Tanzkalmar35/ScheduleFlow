use clap::ArgMatches;
use icalendar::Calendar;

#[derive(Debug)]
pub struct Config {}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        Self {}
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
        let config = Config::new();
        let calendar = Calendar::new();

        User {
            name,
            config,
            calendar,
        }
    }
}
