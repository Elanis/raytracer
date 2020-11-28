use super::hitRecord::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub trait Material {
	fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
	fn box_clone(&self) -> Box<dyn Material>;
}