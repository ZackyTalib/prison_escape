use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, SolidSource, Source};

use crate::lib::grid::Position;

use super::{
    characters::{Guard, Player},
    grid::Grid,
    square::Square,
};

pub(super) fn draw_grid(grid: &Grid, player: &Player, guard: &Guard) {
    print!("\x1B[2J\x1B[1;1H"); // clear the terminal
    let mut grid_str = String::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            grid_str += match Position::new(x, y) {
                p if p == player.position => "\x1b[32m P \x1b[0m".to_string(),
                p if p == guard.position => "\x1b[34m G \x1b[0m".to_string(),
                _ => draw_square(grid.get_square(&Position::new(x, y)).unwrap()),
            }
            .as_str();
        }
        grid_str += "\n";
    }
    println!("{}", grid_str);
}

pub(super) fn draw_square(square: &Square) -> String {
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

pub(super) fn draw_escaped() {
    print!("\x1B[2J\x1B[1;1H"); // clear the terminal
    println!("YOU HAVE ESCAPED");
}

struct GraphicsManager {
    window: Window,
    draw_target: DrawTarget,
    width: usize,
    height: usize,
}

impl GraphicsManager {
    pub fn new(width: usize, height: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new("Prison Escape", width, height, WindowOptions::default())?;
        let draw_target = DrawTarget::new(width as i32, height as i32);
        Ok(Self {
            window,
            draw_target,
            width,
            height,
        })
    }

    pub fn draw_grid(&mut self, grid: &Grid) {
        let square_width = self.width / grid.width as usize;
        let square_height = self.height / grid.height as usize;
        let square_color = |i: usize, j: usize| match ((i % 2) + (j % 2)) % 2 {
            0 => &Source::Solid(SolidSource {
                r: 166,
                g: 204,
                b: 211,
                a: 255,
            }),
            1 => &Source::Solid(SolidSource {
                r: 126,
                g: 147,
                b: 160,
                a: 255,
            }),
            _ => unreachable!(),
        };
        for row in 0..grid.height as usize {
            for cell in 0..grid.width as usize {
                self.draw_target.fill_rect(
                    (cell * square_width) as f32,
                    (row * square_height) as f32,
                    square_width as f32,
                    square_height as f32,
                    square_color(cell, row),
                    &DrawOptions::default(),
                );
            }
        }
    }

    pub fn update_graphics(&mut self) -> Result<(), minifb::Error> {
        self.window
            .update_with_buffer(self.draw_target.get_data(), self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::grid::Grid;

    use super::GraphicsManager;

    #[test]
    fn main() {
        let mut graphics_manager = GraphicsManager::new(500, 500).unwrap();
        let grid = Grid::new(8, 8);
        graphics_manager.draw_grid(&grid);
        loop {
            graphics_manager.update_graphics().unwrap();
        }
    }
}
