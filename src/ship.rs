use crate::config::*;

pub struct Ship {
    /// Which lanes the ship is in
    pub lane: usize,

    /// The ship's speed in the y axis
    pub speed: f64,

    /// The ship's distance in the y axis
    pub progress: f64,
}

/// A Ship in the game is made of 7 squares with the following shape:
///
/// Y  X ⟶
/// ↓
/// +-----+         +-----+
/// |  0  |         |  1  |
/// |     |         |     |
/// +-----+         +-----+
///         +-----+
///         |  2  |
///         |     |
///         +-----+
/// +-----+ +-----+ +-----+
/// |  3  | |  4  | |  5  |
/// |     | |     | |     |
/// +-----+ +-----+ +-----+
///         +-----+
///         |  6  |
///         |     |
///         +-----+
///
/// The grid is 3 by 4
impl Ship {
    /// Creates a new ship
    pub fn new(lane: usize, speed: f64) -> Ship {
        Ship {
            lane,
            speed,
            progress: SPAWN_Y,
        }
    }

    /// Moves the ship forward
    pub fn forward(&mut self, dt: f64) {
        self.progress += self.speed * dt;
    }

    /// Returns the ship's squares
    pub fn get_parts(&self) -> [[f64; 4]; 7] {
        // x, y, w, h
        [
            self.make_pixel(0.0, 0.0),
            self.make_pixel(2.0, 0.0),
            self.make_pixel(1.0, 1.0),
            self.make_pixel(0.0, 2.0),
            self.make_pixel(1.0, 2.0),
            self.make_pixel(2.0, 2.0),
            self.make_pixel(1.0, 3.0),
        ]
    }

    /// Returns whether or not this ship is outside the screen
    pub fn is_outside(&self) -> bool {
        self.progress > W_HEIGHT
    }

    /// Returns a rectangle from a pixel grid
    /// To make the ship:
    /// x <- [0, 2]
    /// y <- [0, 3]
    fn make_pixel(&self, x: f64, y: f64) -> [f64; 4] {
        [
            SP_WIDTH * x + SHIP_MARGIN * x + lanes(self.lane) + MARGIN,
            SP_HEIGHT * y + SHIP_MARGIN * y + self.progress,
            SP_WIDTH,
            SP_HEIGHT,
        ]
    }
}
