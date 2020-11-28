use super::hitRecord::HitRecord;
use super::hitable::Hitable;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f32) -> Sphere {
		Sphere { center, radius }
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
				return true;
			}

			temp = (-b + discriminant.sqrt()) / a;

			if temp < t_max && temp > t_min {
				rec.t = temp;
				rec.p = r.point_at_parameter(rec.t);
				rec.normal = (rec.p - &self.center) * (1.0 / self.radius);
				return true;
			}
		}

		return false;
	}
}