use ui::tui::Tui;

mod ui;

fn main() {
    if let Err(e) = Tui::start() {
        panic!("Tui could not start: {}", e)
    }
}
