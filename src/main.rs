use color_eyre::Result;
use kanban::kanban::Kanban;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = Kanban::new().run(terminal);
    ratatui::restore();
    app_result
}
