use dotenv::dotenv;
use ui::tui::Tui;

mod constants;
mod ui;

fn main() {
    // TODO: Driver
    dotenv().ok();
    if let Err(e) = Tui::start() {
        panic!("Tui could not start: {}", e)
    }
}
