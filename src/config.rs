use sa;

// -- Configurable
// Screen sizes
pub const W_WIDTH: f64 = 480.0;
pub const W_HEIGHT: f64 = 640.0;
pub const W_RES: (f64, f64) = (W_WIDTH, W_HEIGHT);
// Game
// Total number of lanes
pub const N_LANES: usize = 3;
// Margin between lanes
pub const MARGIN: f64 = 5.0;
pub const LANE_WIDTH: f64 = W_WIDTH / N_LANES as f64;
pub const USEABLE_LANE: f64 = LANE_WIDTH - 2.0 * MARGIN;
// Ship
pub const SHIP_HEIGHT: f64 = LANE_WIDTH * 0.9;
pub const SHIP_DEFAULT_SPEED: f64 = 150.0;
// Ship speed increase. Set to 0 to not increase
pub const SHIP_INCREASE_SPEED: f64 = 5.0;
// The margin between the ship's squares
pub const SHIP_MARGIN: f64 = 2.0;
// How above the screen should ships start to spawn
pub const SPAWN_Y: f64 = -(SHIP_HEIGHT * 1.1);
// Color
pub const BG_COLOR: [f32; 4] = [0.8; 4];
pub const SHIP_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
// Game difficulty
// This sets the minimum distance between ships
pub const DIFFICULTIES: [f64; 4] = [2.0, 1.5, 1.0, 0.5]; // easy, medium, hard, ?
pub const GAME_DIFFICULTY: f64 = DIFFICULTIES[1];
// Player
pub const PLAYER_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

// Non-configurable
// Ship Pixel width and height
pub const SP_WIDTH: f64 = USEABLE_LANE / 3.0 - 2.0 * SHIP_MARGIN;
pub const SP_HEIGHT: f64 = SHIP_HEIGHT / 4.0;
pub const MAX_SHIPS: usize = N_LANES;

// Make sure that we can divide the screen correctly AND have enough lanes
sa::const_assert!(W_WIDTH as usize % N_LANES == 0);
sa::const_assert!(N_LANES > 1);

// Utility
pub fn lanes(lane: usize) -> f64 {
    LANE_WIDTH as f64 * lane as f64 + MARGIN
}
