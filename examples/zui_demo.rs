use std::io;
use zui_widgets::backend::ZuiBackend;
use zui_widgets::layout::{Constraint, Direction, Layout};
use zui_widgets::widgets::{Block, Borders, Widget};
use zui_widgets::Terminal;

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let backend = ZuiBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    Ok(())
}
