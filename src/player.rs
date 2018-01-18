use specs::{BTreeStorage, Fetch, Join, ReadStorage, System, WriteStorage};

use cgmath::{Basis2, Rad, Vector2};
use cgmath::prelude::*;

use input::Input;
use physics::Physical;

#[derive(Component, Debug)]
#[component(BTreeStorage)]
pub struct Player {
    forward_acceleration: f32,
    lateral_acceleration: f32,

    angular_acceleration: Rad<f32>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            forward_acceleration: 1.0 * 10.0,
            lateral_acceleration: 0.25 * 10.0,

            angular_acceleration: Rad::full_turn(),
        }
    }
}

pub struct PlayerController;

impl<'a> System<'a> for PlayerController {
    type SystemData = (
        Fetch<'a, Input>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, player, mut physical) = data;

        for (player, mut physical) in (&player, &mut physical).join() {
            if input.actions.accel_forward {
                let delta_velocity = player.forward_acceleration * input.frame_time;
                physical.vel += physical
                    .orientation
                    .rotate_vector(Vector2::new(0.0, delta_velocity));
            }

            if input.actions.accel_right {
                let delta_velocity = player.lateral_acceleration * input.frame_time;
                physical.vel += physical
                    .orientation
                    .rotate_vector(Vector2::new(delta_velocity, 0.0));
            }

            if input.actions.accel_left {
                let delta_velocity = player.lateral_acceleration * input.frame_time;
                physical.vel += physical
                    .orientation
                    .rotate_vector(Vector2::new(-delta_velocity, 0.0));
            }

            if input.actions.turn_right {
                let delta_angle = -player.angular_acceleration * input.frame_time;
                physical.orientation = physical.orientation * Basis2::from_angle(delta_angle);
            }

            if input.actions.turn_left {
                let delta_angle = player.angular_acceleration * input.frame_time;
                physical.orientation = physical.orientation * Basis2::from_angle(delta_angle);
            }
        }
    }
}
