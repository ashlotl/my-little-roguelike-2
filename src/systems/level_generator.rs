use amethyst::{
	assets::{
		Handle,
	},
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
		SpriteRender,
		SpriteSheet,
	},
	shred::{
		System,
		SystemData,
	},
};


use random::Source;


use crate::{
	entities::{
		actor::{
			Actor,
		},
	},
};

pub fn init_level(world:&mut World, sprite_handle:Handle<SpriteSheet>) {
	let local_transform = Transform::default();

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_handle,
		sprite_number: 1,
	};

	world.create_entity()
		.with(sprite_render)
		.with(local_transform)
		.build();
}

#[derive(SystemDesc)]
pub struct LevelGeneratorSystem {
	pub first:bool,

	pub seed_1:u64,
	pub seed_2:u64,
}

impl<'s> System<'s> for LevelGeneratorSystem {
	type SystemData = (
		WriteStorage<'s, Actor>,
		Read<'s, Time>,
	);

	fn run(&mut self, (actors, time):Self::SystemData) {

		if self.first {
			self.first=false;
			let mut source = random::default().seed([self.seed_1, self.seed_2]);
			println!("Scalar:   {:?}", source.read::<f64>());
			println!("Scalar 2: {:?}", source.read::<f64>());
		}
		// println!("Vector: {:?}", source.iter().take(2).collect::<Vec<f64>>());
	}
}
