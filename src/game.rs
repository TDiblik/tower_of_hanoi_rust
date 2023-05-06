use std::rc::Rc;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub struct Game {
    pub pointing_to_tower: PossibleTowers,
    pub left_tower: Tower,
    pub middle_tower: Tower,
    pub right_tower: Tower,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pointing_to_tower: PossibleTowers::Middle,
            left_tower: Tower::new(PossibleTowers::Left),
            middle_tower: Tower::new(PossibleTowers::Middle),
            right_tower: Tower::new(PossibleTowers::Right),
        }
    }

    pub fn point_to_next(&mut self) {
        self.pointing_to_tower = match self.pointing_to_tower {
            PossibleTowers::Left => PossibleTowers::Middle,
            _ => PossibleTowers::Right,
        };
    }

    pub fn point_to_previous(&mut self) {
        self.pointing_to_tower = match self.pointing_to_tower {
            PossibleTowers::Right => PossibleTowers::Middle,
            _ => PossibleTowers::Left,
        };
    }
}

pub struct Tower {
    position: PossibleTowers,
    discks: Vec<TowerDisck>,
}
impl Tower {
    pub fn new(position: PossibleTowers) -> Self {
        Tower {
            position,
            discks: vec![
                TowerDisck::new(30),
                TowerDisck::new(50),
                TowerDisck::new(60),
                TowerDisck::new(80),
            ],
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, container: &Rc<[Rect]>) {
        let tower_border = Block::default().borders(Borders::ALL);
        f.render_widget(tower_border, container[self.position.into_game_index()]);

        let tower_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
            .horizontal_margin(5)
            .vertical_margin(1)
            .split(container[self.position.into_game_index()]);

        let stick_margin = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(tower_layout[0]);
        let stick_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(48),
                Constraint::Percentage(4),
                Constraint::Percentage(48),
            ])
            .split(stick_margin[1]);
        let stick_block = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(stick_block, stick_layout[1]);

        let discks_len = self.discks.len();
        let mut discks_constraints = Vec::with_capacity(discks_len + 1);
        discks_constraints.push(Constraint::Percentage(100));
        for _ in 0..discks_len {
            discks_constraints.push(Constraint::Min(2))
        }

        let discks_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(discks_constraints)
            .split(tower_layout[0]);

        let tower_disck = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        for i in 0..discks_len {
            let disck_width_percent = self.discks[i].width_percent;

            let percentage_padding = 50 - disck_width_percent / 2;
            let block_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(percentage_padding),
                    Constraint::Percentage(disck_width_percent),
                    Constraint::Percentage(percentage_padding),
                ])
                .split(discks_layout[1 + i]);
            f.render_widget(tower_disck.clone(), block_layout[1]);
        }

        let tower_bottom_block =
            Block::default().style(Style::default().fg(Color::White).bg(Color::White));
        f.render_widget(tower_bottom_block, tower_layout[1]);
    }
}

pub struct TowerDisck {
    width_percent: u16,
}

impl TowerDisck {
    pub fn new(width_percent: u16) -> Self {
        assert!(
            width_percent <= 100,
            "Block width cannot be more than 100. Change and recompile."
        );
        TowerDisck { width_percent }
    }
}

#[derive(Copy, Clone)]
pub enum PossibleTowers {
    Left = 0,
    Middle = 1,
    Right = 2,
}

impl PossibleTowers {
    pub fn into_game_index(self) -> usize {
        1 + self as usize
    }
}
