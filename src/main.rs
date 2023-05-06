mod game;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Block,
    Frame, Terminal,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut game = Game::new();
    loop {
        terminal.draw(|f| ui(f, &game))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Left => game.point_to_previous(),
                    KeyCode::Right => game.point_to_next(),
                    _ => (),
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, game: &Game) {
    let overlay = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(f.size());

    // Render towers
    let tower_container_constraints = [
        Constraint::Percentage(5),
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(5),
    ];
    let tower_container_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(tower_container_constraints.as_ref())
        .split(overlay[0]);

    game.left_tower.render(f, &tower_container_chunks);
    game.middle_tower.render(f, &tower_container_chunks);
    game.right_tower.render(f, &tower_container_chunks);

    // Render pointer to towers
    let pointer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(tower_container_constraints.as_ref())
        .split(overlay[1]);

    let pointing_block = Block::default()
        .title(Span::styled(
            "^",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(
        pointing_block,
        pointer_chunks[game.pointing_to_tower.into_game_index()],
    );
}
