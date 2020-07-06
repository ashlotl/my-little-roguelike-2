use amethyst::{
	ecs::{
		Component,
		DenseVecStorage,
	},
};

use crate::{
	entities::{
		actor_variants::{
			echo::{
				Echo,
			},
			enemy::{
				Enemy,
			},
			player::{
				Player,
			},
		},
	},
	math_utils::{
		Vector,
	},
};

pub struct Actor {
	pub sub:ActorVariant,
	pub vel:Velocity,
}

impl Component for Actor {
	type Storage = DenseVecStorage<Self>;
}

pub struct Velocity {
	pub trans:Vector,
	pub rot:f32,
}

pub enum ActorVariant {
	PlayerActor(Player),
	EnemyActor(Enemy),
	EchoActor(Echo),
}
