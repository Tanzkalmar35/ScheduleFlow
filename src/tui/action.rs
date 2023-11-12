use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::tui_app::TuiApp;

pub fn update(app: &mut TuiApp, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        _ => {}
    }
}
