use amethyst::{
	core::{
		SystemDesc,
		timing::{
			Time,
		},
		Transform,
	},
	derive::{
		SystemDesc,
	},
	ecs::{
		Join,
		Read,
		ReadStorage,
		WriteStorage,
	},
	input::{
		InputHandler,
		StringBindings,
	},
	shred::{
		System,
		SystemData,
	},
};

use crate::{
	entities::{
		actor::{
			Actor,
		},
	},
};

#[derive(SystemDesc)]
pub struct PlayerInputSystem {
	pub scale:f32,
	pub deadzone:f32,
}

impl<'s> System<'s> for PlayerInputSystem {
	type SystemData = (
		WriteStorage<'s, Actor>,
		WriteStorage<'s, Transform>,
		Read<'s, InputHandler<StringBindings>>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut actors, mut transforms, input, time):Self::SystemData) {
		let delta=time.delta_seconds()*self.scale;
		for (actor,transform) in (&mut actors, &mut transforms).join() {
			if let(player) = &actor.sub {
				let x_raw = input.axis_value("right").unwrap();
				let y_raw = input.axis_value("up").unwrap();
				if x_raw.abs()>self.deadzone {
					actor.vel.trans.x=x_raw;
				}
				if y_raw.abs()>self.deadzone {
					actor.vel.trans.y=y_raw;
				}
			}
		}
	}
}
