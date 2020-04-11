use std;

use specs::storage::BTreeStorage;
use specs::{Component, Join, Read, ReadStorage, System, WriteStorage};

use crate::na::Vector2;

use crate::input::Input;
use crate::physics::Physical;

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Player {
    forward_acceleration: f32,
    lateral_acceleration: f32,

    angular_acceleration: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            forward_acceleration: 0.50 * 10.0,
            lateral_acceleration: 0.25 * 10.0,

            angular_acceleration: 2.0 * std::f32::consts::PI,
        }
    }
}

pub struct PlayerController;

impl<'a> System<'a> for PlayerController {
    type SystemData = (
        Read<'a, Input>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, player, mut physical) = data;

        for (player, physical) in (&player, &mut physical).join() {
            if input.actions.accel_forward {
                physical.add_relative_pulse(-player.forward_acceleration * Vector2::y());
            }

            if input.actions.accel_right {
                physical.add_relative_pulse(player.lateral_acceleration * Vector2::x());
            }

            if input.actions.accel_left {
                physical.add_relative_pulse(-player.lateral_acceleration * Vector2::x());
            }

            if input.actions.turn_right {
                physical.add_angular_pulse(-player.angular_acceleration);
            }

            if input.actions.turn_left {
                physical.add_angular_pulse(player.angular_acceleration);
            }
        }
    }
}
