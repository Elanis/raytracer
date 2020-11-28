use super::hitRecord::HitRecord;
use super::ray::Ray;

pub trait Hitable {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}