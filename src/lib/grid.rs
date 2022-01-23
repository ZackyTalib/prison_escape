use super::square::{Square, WallState};

#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

pub struct Grid {
    pub(super) width: u8,
    pub(super) height: u8,
    squares: Vec<Square>,
}

impl Grid {
    fn create_standard_square(width: u8, height: u8, position: Position) -> Square {
        let wall_state = match (position.x, position.y) {
            (0, 0) => WallState::top_left(),
            (x, 0) if x == width - 1 => WallState::top_right(),
            (0, y) if y == height - 1 => WallState::bottom_left(),
            (x, y) if x == width - 1 && y == height - 1 => WallState::bottom_right(),
            (0, _) => WallState::left(),
            (x, _) if x == width - 1 => WallState::right(),
            (_, 0) => WallState::top(),
            (_, y) if y == height - 1 => WallState::bottom(),
            _ => WallState::default(),
        };
        return Square::new(wall_state);
    }

    pub fn new(width: u8, height: u8) -> Self {
        let mut squares = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let current_position = Position::new(x, y);
                let square = Self::create_standard_square(width, height, current_position);
                squares.push(square);
            }
        }
        Grid {
            width,
            height,
            squares,
        }
    }

    pub(super) fn get_square(&self, position: &Position) -> Option<&Square> {
        self.squares
            .get((position.y * self.width + position.x) as usize)
    }

    pub fn set_square_wallstate(
        &mut self,
        position: &Position,
        wallstate: WallState,
    ) -> Result<(), &str> {
        let square = self
            .squares
            .get_mut((position.y * self.width + position.x) as usize)
            .ok_or("square does not exist")?;
        square.wall_state = wallstate;
        Ok(())
    }
}