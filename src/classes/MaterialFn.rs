use super::vec3::Vec3;

pub struct MaterialFn {}

impl MaterialFn {
	pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
		v - &(2.0 * Vec3::dot_product(&v, &n) * n)
	}

	pub fn refract(mut v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
		v.normalize();

		let dt = Vec3::dot_product(&v, &n);
		let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

		if discriminant > 0.0 {
			*refracted = ni_over_nt * (v - &(n * dt)) - &(n * discriminant.sqrt());
			return true
		}

		false
	}

	pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
	}
}

