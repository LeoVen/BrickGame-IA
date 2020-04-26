use crate::agent::*;
use crate::config::*;
use crate::player::{KeyState, Player};
use crate::ship::*;
use find_folder::Search;
use piston_window::*;
use rand::*;
use std::f64;

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
        None => return false,
    }
}

/// Runs the game
pub fn run() {
    let mut window: PistonWindow = WindowSettings::new("Brick Game", Size::from(W_RES))
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut path = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    path.push("FiraCode-Regular.ttf");
    let mut glyphs = window.load_font(path).unwrap();

    let mut ships: Vec<Ship> = vec![];
    let mut rng = rand::thread_rng();
    let mut player = Player::new(N_LANES / 2);
    let agent = Agent::new(&mut player);
    let mut speed = SHIP_DEFAULT_SPEED;
    let mut score: usize = 0;
    let mut record: usize = 0;

    // true - player, false - agent
    let mut current_player: bool = true;

    while let Some(e) = window.next() {
        if can_spawn(&ships) {
            // Pick a lane and rotate
            let lane: usize = rng.gen_range(0, N_LANES);
            ships.push(Ship::new(lane, speed));
        }

        // update ships progress
        if let Some(u) = e.update_args() {
            for ship in ships.iter_mut() {
                ship.forward(u.dt);
            }
            // increase speed based on dt
            speed += SHIP_INCREASE_SPEED * u.dt;
        }

        // delete ships outside of the screen
        let prev_len = ships.len();
        ships = ships
            .into_iter()
            .filter(|ship| !ship.is_outside())
            .collect();

        score += prev_len - ships.len();

        // check collision
        if agent.player.check_collision(&ships) {
            // game over
            ships.clear();
            agent.player.reset(N_LANES / 2);
            speed = SHIP_DEFAULT_SPEED;
            if score > record {
                record = score;
            }
            score = 0;
        }

        // check for enter key to change the player
        if let Some(release_args) = e.release_args() {
            match release_args {
                Button::Keyboard(Key::Return) => current_player = !current_player,
                _ => {}
            }
        }

        // draw things
        // context, graphics, device
        window.draw_2d(&e, |c, g, d| {
            clear(BG_COLOR, g);

            // draw ships
            for ship in ships.iter() {
                for rect in ship.get_parts().iter() {
                    // [r, g, b, a], [x, y, w, h], t, g
                    rectangle(SHIP_COLOR, *rect, c.transform, g);
                }
            }
            // draw player
            for rect in agent.player.get_parts().iter() {
                rectangle(PLAYER_COLOR, *rect, c.transform, g);
            }

            // update current player
            let curr_player = if current_player {
                "Player"
            } else {
                "Intelligent Agent"
            };
            Text::new(16)
                .draw(
                    &curr_player,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 20.0),
                    g,
                )
                .unwrap();
            // update score
            Text::new(16)
                .draw(
                    &format!("Score {}[{}]", score, record),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 40.0),
                    g,
                )
                .unwrap();
            Text::new(16)
                .draw(
                    &format!("Speed {:.0}", speed),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 60.0),
                    g,
                )
                .unwrap();

            glyphs.factory.encoder.flush(d);
        });

        // Capture movement, either from keyboard or the intelligent agent
        if current_player {
            // Player is playing
            if let Some(press_args) = e.press_args() {
                match press_args {
                    Button::Keyboard(Key::Left) => {
                        agent.player.delta_left(KeyState::Pressed);
                    }
                    Button::Keyboard(Key::Right) => {
                        agent.player.delta_right(KeyState::Pressed);
                    }
                    _ => {}
                }
            }
            if let Some(release_args) = e.release_args() {
                match release_args {
                    Button::Keyboard(Key::Left) => {
                        agent.player.delta_left(KeyState::NotPressed);
                    }
                    Button::Keyboard(Key::Right) => {
                        agent.player.delta_right(KeyState::NotPressed);
                    }
                    _ => (),
                }
            }
        } else {
            // Agent is playing
            agent.player.change_lane(agent.next_move(&ships));
        }
    }
}
