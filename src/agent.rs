use crate::config::*;
use crate::player::{Player, PlayerMovement};
use crate::ship::Ship;
use std::f64;

/// An intelligent agent with rules to play the game
pub struct Agent<'a> {
    /// The player handled by the agent
    pub player: &'a mut Player,
}

impl<'a> Agent<'a> {
    /// Convenience method to initiate a new Agent that will control a given
    /// player
    pub fn new(player: &'a mut Player) -> Self {
        Agent { player }
    }

    /// Calculates the next movement for this agent given the layout of where
    /// each ship is. The ships here are the environment input.
    pub fn next_move(&self, ships: &Vec<Ship>) -> PlayerMovement {
        // The layout of all lanes. This will capture the distance of the
        // closest ship to the player
        let mut lane_layout: [f64; N_LANES] = [0.0; N_LANES];

        for ship in ships.iter() {
            // The ship progress is a value that represents its distance from
            // the top of the screen. A greater value means a closer distance
            // to the player.
            if ship.progress > lane_layout[ship.lane] {
                lane_layout[ship.lane] = ship.progress;
            }
        }

        // If there is no one in this lane, stay in it
        if lane_layout[self.player.lane] == 0.0 {
            return PlayerMovement::NoMove;
        }

        // Someone is coming towards us. Check if we need to make a move. The
        // best place to move is where the enemy is furthest from us. This means
        // that the ship's progress needs to be as negative as possible.

        // First set the distances to MIN since we think that there are no
        // enemies there yet.
        // distances[0] - Left lane
        // distances[1] - Current lane
        // distances[2] - Right lane
        let mut distances: [f64; 3] = [f64::MIN; 3];

        if self.player.lane == 0 {
            // We can only move right
            distances[0] = f64::MAX; // This means we can never move to the left
        } else if self.player.lane == N_LANES - 1 {
            // We can only move left
            distances[2] = f64::MAX; // This means we can never move to the right
        }

        // Calculate the distances
        for i in 0..distances.len() {
            if distances[i] != f64::MAX {
                distances[i] =
                    self.closest(&ships, (self.player.lane as i32 + (i as i32 - 1)) as usize);
            }
        }

        if distances[1] < distances[0] && distances[1] < distances[2] {
            // Basically where we currently are, is better than moving either
            // left or right
            return PlayerMovement::NoMove;
        }

        if distances[0] < distances[2] {
            // Move left
            return PlayerMovement::MoveLeft;
        } else if distances[2] < distances[0] {
            return PlayerMovement::MoveRight;
        }

        PlayerMovement::NoMove
    }

    /// Returns the closes ship to the player from a given lane
    fn closest(&self, ships: &Vec<Ship>, lane: usize) -> f64 {
        let mut max = f64::MIN;

        for ship in ships {
            if ship.lane == lane && ship.progress > max {
                max = ship.progress;
            }
        }

        max
    }
}
