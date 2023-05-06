pub struct Game {
    pub pointing_to_tower: PossibleTowers,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pointing_to_tower: PossibleTowers::Left,
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

#[derive(Copy, Clone)]
pub enum PossibleTowers {
    Left = 0,
    Middle = 1,
    Right = 2,
}
