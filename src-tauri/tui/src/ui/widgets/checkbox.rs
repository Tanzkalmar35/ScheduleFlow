use ratatui::widgets::Paragraph;

#[derive(Clone)]
pub(crate) struct CheckboxWidget {
    is_checked: bool,
    desc: String,
    pub(crate) key: char,
}

impl CheckboxWidget {
    pub(crate) fn new(desc: String, key: char) -> Self {
        Self {
            is_checked: false,
            desc,
            key,
        }
    }

    pub(crate) fn is_checked(&self) -> bool {
        self.is_checked
    }

    pub(crate) fn toggle(&mut self) {
        self.is_checked = !self.is_checked;
    }

    pub(crate) fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let checkbox = if self.is_checked() {
            Paragraph::new(format!("[x] {} - {}", self.desc, self.key))
        } else {
            Paragraph::new(format!("[ ] {} - {}", self.desc, self.key))
        };
        frame.render_widget(checkbox, area);
    }
}
