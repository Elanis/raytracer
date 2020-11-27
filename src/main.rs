mod vec3;

use std::io::Write;
use vec3::Vec3;

use std::fs::File;

fn main() -> std::io::Result<()> {
	let nx = 200;
	let ny = 100;

	let mut file = File::create("test.ppm")?;

	file.write(b"P3\n")?;
	file.write((nx.to_string() + &" ".to_owned() + &ny.to_string()+ &"\n".to_owned()).as_bytes())?;
	file.write(b"255\n")?;

	for j in (0..ny).rev() {
		for i in 0..nx {
			let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2 as f32);

			let ir = (255.99*col.x).round();
			let ig = (255.99*col.y).round();
			let ib = (255.99*col.z).round();

			file.write((ir.to_string() + &" ".to_owned() + &ig.to_string() + &" ".to_owned() + &ib.to_string() + &"\n".to_owned()).as_bytes())?;
		}
	}

	Ok(())
}
