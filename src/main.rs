use amethyst::{
	utils::{
		application_root_dir,
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
		.with_bindings_from_file(app_config_binding);


	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(app_config_display)?
						.with_clear([0.0,0.0,0.0,1.0]),
				)
				.with_plugin(
					RenderFlat2D::default(),
				),
		)?
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
		.with(
			PhysicsSystem {

			},
			"sys_physics",
			&[],
		)
		.with(
			PlayerInputSystem {
				scale:1.0,
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
				//ohboyohboyohboy
			},
			"sys_level_generator",
			&["sys_projectile"],
		);

	let mut game = Application :: new(app_assets, GameplayState::new(), game_data)?;
	game.run();//yippee

	Ok(())//return that we're not a goose
}
