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
		world::{
			Builder,
		},
		World,
		WorldExt,
		WriteStorage,
	},
	renderer::{
		Camera,
	},
	shred::{
		System,
		SystemData,
	},
};

use crate::{
	states::{
		GAME_WIDTH,
		GAME_HEIGHT,
	},
};

pub fn init_camera(world:&mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 130.0);

	world
		.create_entity()
		.with(Camera::standard_3d(GAME_WIDTH, GAME_HEIGHT))
		.with(transform)
		.build();
}


#[derive(SystemDesc)]
pub struct CameraSystem {
	//some enum(s) for shake, follow mode
}

impl<'s> System<'s> for CameraSystem {
	type SystemData = (
		// ReadStorage<'s,Actor>,
		// ReadStorage<'s, Tile>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
	);

	fn run(&mut self, (transforms, time):Self::SystemData) {

	}
}
