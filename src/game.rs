use std::f64;
use crate::config::*;
use crate::ship::*;
use piston_window::{PistonWindow, WindowSettings, UpdateEvent, Size, clear, rectangle};
use rand::*;
use crate::player::{Player, KeyState};
use piston::input::Button;
use piston::input::keyboard::Key;
use piston::input::*;

/// Checks if a new ship can be spawned
pub fn can_spawn(ships: &Vec<Ship>) -> bool {
    if ships.len() == 0 {
        return true;
    } else if ships.len() >= MAX_SHIPS {
        return false;
    }

    // find the ship closest to the top of the screen
    let mut dist = f64::MAX;
    let mut last_ship = Option::<&Ship>::None;

    for ship in ships.iter() {
        if ship.progress < dist {
            last_ship = Some(&ship);
            dist = ship.progress;
        }
    }

    match last_ship {
        Some(last_ship) => {
            if last_ship.progress > SHIP_HEIGHT * GAME_DIFFICULTY {
                return true;
            }
            false
        }
        None => return false
    }
}

pub fn run() {
    let mut window: PistonWindow = WindowSettings::new("Brick Game", Size::from(W_RES))
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut ships: Vec<Ship> = vec![];
    let mut rng = rand::thread_rng();
    let mut player = Player::new(N_LANES / 2);

    for i in 0..N_LANES {
        ships.push(Ship::new(i, SHIP_DEFAULT_SPEED));
    }

    while let Some(e) = window.next() {
        if can_spawn(&ships) {
            // Pick a lane and rotate
            let lane: usize = rng.gen_range(0, N_LANES);
            ships.push(Ship::new(lane, SHIP_DEFAULT_SPEED));
        }

        // update ships progress
        if let Some(u) = e.update_args() {
            for ship in ships.iter_mut() {
                ship.forward(u.dt);
            }
        }

        // delete ships outside of the screen
        ships = ships
            .into_iter()
            .filter(|ship| !ship.is_outside())
            .collect();

        // draw things
        // context, graphics, device
        window.draw_2d(&e, |c, g, _d| {
            clear(BG_COLOR, g);

            // draw ships
            for ship in ships.iter() {
                for rect in ship.get_parts().iter() {
                    // [r, g, b, a], [x, y, w, h], t, g
                    rectangle(SHIP_COLOR, *rect, c.transform, g);
                }
            }

            // draw player
            for rect in player.get_parts().iter() {
                rectangle(PLAYER_COLOR, *rect, c.transform, g);
            }
        });

        // Capture player movement
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Left) => player.delta_left(KeyState::Pressed),
                Button::Keyboard(Key::Right) => player.delta_right(KeyState::Pressed),
                _ => {}
            }
        }

        if let Some(release_args) = e.release_args() {
            match release_args {
                Button::Keyboard(Key::Left) => player.delta_left(KeyState::NotPressed),
                Button::Keyboard(Key::Right) => player.delta_right(KeyState::NotPressed),
                _ => (),
            }
        }
    }
}
