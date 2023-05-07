mod game;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
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
            // Universal keys
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('r') | KeyCode::Char('R') => game = Game::new(),
                _ => {}
            }

            // Game keys
            if key.kind == KeyEventKind::Press && !game.is_finished {
                match key.code {
                    KeyCode::Left => game.point_to_previous(),
                    KeyCode::Right => game.point_to_next(),
                    KeyCode::Enter => {
                        game.change_selection();
                        game.check_win_conditions();
                    }
                    _ => (),
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, game: &Game) {
    let f_size = f.size();

    let overlay = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(f_size);

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

    game.left_tower.render(f, game, &tower_container_chunks);
    game.middle_tower.render(f, game, &tower_container_chunks);
    game.right_tower.render(f, game, &tower_container_chunks);

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

    // Helper menu
    let helper_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(f_size);

    let helper_text = Paragraph::new(Span::styled(
        "L/R Arrow => move ; Enter => Select ; R => Restart ; Q => Quit",
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Right);
    f.render_widget(helper_text, helper_chunks[1]);

    // Win popup
    if game.is_finished {
        let popup_block = Block::default().title("You win!").borders(Borders::ALL);
        let popup_area = centered_rect(60, 20, f_size);
        f.render_widget(Clear, popup_area);
        f.render_widget(popup_block, popup_area);

        let popup_text = Paragraph::new(Span::styled(
            "Congratulations! Press Q to quit or R to restart.",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center);
        let popup_text_area = centered_rect(90, 20, popup_area);

        f.render_widget(popup_text, popup_text_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let center_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(center_layout[1])[1]
}
