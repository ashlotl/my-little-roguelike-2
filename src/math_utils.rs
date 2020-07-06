pub struct Vector {
	pub x:f32,
	pub y:f32,
	pub z:f32,
}

impl Vector {
	pub fn magn(&self) -> f32 {
		self.dist(Vector{x:0.0,y:0.0,z:0.0})
	}

	pub fn dist(&self, other:Self)->f32 {
		((self.x-other.x).powi(2)+(self.y-other.y).powi(2)+(self.z-other.z).powi(2)).sqrt()
	}
}
