use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::chip8;

pub struct DisplayWidget<'b> {
    display: &'b chip8::Display,
}

impl DisplayWidget<'_> {
    pub fn new(display: &chip8::Display) -> DisplayWidget<'_> {
        DisplayWidget { display }
    }
}

impl Widget for DisplayWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (y, row) in self.display.iter().enumerate() {
            let string = row.map(|pixel| if pixel { "██" } else { "  " }).join("");
            buf.set_string(area.x, area.y + y as u16, string, Style::default());
        }
    }
}
