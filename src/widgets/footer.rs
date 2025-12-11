use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

pub struct Footer;

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use j/k to move, a/s/d to navigate lists, A/S/D move items.")
            .centered()
            .render(area, buf);
    }
}
