use std;

use specs::storage::BTreeStorage;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use nalgebra::{UnitComplex, Vector2};

use input::Input;
use physics::Physical;

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
            forward_acceleration: 1.0 * 10.0,
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

        for (player, mut physical) in (&player, &mut physical).join() {
            if input.actions.accel_forward {
                let delta_velocity = player.forward_acceleration * input.frame_time;
                physical.vel += physical.orientation * Vector2::new(0.0, delta_velocity);
            }

            if input.actions.accel_right {
                let delta_velocity = player.lateral_acceleration * input.frame_time;
                physical.vel += physical.orientation * Vector2::new(delta_velocity, 0.0);
            }

            if input.actions.accel_left {
                let delta_velocity = player.lateral_acceleration * input.frame_time;
                physical.vel += physical.orientation * Vector2::new(-delta_velocity, 0.0);
            }

            if input.actions.turn_right {
                let delta_angle = UnitComplex::new(-player.angular_acceleration * input.frame_time);
                physical.orientation = physical.orientation * delta_angle;
            }

            if input.actions.turn_left {
                let delta_angle = UnitComplex::new(player.angular_acceleration * input.frame_time);
                physical.orientation = physical.orientation * delta_angle;
            }
        }
    }
}
