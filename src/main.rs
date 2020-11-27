mod vec3;

use vec3::Vec3;

fn main() {
	let nx = 200;
	let ny = 100;

	println!("P3");
	println!("{}", nx.to_string() + &" ".to_owned() + &ny.to_string());
	println!("255");

	for j in (0..ny).rev() {
		for i in 0..nx {
			let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2 as f32);

			let ir = (255.99*col.x).round();
			let ig = (255.99*col.y).round();
			let ib = (255.99*col.z).round();

			println!("{}", ir.to_string() + &" ".to_owned() + &ig.to_string() + &" ".to_owned() + &ib.to_string());
		}
	}
}
