use std::f64;
use crate::config::*;
use crate::ship::*;
use piston_window::*;
use rand::*;

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

        ships = ships
            .into_iter()
            .filter(|ship| !ship.is_outside())
            .collect();

        // context, graphics, device
        window.draw_2d(&e, |c, g, _d| {
            clear(BG_COLOR, g);
            for ship in ships.iter() {
                for rect in ship.get_parts().iter() {
                    // [r, g, b, a], [x, y, w, h], t, g
                    rectangle(SHIP_COLOR, *rect, c.transform, g);
                }
            }
        });
    }
}
