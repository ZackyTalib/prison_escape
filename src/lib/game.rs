use std::io::stdin;

use crate::lib::characters::Direction;

use super::{characters::{Player, Guard, Character}, grid::{Grid, Position}, square::Square};

pub struct Game {
    player: Player,
    grid: Grid,
    guard: Guard,
    escaped: bool,
}

impl Game {
    pub fn new(player: Player, guard: Guard, grid: Grid) -> Self {
        Self {
            player,
            grid,
            escaped: false,
            guard,
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "W" => self.move_player(Direction::Forward),
                "A" => self.move_player(Direction::Leftward),
                "S" => self.move_player(Direction::Backward),
                "D" => self.move_player(Direction::Rightward),
                _ => (),
            }
            print!("\x1B[2J\x1B[1;1H"); // clear the terminal
            if self.escaped {
                println!("YOU HAVE ESCAPED");
                break;
            } else {
                self.draw_grid();
            }
        }
    }

    fn movement_possible<F, P>(
        &self,
        overflow_check: bool,
        axis_position: u8,
        current_wall_condition: bool,
        next_wall_condition: F,
        mut new_position: P,
    ) -> (bool, bool)
    where
        F: Fn(&Square) -> bool,
        P: FnMut(&mut Position),
    {
        let next_square = if axis_position == 0 && overflow_check {
            None
        } else {
            let mut position = Position::new(self.player.position.x, self.player.position.y);
            new_position(&mut position);
            self.grid.get_square(&position)
        };
        (
            !current_wall_condition
                && next_square.map_or_else(|| true, |x| !next_wall_condition(x)),
            next_square.map_or_else(|| true, |_| false),
        )
    }

    fn move_player(&mut self, direction: Direction) {
        let current_square = self.grid.get_square(&self.player.position).unwrap();
        let (movement_possible, out_of_bounds) = match direction {
            Direction::Forward => self.movement_possible(
                true,
                self.player.position.y,
                current_square.wall_state.wall_n,
                |x| x.wall_state.wall_s,
                |p| p.y -= 1,
            ),
            Direction::Backward => self.movement_possible(
                false,
                self.player.position.y,
                current_square.wall_state.wall_s,
                |x| x.wall_state.wall_n,
                |p| p.y += 1,
            ),
            Direction::Leftward => self.movement_possible(
                true,
                self.player.position.x,
                current_square.wall_state.wall_w,
                |x| x.wall_state.wall_e,
                |p| p.x -= 1,
            ),
            Direction::Rightward => self.movement_possible(
                false,
                self.player.position.x,
                current_square.wall_state.wall_e,
                |x| x.wall_state.wall_w,
                |p| p.x += 1,
            ),
        };
        if movement_possible && out_of_bounds {
            self.escaped = true;
        }
        if movement_possible {
            self.player.move_character(direction);
        }
    }

    fn draw_grid(&self) {
        if self.escaped {
            println!("YOU HAVE ESCAPED SUCCESSFULLY");
            return;
        }
        let mut grid_str = String::new();
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                grid_str += match Position::new(x, y) {
                    p if p == self.player.position => "\x1b[32m P \x1b[0m".to_string(),
                    p if p == self.guard.position => "\x1b[34m G \x1b[0m".to_string(),
                    _ => Self::draw_square(self.grid.get_square(&Position::new(x, y)).unwrap()),
                }
                .as_str();
            }
            grid_str += "\n";
        }
        println!("{}", grid_str);
    }

    fn draw_square(square: &Square) -> String {
        let mut wall_count: u8 = 0;
        if square.wall_state.wall_n {
            wall_count += 1;
        }
        if square.wall_state.wall_s {
            wall_count += 2;
        }
        if square.wall_state.wall_w {
            wall_count += 3;
        }
        if square.wall_state.wall_e {
            wall_count += 4;
        }
        format!(" {} ", wall_count)
    }
}