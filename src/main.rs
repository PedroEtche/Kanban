use color_eyre::Result;
use kanban::kanban::Kanban;

/// Runs the Kanban app. Ratatui makes the heavy lifting for renderization
fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = Kanban::new()?.run(terminal);
    ratatui::restore();
    app_result
}
