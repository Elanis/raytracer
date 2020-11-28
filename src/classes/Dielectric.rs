use super::hitRecord::HitRecord;
use super::material::Material;
use super::materialFn::MaterialFn;
use super::ray::Ray;
use super::vec3::Vec3;

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

		if Vec3::dot_product(&r.direction(), &rec.normal) > 0.0 {
			outward_normal = -rec.normal;
			ni_over_nt = self.ref_idx;
		} else {
			outward_normal = rec.normal;
			ni_over_nt = 1.0 / self.ref_idx;
		}

		if MaterialFn::refract(r.direction(), outward_normal, ni_over_nt, &mut refracted)  {
			*scattered = Ray::new(rec.p, refracted);
		} else {
			*scattered = Ray::new(rec.p, reflected);
			return false;
		}

		true
	}

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}