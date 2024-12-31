use ratatui::prelude::{Buffer, Rect};
use ratatui::style::Style;
use ratatui::widgets::Widget;

pub(crate) struct InputField {
    value: String,
}

impl InputField {
    pub(crate) fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub(crate) fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub(crate) fn get_value(&self) -> &str {
        &self.value
    }
}

impl Widget for InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Draw the border of the input field
        //let border_style = Style::default(); // You can customize the style here

        // Draw the input value inside the field
        let input_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        // Draw the text inside the input area
        buf.set_stringn(
            input_area.x,
            input_area.y,
            &self.value,
            input_area.width as usize,
            Style::default(),
        );
    }
}
