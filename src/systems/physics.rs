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

const VELOCITY_CEASING_THRESHOLD:f32 = 0.3;
const FRICTION_COEFFICIENT:f32=0.95;

#[derive(SystemDesc)]
pub struct PhysicsSystem {
	pub scale:f32,
}

impl<'s> System<'s> for PhysicsSystem {
	type SystemData = (
		WriteStorage<'s,Actor>,
		// ReadStorage<'s, Tile>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut actors, mut transform, time):Self::SystemData) {
		let delta=time.delta_seconds()*self.scale;
		for (actor, transform) in (&mut actors, &mut transform).join() {
			if actor.vel.trans.magn()<VELOCITY_CEASING_THRESHOLD {
				actor.vel.trans.x=0.0;
				actor.vel.trans.y=0.0;
			}
			if transform.translation().z<=0.0 {
				actor.vel.trans.z=0.0;
				transform.set_translation_z(0.0);
				actor.vel.trans.x-=FRICTION_COEFFICIENT*actor.vel.trans.x*delta;
				actor.vel.trans.y-=FRICTION_COEFFICIENT*actor.vel.trans.y*delta;
			}

			transform.prepend_translation({
				let mut t = Transform::default();
				t.set_translation_xyz(actor.vel.trans.x, actor.vel.trans.y, actor.vel.trans.z);
				t.translation().clone()
			});
		}
	}
}
