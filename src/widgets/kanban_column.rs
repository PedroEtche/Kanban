use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::Widget,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
};

use crate::constants::HIGHLIGHT_SIMBOL;

/// A widget use to represent one of the three kanban columns (todo, doing, done)
#[derive(Debug, Clone)]
pub struct KanbanColumn {
    items: Vec<String>,
    state: ListState,
    title: String,
}

impl KanbanColumn {
    pub fn new(title: String) -> Self {
        // Precondition: title should not be empty
        assert!(!title.is_empty(), "Column title cannot be empty");

        let column = KanbanColumn {
            items: Vec::new(),
            state: ListState::default(),
            title,
        };

        // Postcondition: ensure proper initialization
        assert!(column.items.is_empty(), "New column should start empty");
        assert_eq!(
            column.selected(),
            None,
            "New column should have no selection"
        );

        column
    }

    pub fn load(&mut self, items: Vec<String>) {
        self.items = items
    }

    pub fn to_json(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn clear_select(&mut self) {
        self.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.state.select_next()
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous()
    }

    pub fn push(&mut self, task: String) {
        self.items.push(task)
    }

    pub fn remove(&mut self, i: usize) -> String {
        self.items.remove(i)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(Line::raw(self.title.clone()).centered());

        let available_width = calculate_available_width(area, &block);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| {
                let fit_item = fit_to_width(item, available_width);
                ListItem::from(fit_item)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(HIGHLIGHT_SIMBOL)
            .highlight_style(Color::White)
            .fg(Color::Yellow)
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

fn calculate_available_width(area: Rect, block: &Block<'_>) -> usize {
    let block_width = block.inner(area);
    let highlight_width = HIGHLIGHT_SIMBOL.len() as u16;
    block_width.width.saturating_sub(highlight_width) as usize
}

fn fit_to_width(s: &str, width: usize) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

impl Widget for &mut KanbanColumn {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_column_is_empty() {
        let column = KanbanColumn::new("Test".to_string());
        assert_eq!(column.items.len(), 0);
        assert_eq!(column.selected(), None);
        assert_eq!(column.title, "Test");
    }

    #[test]
    fn test_push_item() {
        let mut column = KanbanColumn::new("Test".to_string());
        column.push("Task 1".to_string());
        assert_eq!(column.items.len(), 1);
        assert_eq!(column.items[0], "Task 1");
    }

    #[test]
    fn test_remove_item() {
        let mut column = KanbanColumn::new("Test".to_string());
        column.push("Task 1".to_string());
        column.push("Task 2".to_string());

        let removed = column.remove(0);
        assert_eq!(removed, "Task 1");
        assert_eq!(column.items.len(), 1);
        assert_eq!(column.items[0], "Task 2");
    }

    #[test]
    fn test_selection_navigation() {
        let mut column = KanbanColumn::new("Test".to_string());
        column.push("Task 1".to_string());
        column.push("Task 2".to_string());
        column.push("Task 3".to_string());

        // Test initial selection
        assert_eq!(column.selected(), None);

        // Test select next
        column.select_next();
        assert_eq!(column.selected(), Some(0));

        column.select_next();
        assert_eq!(column.selected(), Some(1));

        // Test select previous
        column.select_previous();
        assert_eq!(column.selected(), Some(0));

        // Test clear selection
        column.clear_select();
        assert_eq!(column.selected(), None);
    }
}
