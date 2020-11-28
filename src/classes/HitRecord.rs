use super::lambertian::Lambertian;
use super::material::Material;
use super::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		HitRecord { t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0), material: Box::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))) }
	}
}