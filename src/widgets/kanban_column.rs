use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::Widget,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
};

#[derive(Debug, Clone)]
pub struct KanbanColumn {
    items: Vec<String>,
    state: ListState,
    title: String,
}

impl KanbanColumn {
    pub fn new(title: String) -> Self {
        KanbanColumn {
            items: Vec::new(),
            state: ListState::default(),
            title,
        }
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

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| ListItem::from(item.clone())) //.bg(Color::Black))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_symbol("> ")
            .fg(Color::Yellow)
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl Widget for &mut KanbanColumn {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
}
