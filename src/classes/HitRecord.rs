use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		HitRecord { t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0) }
	}
}