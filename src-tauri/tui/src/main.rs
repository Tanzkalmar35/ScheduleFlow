use dotenv::dotenv;
use ui::tui::Tui;

mod constants;
mod ui;

fn main() {
    dotenv().ok();
    if let Err(e) = Tui::start() {
        panic!("Tui could not start: {}", e)
    }
}
