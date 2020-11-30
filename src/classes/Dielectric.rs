use super::hitRecord::HitRecord;
use super::material::Material;
use super::materialFn::MaterialFn;
use super::ray::Ray;
use super::vec3::Vec3;

use rand::Rng;

#[derive(Clone)]
pub struct Dielectric {
	pub ref_idx : f32
}

impl Dielectric {
	pub fn new(ref_idx: f32) -> Dielectric {
		Dielectric { ref_idx }  
	}
}

impl Material for Dielectric {
	fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		let outward_normal;
		let reflected = MaterialFn::reflect(r.direction(), rec.normal);
		let ni_over_nt;
		*attenuation = Vec3::new(1.0, 1.0, 1.0);
		let mut refracted = Vec3::new(0.0, 0.0, 0.0);
		let reflect_prob;
		let cosine;
		let mut rng = rand::thread_rng();

		let dot_dir_norm = Vec3::dot_product(&r.direction(), &rec.normal);
		if dot_dir_norm > 0.0 {
			outward_normal = -rec.normal;
			ni_over_nt = self.ref_idx;
			cosine = self.ref_idx * dot_dir_norm / r.direction().length();
		} else {
			outward_normal = rec.normal;
			ni_over_nt = 1.0 / self.ref_idx;
			cosine = -dot_dir_norm / r.direction().length();
		}

		if MaterialFn::refract(r.direction(), outward_normal, ni_over_nt, &mut refracted)  {
			reflect_prob = MaterialFn::schlick(cosine, self.ref_idx);
		} else {
			*scattered = Ray::new(rec.p, reflected);
			reflect_prob = 1.0;
		}

		if rng.gen_range(0.0, 1.0) < reflect_prob {
			*scattered = Ray::new(rec.p, reflected);
		} else {
			*scattered = Ray::new(rec.p, refracted);
		}

		true
	}

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}