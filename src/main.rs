mod classes;

use std::io::Write;

use classes::camera::Camera;
use classes::dielectric::Dielectric;
use classes::hitable::Hitable;
use classes::hitableList::HitableList;
use classes::hitRecord::HitRecord;
use classes::lambertian::Lambertian;
use classes::metal::Metal;
use classes::ray::Ray;
use classes::sphere::Sphere;
use classes::vec3::Vec3;

use std::fs::File;

use rand::Rng;

fn color_from_ray(r: &Ray, world: Box<&dyn Hitable>, depth: u32) -> Vec3 {
	let mut rec = HitRecord::new();
	if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
		let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
		let mut attenuation = Vec3::new(0.0, 0.0, 0.0);

		let material = &rec.material;

		if depth < 50 && material.scatter(r, &rec, &mut attenuation, &mut scattered) {
			let color = color_from_ray(&scattered, world, depth + 1);

			return Vec3::new(color.x * attenuation.x, color.y * attenuation.y, color.z * attenuation.z);
		}

		return Vec3::new(0.0, 0.0, 0.0);
	}

	let mut unit_direction = Vec3::new(r.direction().x, r.direction().y, r.direction().z);
	unit_direction.normalize();
	let t = 0.5 * (unit_direction.y + 1.0);

	(1.0 -t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
	let nx = 200;
	let ny = 100;
	let ns = 100;

	// File header
	let mut file = File::create("test.ppm")?;
	file.write(b"P3\n")?;
	file.write((nx.to_string() + &" ".to_owned() + &ny.to_string()+ &"\n".to_owned()).as_bytes())?;
	file.write(b"255\n")?;

	// Preparate variables
	let mut rng = rand::thread_rng();
	let camera = Camera::new(
		Vec3::new(-2.0, 2.0, 1.0),
		Vec3::new(0.0, 0.0, -1.0),
		Vec3::new(0.0, -1.0, 0.0),
		90.0,
		nx as f32 / ny as f32
	);

	let mut word = HitableList::new();
	word.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))));
	word.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))));
	word.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)))));
	word.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5)))));
	word.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5)))));

	// File content
	for j in (0..ny).rev() {
		for i in 0..nx {
			let mut color = Vec3::new(0.0, 0.0, 0.0);

			for _s in 0..ns {
				let u = (i as f32 + rng.gen_range(0.0, 1.0))  / nx as f32;
				let v = (j as f32 + rng.gen_range(0.0, 1.0))  / ny as f32;

				let r = camera.get_ray(u, v);
				let col = color_from_ray(&r, Box::new(&word), 0);

				color = color + col;
			}

			let ir = (255.0*(color.x/ns as f32).sqrt()).round();
			let ig = (255.0*(color.y/ns as f32).sqrt()).round();
			let ib = (255.0*(color.z/ns as f32).sqrt()).round();

			file.write((ir.to_string() + &" ".to_owned() + &ig.to_string() + &" ".to_owned() + &ib.to_string() + &"\n".to_owned()).as_bytes())?;
		}
	}

	Ok(())
}