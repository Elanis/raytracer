use super::hitRecord::HitRecord;
use super::material::Material;
use super::materialFn::MaterialFn;
use super::ray::Ray;
use super::sphere::Sphere;
use super::vec3::Vec3;

#[derive(Clone)]
pub struct Metal {
	pub albedo: Vec3,
	pub fuzz: f32,
}

impl Metal {
	pub fn new(albedo: Vec3, mut fuzz: f32) -> Metal {
		if fuzz > 1.0 { fuzz = 1.0; }
		Metal { albedo, fuzz }  
	}
}

impl Material for Metal {
	fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		let mut rdirnorm = Vec3::new(r.direction().x, r.direction().y, r.direction().z);
		rdirnorm.normalize();

		let reflected = MaterialFn::reflect(rdirnorm, rec.normal);

		let random = Sphere::random_in_unit_sphere();

		*scattered = Ray::new(rec.p, reflected + Vec3::new(self.fuzz * random.x, self.fuzz * random.y, self.fuzz * random.z));
		*attenuation = self.albedo;

		Vec3::dot_product(&scattered.direction(), &rec.normal) > 0.0
	}

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}