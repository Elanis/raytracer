use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
	origin: Vec3,
	lower_left_corner: Vec3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
		let theta = vfov.to_radians();
		let half_height = (theta / 2.0).tan();
		let half_width = aspect * half_height;

		let mut w = lookfrom - &lookat;
		w.normalize();

		let mut u = Vec3::cross_product(&vup, &w);
		u.normalize();

		let v = Vec3::cross_product(&w, &u);

		Camera {
			lower_left_corner: lookfrom - &(half_width * u) - &(half_height * v) - &w,
			horizontal       : 2.0 * half_width * u,
			vertical         : 2.0 * half_height * v,
			origin           : lookfrom,
		}
	}

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - &self.origin)
    }
}