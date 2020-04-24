use crate::config::*;

pub struct Player {
    lane: usize,
    left: KeyState,
    right: KeyState,
}

pub enum KeyState {
    Pressed,
    NotPressed,
}

/// A Player is the opposite of a ship
///
/// Y  X ⟶
/// ↓
///         +-----+
///         |  0  |
///         |     |
///         +-----+
/// +-----+ +-----+ +-----+
/// |  1  | |  2  | |  3  |
/// |     | |     | |     |
/// +-----+ +-----+ +-----+
///         +-----+
///         |  4  |
///         |     |
///         +-----+
/// +-----+         +-----+
/// |  5  |         |  6  |
/// |     |         |     |
/// +-----+         +-----+
///
/// The grid is 3 by 4
impl Player {

    pub fn new(initial_lane: usize) -> Player {
        Player {
            lane: initial_lane,
            left: KeyState::NotPressed,
            right: KeyState::NotPressed,
        }
    }

    pub fn delta_left(&mut self, state: KeyState) {
        match (&self.left, state) {
            (&KeyState::Pressed, KeyState::NotPressed) => {
                self.left = KeyState::NotPressed;
                self.change_lane(-1);
            },
            (&KeyState::NotPressed, KeyState::Pressed) => {
                self.left = KeyState::Pressed;
            }
            _ => {}
        }
    }

    pub fn delta_right(&mut self, state: KeyState) {
        match (&self.right, state) {
            (&KeyState::Pressed, KeyState::NotPressed) => {
                self.right = KeyState::NotPressed;
                self.change_lane(1);
            },
            (&KeyState::NotPressed, KeyState::Pressed) => {
                self.right = KeyState::Pressed;
            }
            _ => {}
        }
    }

    pub fn change_lane(&mut self, change: i32) {
        if change < 0 {
            // go left
            if self.lane == 0 {
                return;
            }
            self.lane = (self.lane as i32 + change) as usize;
        } else {
            if self.lane == N_LANES - 1 {
                return;
            }
            self.lane = self.lane + change as usize;
        }
    }

    /// Returns the ship's squares
    pub fn get_parts(&self) -> [[f64; 4]; 7] {
        // x, y, w, h
        [
            self.make_pixel(1.0, 0.0),
            self.make_pixel(0.0, 1.0),
            self.make_pixel(1.0, 1.0),
            self.make_pixel(2.0, 1.0),
            self.make_pixel(1.0, 2.0),
            self.make_pixel(0.0, 3.0),
            self.make_pixel(2.0, 3.0),
        ]
    }

    fn make_pixel(&self, x: f64, y: f64) -> [f64; 4] {
        [
            SP_WIDTH * x + SHIP_MARGIN * x + lanes(self.lane) + MARGIN,
            W_HEIGHT - SHIP_HEIGHT + SP_HEIGHT * y + SHIP_MARGIN * y - MARGIN * 2.0,
            SP_WIDTH,
            SP_HEIGHT,
        ]
    }
}
