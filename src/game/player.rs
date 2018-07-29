use super::map::{MAP_HEIGHT, MAP_WIDTH};
use util::vector::Vector2D;

pub struct Player {
    position: Vector2D<i32>,
}

impl Player {
    pub fn new() -> Player {
        let local_x = 20;
        let local_y = 25;

        Player {
            position: Vector2D::new(10000 * MAP_WIDTH + local_x, 10000 * MAP_HEIGHT + local_y),
        }
    }

    pub fn position(&self) -> &Vector2D<i32> {
        &self.position
    }

    pub fn move_position(&mut self, x_move: i32, y_move: i32) {
        self.position.x += x_move;
        self.position.y += y_move;
    }
}
