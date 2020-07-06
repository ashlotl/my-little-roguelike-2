use amethyst::{
	assets::{
		Handle,
	},
	ecs::{
		WorldExt,
	},
	GameData,
	renderer::{
		SpriteSheet,
	},
	SimpleState,
	StateData,
	StateEvent,
	Trans,
};

use crate::{
	entities::{
		actor::{
			Actor,
		},
		actor_variants::{
			player,
		},
	},
	resource_management,
	systems::{
		camera,
		level_generator,
	},
};

pub const GAME_WIDTH:f32 = 1200.0;
pub const GAME_HEIGHT:f32 = 600.0;

pub struct GameplayState {
	sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl GameplayState {
	pub fn new() -> Self {
		Self {
			sprite_sheet_handle: Option::None,
		}
	}
}

impl SimpleState for GameplayState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		self.sprite_sheet_handle.replace(resource_management::make_sprite_sheet(data.world));

		data.world.register::<Actor>();

		player::init_player(data.world, self.sprite_sheet_handle.clone().unwrap());

		level_generator::init_level(data.world, self.sprite_sheet_handle.clone().unwrap());

		camera::init_camera(data.world);
	}
	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, StateEvent> {
		Trans::None
	}
}
