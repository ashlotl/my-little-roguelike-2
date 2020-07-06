use amethyst::{
	core::{
		SystemDesc,
		timing::{
			Time,
		},
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

#[derive(SystemDesc)]
pub struct ProjectileSystem {

}

impl<'s> System<'s> for ProjectileSystem {
	type SystemData = (
		// WriteStorage<'s, Actor>,
		Read<'s, Time>,
	);

	fn run(&mut self, (time):Self::SystemData) {

	}
}
