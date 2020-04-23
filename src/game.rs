use sa;
use piston_window::*;

// Constants
const W_WIDTH : f64 = 480.0;
const W_HEIGHT: f64 = 640.0;
const W_RES : (f64, f64) = (W_WIDTH, W_HEIGHT);

// Make sure that we can divide the screen correctly
sa::const_assert!(W_WIDTH as usize % 3 == 0);

const LANE_WIDTH : f64 = W_WIDTH / 3.0;
// Where each lane starts and ends
// |          |          |          |
// 0         10         20         30
const LANES : [f64; 4] = [0.0, LANE_WIDTH, LANE_WIDTH * 2.0, LANE_WIDTH * 3.0];

// Colors
const SHIP_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const BG_COLOR : [f32; 4] = [0.8; 4];
const MARGIN : f64 = 10.0;

pub fn run() {
    println!("Lane Width -> {:?}", LANE_WIDTH);
    println!("Lane Width Boundaries -> {:?}", LANES);
    println!("Possible Lane Width -> {:?}", LANE_WIDTH - 2.0 * MARGIN);

    let mut window: PistonWindow = WindowSettings::new("Brick Game", Size::from(W_RES))
        .resizable(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        // context, graphics, device
        window.draw_2d(&event, |c, g, _d| {
            clear(BG_COLOR, g);
            // params
            // [r, g, b, a], [x, y, w, h], t, g
            rectangle(SHIP_COLOR, [LANES[0] + MARGIN, MARGIN, LANE_WIDTH - 2.0 * MARGIN, LANE_WIDTH], c.transform, g);
            rectangle(SHIP_COLOR, [LANES[1] + MARGIN, MARGIN, LANE_WIDTH - 2.0 * MARGIN, LANE_WIDTH], c.transform, g);
            rectangle(SHIP_COLOR, [LANES[2] + MARGIN, MARGIN, LANE_WIDTH - 2.0 * MARGIN, LANE_WIDTH], c.transform, g);
        });
    }
}
