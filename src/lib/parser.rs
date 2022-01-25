use std::{fs::read_to_string, path::Path};

use super::{grid::Position, square::WallState};

#[derive(Debug)]
pub struct GameFile {
    pub grid_size: (u8, u8),
    pub player_position: Position,
    pub guard_position: Position,
    pub custom_squares: Vec<(Position, WallState)>,
}

impl GameFile {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file_contents = read_to_string(path)?;
        let declarations = Self::get_declarations(&file_contents);
        let get_declaration_value =
            |key| declarations.iter().filter(|x| x.0 == key).nth(0).unwrap().1;
        let custom_squares_declarations = declarations
            .iter()
            .filter(|x| x.0.contains("("))
            .collect::<Vec<&(&str, &str)>>();
        Ok(Self {
            grid_size: Self::get_grid_size(get_declaration_value("grid"))?,
            player_position: Self::get_position(get_declaration_value("player"))?,
            guard_position: Self::get_position(get_declaration_value("guard"))?,
            custom_squares: Self::get_custom_squares(custom_squares_declarations)?,
        })
    }

    fn get_declarations(file_contents: &String) -> Vec<(&str, &str)> {
        file_contents
            .lines()
            .filter(|x| !x.contains("#") && !x.is_empty())
            .map(|x| {
                let mut split = x.split("=");
                (split.next().unwrap().trim(), split.next().unwrap().trim())
            })
            .collect::<Vec<(&str, &str)>>()
    }

    fn get_grid_size(value: &str) -> Result<(u8, u8), Box<dyn std::error::Error>> {
        let mut split = value.split("x");
        Ok((
            split.next().unwrap().parse::<u8>()?,
            split.next().unwrap().parse::<u8>()?,
        ))
    }

    fn get_position(value: &str) -> Result<Position, Box<dyn std::error::Error>> {
        let split = &mut value[1..value.len() - 1].split(",");
        Ok(Position::new(
            split.next().unwrap().parse::<u8>()?,
            split.next().unwrap().parse::<u8>()?,
        ))
    }

    fn get_wallstate(value: &str) -> WallState {
        let mut wallstate = WallState::default();
        if value.contains("N") {
            wallstate.wall_n = true;
        }
        if value.contains("S") {
            wallstate.wall_s = true;
        }
        if value.contains("W") {
            wallstate.wall_w = true;
        }
        if value.contains("E") {
            wallstate.wall_e = true;
        }
        wallstate
    }

    fn get_custom_squares(
        custom_squares_declaration: Vec<&(&str, &str)>,
    ) -> Result<Vec<(Position, WallState)>, Box<dyn std::error::Error>> {
        let mut res = Vec::with_capacity(custom_squares_declaration.len());
        for declaration in custom_squares_declaration {
            res.push((
                Self::get_position(declaration.0)?,
                Self::get_wallstate(declaration.1),
            ))
        }
        Ok(res)
    }
}
