use super::hitRecord::HitRecord;
use super::material::Material;
use super::ray::Ray;
use super::sphere::Sphere;
use super::vec3::Vec3;

#[derive(Clone)]
pub struct Lambertian {
	pub albedo: Vec3,
}

impl Lambertian {
	pub fn new(albedo: Vec3) -> Lambertian {
		Lambertian { albedo }  
	}
}

impl Material for Lambertian {
	fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		let target = rec.p + rec.normal + Sphere::random_in_unit_sphere();

		*scattered = Ray::new(rec.p, target - &rec.p);
		*attenuation = self.albedo;

		true
	}

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}