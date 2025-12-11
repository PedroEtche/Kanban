use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::constants::{
    DOING_LIST, DONE_LIST, MOVE_DOWN, MOVE_TO_DOING, MOVE_TO_DONE, MOVE_TO_TODO, MOVE_UP, TODO_LIST,
};

pub struct Footer;

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let message = format!(
            "Use {}/{} to move, {}/{}/{} to navigate lists, {}/{}/{} move items.",
            MOVE_DOWN,
            MOVE_UP,
            TODO_LIST,
            DOING_LIST,
            DONE_LIST,
            MOVE_TO_TODO,
            MOVE_TO_DOING,
            MOVE_TO_DONE
        );
        Paragraph::new(message).centered().render(area, buf);
    }
}
