use crate::{
    constants::{
        DELETE_TASK_KEY, DOING_LIST_KEY, DONE_LIST_KEY, MOVE_TASK_TO_DOING_LIST_KEY,
        MOVE_TASK_TO_DONE_LIST_KEY, MOVE_TASK_TO_TODO_LIST_KEY, TODO_LIST_KEY,
    },
    helpers::popup_area,
    widgets::footer::Footer,
    widgets::input_box::InputBox,
    widgets::kanban_list::KanbanList,
};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Position},
};

pub struct Kanban {
    /// Flag to gracefully shutdown
    should_exit: bool,
    /// Kanban List to render
    todo_list: KanbanList,
    doing_list: KanbanList,
    done_list: KanbanList,
    selected_list: SelectedList,
    input_mode: InputMode,
    input_box: InputBox,
}

#[derive(Debug, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, PartialEq)]
pub enum SelectedList {
    Todo,
    Doing,
    Done,
}

impl Default for Kanban {
    fn default() -> Self {
        let list = KanbanList::from_iter([
            "Rewrite everything with Rust!",
            "Rewrite all of your tui apps with Ratatui",
            "Pet your cat",
            "Walk with your dog",
            "Pay the bills",
            "Refactor list example",
        ]);

        Self {
            should_exit: false,
            todo_list: list.clone(),
            doing_list: list.clone(),
            done_list: list,
            selected_list: SelectedList::Todo,
            input_mode: InputMode::Normal,
            input_box: InputBox::new(),
        }
    }
}

impl Kanban {
    pub fn new() -> Self {
        Kanban {
            should_exit: false,
            todo_list: KanbanList::new(String::from("TODO")),
            doing_list: KanbanList::new(String::from("Doing")),
            done_list: KanbanList::new(String::from("Done")),
            selected_list: SelectedList::Todo,
            input_mode: InputMode::Normal,
            input_box: InputBox::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
        let [main_area, footer_area] = layout.areas(frame.area());

        let [todo_area, doing_area, done_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(main_area);

        frame.render_widget(Footer, footer_area);

        frame.render_widget(&mut self.todo_list, todo_area);
        frame.render_widget(&mut self.doing_list, doing_area);
        frame.render_widget(&mut self.done_list, done_area);

        if self.input_mode == InputMode::Editing {
            let input_area = popup_area(main_area, 60, 20);
            frame.render_widget(&mut self.input_box, input_area);

            // TODO: Intentar delegar el renderizado del cursor al input box
            frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.input_box.get_char_index() as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            ));
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.input_mode {
            InputMode::Normal => self.normal_mode_input(key),
            InputMode::Editing => self.editing_mode_input(key),
        }
    }

    fn normal_mode_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('p') => self.change_input_mode(),
            KeyCode::Char(TODO_LIST_KEY) => self.focus_todo(),
            KeyCode::Char(DOING_LIST_KEY) => self.focus_doing(),
            KeyCode::Char(DONE_LIST_KEY) => self.focus_done(),
            KeyCode::Char(MOVE_TASK_TO_TODO_LIST_KEY) => self.move_item_to_todo(),
            KeyCode::Char(MOVE_TASK_TO_DOING_LIST_KEY) => self.move_item_to_doing(),
            KeyCode::Char(MOVE_TASK_TO_DONE_LIST_KEY) => self.move_item_to_done(),
            KeyCode::Char(DELETE_TASK_KEY) => self.delete_item(),
            _ => {}
        }
    }

    fn editing_mode_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => self.submit_message(),
            KeyCode::Char(to_insert) => self.input_box.enter_char(to_insert),
            KeyCode::Backspace => self.input_box.delete_char(),
            KeyCode::Left => self.input_box.move_cursor_left(),
            KeyCode::Right => self.input_box.move_cursor_right(),
            KeyCode::Esc => self.input_mode = InputMode::Normal,
            _ => {}
        }
    }

    // Helper to get the currently active list
    fn current_list(&mut self) -> &mut KanbanList {
        match self.selected_list {
            SelectedList::Todo => &mut self.todo_list,
            SelectedList::Doing => &mut self.doing_list,
            SelectedList::Done => &mut self.done_list,
        }
    }

    fn focus_todo(&mut self) {
        self.change_focus_to(SelectedList::Todo);
    }

    fn focus_doing(&mut self) {
        self.change_focus_to(SelectedList::Doing);
    }

    fn focus_done(&mut self) {
        self.change_focus_to(SelectedList::Done);
    }

    fn change_focus_to(&mut self, new_focus: SelectedList) {
        self.current_list().clear_select();
        self.selected_list = new_focus;
        self.current_list().select_next();
    }

    fn move_item_to_todo(&mut self) {
        self.move_item_to(SelectedList::Todo)
    }

    fn move_item_to_doing(&mut self) {
        self.move_item_to(SelectedList::Doing)
    }

    fn move_item_to_done(&mut self) {
        self.move_item_to(SelectedList::Done)
    }

    fn move_item_to(&mut self, destination_list: SelectedList) {
        if self.selected_list == destination_list {
            return;
        }

        if let Some(i) = self.current_list().selected() {
            let item = self.current_list().remove(i);
            self.move_item_to_list(destination_list, item);
        }
    }

    fn move_item_to_list(&mut self, destination_list: SelectedList, item: String) {
        match destination_list {
            SelectedList::Todo => self.todo_list.push(item),
            SelectedList::Doing => self.doing_list.push(item),
            SelectedList::Done => self.done_list.push(item),
        }
    }

    fn delete_item(&mut self) {
        if let Some(i) = self.current_list().selected() {
            self.current_list().remove(i);
        }
    }

    fn select_next(&mut self) {
        self.current_list().select_next();
    }

    fn select_previous(&mut self) {
        self.current_list().select_previous();
    }

    fn change_input_mode(&mut self) {
        match self.input_mode {
            InputMode::Normal => self.input_mode = InputMode::Editing,
            InputMode::Editing => self.input_mode = InputMode::Normal,
        }
    }

    fn submit_message(&mut self) {
        if let Some(message) = self.input_box.submit_message() {
            self.todo_list.push(message);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn add_todo_is_saved_and_selectable() {
//         let mut kanban = Kanban::new();
//
//         kanban.add_todo(String::from("Sacar a Emma"));
//
//         // Test that the todo was added
//         assert_eq!(kanban.todo_list.items.len(), 1);
//         assert_eq!(kanban.todo_list.items[0], "Sacar a Emma");
//
//         // Test UI state: initially nothing is selected
//         assert_eq!(kanban.todo_list.state.selected(), None);
//
//         // Test UI state: can select the new item
//         kanban.todo_list.state.select_next();
//         assert_eq!(kanban.todo_list.state.selected(), Some(0));
//         assert_eq!(kanban.todo_list.items[0], "Sacar a Emma");
//     }
//
//     #[test]
//     fn ui_selection_works_with_multiple_todos() {
//         let mut kanban = Kanban::new();
//
//         kanban.add_todo(String::from("First task"));
//         kanban.add_todo(String::from("Second task"));
//         kanban.add_todo(String::from("Third task"));
//
//         // Initially nothing selected
//         assert_eq!(kanban.todo_list.state.selected(), None);
//
//         // Select first item
//         kanban.todo_list.state.select_next();
//         assert_eq!(kanban.todo_list.state.selected(), Some(0));
//         assert_eq!(kanban.todo_list.items[0], "First task");
//
//         // Select second item
//         kanban.todo_list.state.select_next();
//         assert_eq!(kanban.todo_list.state.selected(), Some(1));
//         assert_eq!(kanban.todo_list.items[1], "Second task");
//
//         // Select third item
//         kanban.todo_list.state.select_next();
//         assert_eq!(kanban.todo_list.state.selected(), Some(2));
//         assert_eq!(kanban.todo_list.items[2], "Third task");
//     }
//
//     #[test]
//     fn focus_changes_work_correctly() {
//         let mut kanban = Kanban::new();
//
//         // Initially focused on todo
//         kanban.focus_todo();
//         assert_eq!(kanban.selected_list, SelectedList::Todo);
//
//         // Switch to doing
//         kanban.focus_doing();
//         assert_eq!(kanban.selected_list, SelectedList::Doing);
//
//         // Switch to done
//         kanban.focus_done();
//         assert_eq!(kanban.selected_list, SelectedList::Done);
//     }
//
//     #[test]
//     fn move_item_between_lists_updates_ui_correctly() {
//         let mut kanban = Kanban::new();
//
//         kanban.add_todo(String::from("Task to move"));
//
//         // Focus on todo list and select the item
//         kanban.focus_todo();
//         assert_eq!(kanban.todo_list.items.len(), 1);
//         assert_eq!(kanban.doing_list.items.len(), 0);
//
//         // Move item to doing
//         kanban.move_item_to_doing();
//
//         // Verify the move worked
//         assert_eq!(kanban.todo_list.items.len(), 0);
//         assert_eq!(kanban.doing_list.items.len(), 1);
//         assert_eq!(kanban.doing_list.items[0], "Task to move");
//     }
// }
