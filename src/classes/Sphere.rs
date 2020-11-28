use super::hitRecord::HitRecord;
use super::hitable::Hitable;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

use rand::Rng;

pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
	pub material: Box<dyn Material>,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
		Sphere { center, radius, material }
	}


	pub fn random_in_unit_sphere() -> Vec3 {
		let mut p;
		let mut rng = rand::thread_rng();

		loop {
			p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));

			if p.squared_length() < 1.0 {
				break;
			}
		}

		p
	}
}

impl Hitable for Sphere {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
		let oc = r.origin() - &self.center;

		let a = Vec3::dot_product(&r.direction(), &r.direction());
		let b = Vec3::dot_product(&oc, &r.direction());
		let c = Vec3::dot_product(&oc, &oc) - self.radius * self.radius;

		let discriminant = b*b - a*c;

		if discriminant > 0.0 {
			let mut temp = (-b - discriminant.sqrt()) / a;

			if temp < t_max && temp > t_min  {
				rec.t = temp;
				rec.p = r.point_at_parameter(rec.t);
				rec.normal = (rec.p - &self.center) * (1.0 / self.radius);
				rec.material = self.material.box_clone();
				return true;
			}

			temp = (-b + discriminant.sqrt()) / a;

			if temp < t_max && temp > t_min {
				rec.t = temp;
				rec.p = r.point_at_parameter(rec.t);
				rec.normal = (rec.p - &self.center) * (1.0 / self.radius);
				rec.material = self.material.box_clone();
				return true;
			}
		}

		return false;
	}
}