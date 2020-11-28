mod classes;

use std::io::Write;

use classes::hitable::Hitable;
use classes::hitableList::HitableList;
use classes::hitRecord::HitRecord;
use classes::ray::Ray;
use classes::sphere::Sphere;
use classes::vec3::Vec3;

use std::fs::File;

fn color_from_ray(r: &Ray, world: Box<&dyn Hitable>) -> Vec3 {
	let mut rec = HitRecord::new();
	if world.hit(r, 0.0, std::f32::MAX, &mut rec) {
		return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
	}

	let unit_direction = r.direction();
	let t = 0.5 * (unit_direction.y + 1.0);

	(1.0 -t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
	let nx = 200;
	let ny = 100;

	// File header
	let mut file = File::create("test.ppm")?;
	file.write(b"P3\n")?;
	file.write((nx.to_string() + &" ".to_owned() + &ny.to_string()+ &"\n".to_owned()).as_bytes())?;
	file.write(b"255\n")?;

	// Preparate variables
	let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
	let horizontal        = Vec3::new( 4.0,  0.0,  0.0);
	let vertical          = Vec3::new( 0.0,  2.0,  0.0);
	let origin            = Vec3::new( 0.0,  0.0,  0.0);

	let mut word = HitableList::new();
	word.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
	word.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

	// File content
	for j in (0..ny).rev() {
		for i in 0..nx {
			let u = i as f32 / nx as f32;
			let v = j as f32 / ny as f32;

			let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
			let col = color_from_ray(&r, Box::new(&word));

			let ir = (255.0*col.x).round();
			let ig = (255.0*col.y).round();
			let ib = (255.0*col.z).round();

			file.write((ir.to_string() + &" ".to_owned() + &ig.to_string() + &" ".to_owned() + &ib.to_string() + &"\n".to_owned()).as_bytes())?;
		}
	}

	Ok(())
}
