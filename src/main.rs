//have children
mod entities;
mod systems;

//lonely
mod math_utils;
mod resource_management;
mod states;


use amethyst::{
	Application,
	core::{
		transform::{
			TransformBundle,
		},
	},
	GameDataBuilder,
	input::{
		InputBundle,
		StringBindings,
	},
	renderer::{
		plugins::{
			RenderFlat2D,
			RenderToWindow,
		},
		types::{
			DefaultBackend,
		},
		RenderingBundle,
	},
	utils::{
		application_root_dir,
	},
};

use crate::{
	states::{
		GameplayState,
	},
	systems::{
		echo_avatar::{
			EchoAvatarSystem,
		},
		enemy::{
			EnemySystem,
		},
		level_generator::{
			LevelGeneratorSystem,
		},
		physics::{
			PhysicsSystem,
		},
		player_input::{
			PlayerInputSystem,
		},
		projectile::{
			ProjectileSystem,
		},
	},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());


	let app_root = application_root_dir()?;

	let app_assets = app_root.join("assets");
	let app_config = app_root.join("config");
	let app_config_display = app_config.join("display.ron");
	let app_config_binding = app_config.join("bindings.ron");


	let input_bundle = InputBundle::<StringBindings>::new()
		.with_bindings_from_file(app_config_binding)?;


	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(app_config_display)?
						.with_clear([0.0,0.0,0.5,1.0]),
				)
				.with_plugin(
					RenderFlat2D::default(),
				),
		)?
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
		.with(
			PhysicsSystem {
				scale:2.0,
			},
			"sys_physics",
			&[],
		)
		.with(
			PlayerInputSystem {
				scale:120.0,
				deadzone:0.05,
			},
			"sys_player_input",
			&["sys_physics"],
		)
		.with(
			EchoAvatarSystem {

			},
			"sys_echo_avatar",
			&["sys_player_input"],
		)
		.with(
			EnemySystem {

			},
			"sys_enemy",
			&["sys_echo_avatar"],
		)
		.with(
			ProjectileSystem {
				//nothing too complicated here, please. no heat seeking missles or nonsense like that
			},
			"sys_projectile",
			&["sys_enemy"],
		)
		.with(
			LevelGeneratorSystem {
				first:true,
				seed_1:420,
				seed_2:69,
			},
			"sys_level_generator",
			&["sys_projectile"],
		);

	let mut game = Application :: new(app_assets, GameplayState::new(), game_data)?;
	game.run();//yippee

	Ok(())//return that we're not a goose
}
