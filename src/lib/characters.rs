use super::grid::Position;

pub(super) enum Direction {
    Forward,
    Backward,
    Leftward,
    Rightward,
}

pub(super) trait Character {
    fn move_character(&mut self, direction: Direction) {
        let current_position = self.get_position();
        match direction {
            Direction::Forward => current_position.y -= 1,
            Direction::Backward => current_position.y += 1,
            Direction::Leftward => current_position.x -= 1,
            Direction::Rightward => current_position.x += 1,
        };
    }

    fn get_position(&mut self) -> &mut Position;
}

pub struct Player {
    pub(super) position: Position,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl Character for Player {
    fn get_position(&mut self) -> &mut Position {
        &mut self.position
    }
}

pub struct Guard {
    pub(super) position: Position,
}

impl Guard {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl Character for Guard {
    fn get_position(&mut self) -> &mut Position {
        &mut self.position
    }
}