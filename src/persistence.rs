use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::widgets::kanban_column::KanbanColumn;

#[derive(Debug, Serialize, Deserialize)]
pub struct Persistence {
    todo: Vec<String>,
    doing: Vec<String>,
    done: Vec<String>,
}

impl Persistence {
    pub fn load() -> Result<(KanbanColumn, KanbanColumn, KanbanColumn)> {
        let data = fs::read_to_string("kanban.json")?;
        let load_data: Persistence = serde_json::from_str(&data)?;

        let mut todo_list = KanbanColumn::new(String::from("TODO"));
        let mut doing_list = KanbanColumn::new(String::from("Doing"));
        let mut done_list = KanbanColumn::new(String::from("Done"));

        todo_list.load(load_data.todo);
        doing_list.load(load_data.doing);
        done_list.load(load_data.done);

        Ok((todo_list, doing_list, done_list))
    }

    pub fn persist(todo_list: &KanbanColumn, doing_list: &KanbanColumn, done_list: &KanbanColumn) {
        let john = json!({
            "todo": todo_list.to_json(),
            "doing": doing_list.to_json(),
            "done": done_list.to_json()
        });

        let file = File::create("kanban.json").unwrap();
        serde_json::to_writer_pretty(file, &john).unwrap();
    }
}
