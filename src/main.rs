mod classes;

use std::io::Write;
use classes::vec3::Vec3;
use classes::ray::Ray;

use std::fs::File;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
	let oc = r.origin() - &center;

	let a = Vec3::dot_product(&r.direction(), &r.direction());
	let b = 2.0 * Vec3::dot_product(&oc, &r.direction());
	let c = Vec3::dot_product(&oc, &oc) - radius * radius;

	(b*b - 4.0*a*c) > 0.0
}

fn color_from_ray(r: &Ray) -> Vec3 {
	if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
		return Vec3::new(1.0,0.0,0.0)
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

	// File content
	for j in (0..ny).rev() {
		for i in 0..nx {
			let u = i as f32 / nx as f32;
			let v = j as f32 / ny as f32;

			let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
			let col = color_from_ray(&r);

			let ir = (255.0*col.x).round();
			let ig = (255.0*col.y).round();
			let ib = (255.0*col.z).round();

			file.write((ir.to_string() + &" ".to_owned() + &ig.to_string() + &" ".to_owned() + &ib.to_string() + &"\n".to_owned()).as_bytes())?;
		}
	}

	Ok(())
}
