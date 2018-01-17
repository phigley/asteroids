use specs::{Fetch, Join, NullStorage, ReadStorage, System, WriteStorage};

use cgmath::Vector2;

use input::{Action, Input};
use physics::Physical;

#[derive(Component, Debug, Default)]
#[component(NullStorage)]
pub struct Player;

pub struct PlayerController;

impl<'a> System<'a> for PlayerController {
    type SystemData = (
        Fetch<'a, Input>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, player, mut physical) = data;

        for (_, ref mut physical) in (&player, &mut physical).join() {
            for action in &input.actions {
                match *action {
                    Action::Forward => physical.vel += Vector2::new(0.0, 0.1),
                    Action::Left => physical.vel += Vector2::new(-0.1, 0.1),
                    Action::Right => physical.vel += Vector2::new(0.1, 0.0),
                }
            }
        }
    }
}
