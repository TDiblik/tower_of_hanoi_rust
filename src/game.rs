use std::{collections::VecDeque, rc::Rc};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub struct Game {
    pub left_tower: Tower,
    pub middle_tower: Tower,
    pub right_tower: Tower,

    pub pointing_to_tower: PossibleTowers,
    pub last_selected_tower: Option<PossibleTowers>,

    pub is_finished: bool,
}

const NUMBER_OF_DISCKS: usize = 4;
impl Game {
    pub fn new() -> Self {
        Game {
            left_tower: Tower::new(PossibleTowers::Left, VecDeque::new()),
            middle_tower: Tower::new(
                PossibleTowers::Middle,
                vec![
                    TowerDisck::new(25),
                    TowerDisck::new(45),
                    TowerDisck::new(65),
                    TowerDisck::new(85),
                ]
                .into(),
            ),
            right_tower: Tower::new(PossibleTowers::Right, VecDeque::new()),

            pointing_to_tower: PossibleTowers::Middle,
            last_selected_tower: None,
            is_finished: false,
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

    pub fn change_selection(&mut self) {
        match self.last_selected_tower {
            Some(last_selection) => {
                let last_selected_tower = self.tower_enum_to_ref(last_selection);
                let top_disck = last_selected_tower.discks.pop_front().unwrap();

                let current_tower = self.tower_enum_to_ref(self.pointing_to_tower);
                current_tower.discks.push_front(top_disck);

                self.last_selected_tower = None;
            }
            None => {
                let current_tower = self.tower_enum_to_ref(self.pointing_to_tower);
                if !current_tower.discks.is_empty() {
                    self.last_selected_tower = Some(self.pointing_to_tower);
                } else {
                    self.last_selected_tower = None;
                }
            }
        }
    }

    fn tower_enum_to_ref(&mut self, tower_enum: PossibleTowers) -> &mut Tower {
        match tower_enum {
            PossibleTowers::Left => &mut self.left_tower,
            PossibleTowers::Middle => &mut self.middle_tower,
            PossibleTowers::Right => &mut self.right_tower,
        }
    }

    pub fn check_win_conditions(&mut self) {
        self.is_finished = self.check_win_conditions_for_one_tower(&self.left_tower)
            || self.check_win_conditions_for_one_tower(&self.right_tower);
    }

    fn check_win_conditions_for_one_tower(&self, t: &Tower) -> bool {
        let len = t.discks.len();
        if len == 0 || len != NUMBER_OF_DISCKS {
            return false;
        }

        let mut previous = t.discks.front().unwrap().width_percent;
        for current_disck in t.discks.iter().skip(1) {
            let current = current_disck.width_percent;
            if previous > current {
                return false;
            }
            previous = current;
        }

        true
    }
}

pub struct Tower {
    position: PossibleTowers,
    discks: VecDeque<TowerDisck>,
}
impl Tower {
    pub fn new(position: PossibleTowers, discks: VecDeque<TowerDisck>) -> Self {
        Tower { position, discks }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, game: &Game, container: &Rc<[Rect]>) {
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

            let mut tower_disck = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            if i == 0
                && game.last_selected_tower.is_some()
                && self.position == game.last_selected_tower.unwrap()
            {
                tower_disck = tower_disck.border_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );
            }

            f.render_widget(tower_disck, block_layout[1]);
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

#[derive(Copy, Clone, PartialEq)]
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
