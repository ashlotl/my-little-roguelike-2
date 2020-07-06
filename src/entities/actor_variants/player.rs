use amethyst::{
	assets::{
		Handle,
	},
	core::{
		Transform,
	},
	ecs::{
		world::{
			Builder,
		},
		World,
		WorldExt,
	},
	renderer::{
		SpriteRender,
		SpriteSheet,
	},
};

use crate::{
	entities::{
		actor::{
			Actor,
			ActorVariant,
			Velocity,
		},
	},
	math_utils::{
		Vector,
	},
};

pub struct Player {

}

pub fn init_player(world:&mut World, sprite_handle:Handle<SpriteSheet>) {
	let local_transform = Transform::default();

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_handle,
		sprite_number: 0,
	};

	world.create_entity()
		.with(sprite_render)
		.with(Actor {
			sub:ActorVariant::PlayerActor(
				Player {

				},
			),
			vel: Velocity {
				trans: Vector {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				},
				rot: 0.0,
			},
		})
		.with(local_transform)
		.build();
}
