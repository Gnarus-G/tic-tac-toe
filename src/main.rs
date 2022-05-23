use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let third =
        |c| tui::layout::Rect::new(f.size().x + (f.size().width / 3 * c), f.size().y, f.size().width / 3, f.size().height);

    let layout = |n| {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(third(n))
    };

    let ttt = layout(0);
    let ttt1 = layout(1);
    let ttt2 = layout(2);

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block.clone(), ttt[0]);
    f.render_widget(block.clone(), ttt1[0]);
    f.render_widget(block, ttt2[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block.clone(), ttt[1]);
    f.render_widget(block.clone(), ttt1[1]);
    f.render_widget(block, ttt2[1]);
    let block = Block::default().title("Block 3").borders(Borders::ALL);
    f.render_widget(block.clone(), ttt[2]);
    f.render_widget(block.clone(), ttt1[2]);
    f.render_widget(block, ttt2[2]);
}
